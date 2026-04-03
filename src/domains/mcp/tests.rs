//! MCP domain integration tests
//!
//! Tests the MCP server lifecycle including:
//! - Server start/stop
//! - EdgeServer creation and serving
//! - Tool/resource/prompt handling
//! - Local action handlers
//! - Subscription handlers

#[cfg(test)]
mod integration_tests {
    use crate::domains::alerts::AlertsHandle;
    use crate::domains::client::ClientHandle;
    use crate::domains::enclave::EnclaveHandle;
    use crate::domains::mcp::{EdgeServer, McpHandle, McpMode, TransportType};
    use crate::domains::trades::TradesHandle;
    use crate::event_bus::EventBus;

    /// Test that EdgeServer can be created
    #[tokio::test]
    #[ignore = "Requires connected client with manifest"]
    async fn test_edgeserver_creation() {
        let event_bus = EventBus::new(128);

        // Create dependencies
        let client = ClientHandle::new(event_bus.clone());
        let enclave = EnclaveHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let trades = TradesHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let alerts = AlertsHandle::default();

        // Create MCP handle with dependencies
        let (mcp, _actor) = McpHandle::new(client, enclave, trades, alerts, event_bus.clone());

        // Verify handle was created
        let status = mcp.get_status().await;
        assert!(status.is_ok());

        let state = status.unwrap();
        assert!(matches!(state.mode, McpMode::Stopped));
    }

    /// Test MCP server lifecycle (start/stop)
    #[tokio::test]
    #[ignore = "Requires connected client with manifest"]
    async fn test_mcp_server_lifecycle() {
        let event_bus = EventBus::new(128);

        // Create dependencies
        let client = ClientHandle::new(event_bus.clone());
        let enclave = EnclaveHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let trades = TradesHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let alerts = AlertsHandle::default();

        // Create MCP handle with dependencies
        let (mcp, _actor) = McpHandle::new(client, enclave, trades, alerts, event_bus.clone());

        // Verify initial state
        let status = mcp.get_status().await.unwrap();
        assert!(matches!(status.mode, McpMode::Stopped));

        // Note: Starting the server requires a connected client with manifest
        // This test would fail without proper setup, hence it's ignored
        // The actual start/stop test would look like:
        //
        // let result = mcp.start(TransportType::Stdio).await;
        // assert!(result.is_ok());
        //
        // let status = mcp.get_status().await.unwrap();
        // assert!(matches!(status.mode, McpMode::Running { .. }));
        //
        // let result = mcp.stop().await;
        // assert!(result.is_ok());
        //
        // let status = mcp.get_status().await.unwrap();
        // assert!(matches!(status.mode, McpMode::Stopped));
    }

    /// Test that stopping a non-running server fails
    #[tokio::test]
    async fn test_mcp_server_not_running() {
        let event_bus = EventBus::new(128);

        // Create dependencies
        let client = ClientHandle::new(event_bus.clone());
        let enclave = EnclaveHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let trades = TradesHandle::new(tokio::sync::mpsc::channel(64).1, event_bus.clone())
            .await
            .unwrap();
        let alerts = AlertsHandle::default();

        let (mcp, _actor) = McpHandle::new(client, enclave, trades, alerts, event_bus.clone());

        // Try to stop without starting - should fail
        let result = mcp.stop().await;
        assert!(result.is_err());
        assert!(matches!(result, Err(crate::domains::mcp::McpError::NotRunning)));
    }

    /// Test TransportType serialization/deserialization
    #[test]
    fn test_transport_type_serde() {
        let stdio = TransportType::Stdio;
        let json = serde_json::to_string(&stdio).unwrap();
        assert_eq!(json, r#""Stdio""#);

        let http = TransportType::Http {
            host: "127.0.0.1".to_string(),
            port: 8080,
        };
        let json = serde_json::to_string(&http).unwrap();
        assert!(json.contains("127.0.0.1"));
        assert!(json.contains("8080"));
    }

    /// Test McpMode variants
    #[test]
    fn test_mcp_mode_variants() {
        use crate::domains::mcp::McpMode;
        use tokio_util::sync::CancellationToken;

        let stopped = McpMode::Stopped;
        assert!(matches!(stopped, McpMode::Stopped));

        let token = CancellationToken::new();
        let running = McpMode::Running {
            shutdown: token.clone(),
        };
        assert!(matches!(running, McpMode::Running { .. }));
    }

    /// Test EdgeServer Clone (needed for HTTP server spawning)
    #[test]
    fn test_edgeserver_clone_compiles() {
        // EdgeServer derives Clone, verify it compiles
        fn assert_clone<T: Clone>() {}
        assert_clone::<EdgeServer>();
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::domains::mcp::server::{AlertDelivery, WebhookDispatcher, next_alert_id};

    #[test]
    fn test_next_alert_id() {
        let id1 = next_alert_id();
        let id2 = next_alert_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_alert_delivery_webhook() {
        let delivery = AlertDelivery::Webhook {
            url: "https://example.com/webhook".to_string(),
            secret: Some("secret".to_string()),
        };

        let json = serde_json::to_string(&delivery).unwrap();
        assert!(json.contains("webhook"));
        assert!(json.contains("https://example.com/webhook"));
    }

    #[test]
    fn test_alert_delivery_redis() {
        let delivery = AlertDelivery::Redis {
            url: "redis://localhost:6379".to_string(),
            channel: "alerts".to_string(),
        };

        let json = serde_json::to_string(&delivery).unwrap();
        assert!(json.contains("redis"));
        assert!(json.contains("alerts"));
    }

    #[test]
    fn test_alert_delivery_telegram() {
        let delivery = AlertDelivery::Telegram {
            bot_token: "token123".to_string(),
            chat_id: "chat456".to_string(),
        };

        let json = serde_json::to_string(&delivery).unwrap();
        assert!(json.contains("telegram"));
    }

    #[tokio::test]
    async fn test_webhook_dispatcher_new() {
        let dispatcher = WebhookDispatcher::new();
        // Just verify it creates
        drop(dispatcher);
    }

    #[tokio::test]
    async fn test_webhook_dispatcher_register_get() {
        let dispatcher = WebhookDispatcher::new();

        dispatcher
            .register("test_proc", "https://example.com/webhook", Some("secret"))
            .await;

        let config = dispatcher.get_webhook("test_proc").await;
        assert!(config.is_some());

        let (url, secret) = config.unwrap();
        assert_eq!(url, "https://example.com/webhook");
        assert_eq!(secret, Some("secret".to_string()));
    }

    #[tokio::test]
    async fn test_webhook_dispatcher_unregister() {
        let dispatcher = WebhookDispatcher::new();

        dispatcher
            .register("test_proc", "https://example.com/webhook", None)
            .await;
        assert!(dispatcher.get_webhook("test_proc").await.is_some());

        dispatcher.unregister("test_proc").await;
        assert!(dispatcher.get_webhook("test_proc").await.is_none());
    }
}
