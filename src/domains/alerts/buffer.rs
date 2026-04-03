//! Subscription buffer management
//!
//! MIGRATED FROM: pkg/poseidon/src/commands/subscribe/buffer.rs
//! Original implementation preserved with adaptations for actor pattern

use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Maximum number of events to buffer per subscription
const MAX_BUFFER_SIZE: usize = 1000;

/// Manages event buffers for subscription-based event streaming.
/// Events are stored in memory and can be polled by clients.
#[derive(Clone)]
pub struct SubscriptionManager {
    buffers: Arc<Mutex<HashMap<String, VecDeque<Value>>>>,
}

impl SubscriptionManager {
    /// Create a new subscription manager with empty buffers
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Create a new subscription buffer
    pub async fn create_subscription(&self, subscription_id: String) {
        let mut buffers = self.buffers.lock().await;
        buffers.insert(subscription_id, VecDeque::new());
    }

    /// Push an event to a subscription buffer
    /// If buffer exceeds MAX_BUFFER_SIZE, oldest events are dropped
    pub async fn push_event(&self, subscription_id: &str, event: Value) {
        let mut buffers = self.buffers.lock().await;
        if let Some(buffer) = buffers.get_mut(subscription_id) {
            if buffer.len() >= MAX_BUFFER_SIZE {
                buffer.pop_front();
            }
            buffer.push_back(event);
        }
    }

    /// Poll events from a subscription buffer
    /// Removes and returns up to `limit` events from the buffer
    pub async fn poll_events(&self, subscription_id: &str, limit: usize) -> Vec<Value> {
        let mut buffers = self.buffers.lock().await;
        if let Some(buffer) = buffers.get_mut(subscription_id) {
            let count = limit.min(buffer.len());
            buffer.drain(..count).collect()
        } else {
            Vec::new()
        }
    }

    /// Remove a subscription and its buffer
    pub async fn remove_subscription(&self, subscription_id: &str) {
        let mut buffers = self.buffers.lock().await;
        buffers.remove(subscription_id);
    }

    /// Get the current size of a subscription buffer
    pub async fn buffer_size(&self, subscription_id: &str) -> usize {
        let buffers = self.buffers.lock().await;
        buffers.get(subscription_id).map(|b| b.len()).unwrap_or(0)
    }

    /// Check if a subscription exists
    pub async fn has_subscription(&self, subscription_id: &str) -> bool {
        let buffers = self.buffers.lock().await;
        buffers.contains_key(subscription_id)
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}
