/// Manifest manager that handles caching, fetching, and refreshing
/// the MCP manifest from the server.
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, interval};

use crate::domains::config::Config;
use crate::messages;

use super::types::McpManifest;
use super::{ManifestError, cache, fetch, inject, sha256};

#[derive(Debug)]
pub struct ManifestManager {
    manifest: Arc<RwLock<McpManifest>>,
    url: String,
    api_key: String,
    config_dir: PathBuf,
}

impl ManifestManager {
    /// Create new manager, loading from cache or fetching from server.
    /// If refresh=true, starts background refresh task.
    pub async fn new(url: String, api_key: String, refresh: bool) -> Result<Self, ManifestError> {
        // Get config directory
        let config_dir = dirs::config_dir()
            .ok_or_else(|| ManifestError::Config("Could not find config directory".to_string()))?
            .join("edge");

        // Ensure directory exists
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir)
                .map_err(|e| ManifestError::Config(format!("Failed to create config dir: {}", e)))?;
        }

        // Try to load from cache
        let (mut manifest, _last_fetched) = if let Some((cached, fetched_at)) = cache::load_cached_manifest(&config_dir)
        {
            // Check if fresh (within 10 minutes)
            if cache::is_fresh(fetched_at, cache::DEFAULT_TTL_MINUTES) {
                // eprintln!(
                //     "[edge] Using cached manifest (fetched {} minutes ago)",
                //     (Utc::now() - fetched_at).num_minutes()
                // );
                (cached, Some(fetched_at))
            } else {
                (cached, Some(fetched_at)) // Stale but we'll use as fallback
            }
        } else {
            // No cache, must fetch
            let fetched = fetch::fetch_manifest(&url, &api_key).await?;
            cache::save_manifest(&config_dir, &fetched)?;

            // Update config timestamp (load from default config path)
            if let Ok(mut config) = Config::load(None) {
                let _ = config.update_manifest_timestamp();
            }

            messages::success::manifest_cached();
            (fetched, Some(Utc::now()))
        };

        // Inject local actions and resources
        inject::inject_local_agent_actions(&mut manifest);
        inject::inject_local_resources(&mut manifest);

        let manifest = Arc::new(RwLock::new(manifest));

        let manager = Self {
            manifest: manifest.clone(),
            url,
            api_key,
            config_dir,
        };

        // Start background refresh if requested
        if refresh {
            manager.start_refresh_task();
        }

        Ok(manager)
    }

    /// Get shared access to the manifest.
    pub fn manifest(&self) -> Arc<RwLock<McpManifest>> {
        self.manifest.clone()
    }

    /// Manually trigger a refresh.
    pub async fn refresh(&self) -> Result<(), ManifestError> {
        let new_manifest = fetch::fetch_manifest(&self.url, &self.api_key).await?;
        cache::save_manifest(&self.config_dir, &new_manifest)?;

        // Load from default config path to update timestamp
        if let Ok(mut config) = Config::load(None) {
            let _ = config.update_manifest_timestamp();
        }

        let mut manifest = self.manifest.write().await;
        *manifest = new_manifest;
        inject::inject_local_agent_actions(&mut manifest);
        inject::inject_local_resources(&mut manifest);

        messages::success::manifest_refreshed();
        Ok(())
    }

    /// Start background refresh task (60 second interval).
    fn start_refresh_task(&self) {
        let manifest = self.manifest.clone();
        let url = self.url.clone();
        let api_key = self.api_key.clone();
        let config_dir = self.config_dir.clone();

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(60));

            loop {
                interval.tick().await;

                match fetch::fetch_manifest_raw(&url, &api_key).await {
                    Ok(body) => {
                        // Compare hash to see if changed
                        let current_hash = {
                            let guard = manifest.read().await;
                            sha256(&serde_json::to_vec(&*guard).unwrap_or_default())
                        };
                        let new_hash = sha256(&body);

                        if new_hash != current_hash {
                            match serde_json::from_slice::<McpManifest>(&body) {
                                Ok(mut new_manifest) => {
                                    inject::inject_local_agent_actions(&mut new_manifest);
                                    inject::inject_local_resources(&mut new_manifest);

                                    if let Err(e) = cache::save_manifest(&config_dir, &new_manifest) {
                                        messages::error::manifest_save_error(&e.to_string());
                                    }

                                    // Load from default config path to update timestamp
                                    if let Ok(mut config) = Config::load(None) {
                                        let _ = config.update_manifest_timestamp();
                                    }

                                    *manifest.write().await = new_manifest;
                                    messages::success::manifest_reloaded();
                                }
                                Err(e) => messages::error::manifest_parse_error(&e.to_string()),
                            }
                        }
                    }
                    Err(_e) => {
                        // eprintln!("[edge] Background refresh failed: {} — serving cached", e);
                    }
                }
            }
        });
    }
}
