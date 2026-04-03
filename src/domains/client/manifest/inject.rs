use super::types::{ActionDef, McpManifest, ResourceDef};

/// Injects the locally-handled `register_alert` / `unregister_alert` actions into
/// the `agent` namespace tool so they appear in MCP `list_tools` responses.
/// Called both at startup and after each manifest refresh.
pub fn inject_local_agent_actions(manifest: &mut McpManifest) {
    let Some(agent_tool) = manifest.tools.iter_mut().find(|t| t.name == "agent") else {
        return;
    };
    if agent_tool.actions.iter().any(|a| a.name == "ping") {
        return;
    }

    agent_tool.actions.push(ActionDef {
        name: "list_alerts".to_string(),
        description:
            "List all currently active alert subscriptions with their IDs, event types, and delivery destinations."
                .to_string(),
        input_schema: serde_json::json!({ "type": "object" }),
        procedure: "agent.list_alerts".to_string(),
        kind: "local".to_string(),
    });

    agent_tool.actions.push(ActionDef {
        name: "register_alert".to_string(),
        description: "Register an alert subscription. Read edge://alerts first to discover available alert types and their input schemas. Delivers events via webhook, Redis stream, or Telegram.".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "alert_name": {
                    "type": "string",
                    "description": "Alert type name from edge://alerts (e.g. on_pair_updates)"
                },
                "input": {
                    "type": "object",
                    "description": "Filter parameters for the alert (see inputSchema in edge://alerts)"
                },
                "delivery": {
                    "oneOf": [
                        {
                            "type": "object",
                            "properties": {
                                "type": { "type": "string", "const": "webhook" },
                                "url": { "type": "string" },
                                "secret": { "type": "string" }
                            },
                            "required": ["type", "url"]
                        },
                        {
                            "type": "object",
                            "properties": {
                                "type": { "type": "string", "const": "redis" },
                                "url": { "type": "string" },
                                "channel": { "type": "string" }
                            },
                            "required": ["type", "url", "channel"]
                        },
                        {
                            "type": "object",
                            "properties": {
                                "type": { "type": "string", "const": "telegram" },
                                "bot_token": { "type": "string" },
                                "chat_id": { "type": "string" }
                            },
                            "required": ["type", "bot_token", "chat_id"]
                        }
                        ]
                    }
                },
                "required": ["alert_name", "input", "delivery"]
            }),
            procedure: "agent.register_alert".to_string(),
            kind: "local".to_string(),
        });

    agent_tool.actions.push(ActionDef {
        name: "unregister_alert".to_string(),
        description: "Stop and remove a previously registered alert subscription.".to_string(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "alert_id": { "type": "string", "description": "Alert ID returned by register_alert" }
            },
            "required": ["alert_id"]
        }),
        procedure: "agent.unregister_alert".to_string(),
        kind: "local".to_string(),
    });

    agent_tool.actions.push(ActionDef {
            name: "ping_subscription".to_string(),
            description: "Subscribe to a periodic ping emitted every 5 seconds from the server. Use to verify subscription connectivity. Call with _action=subscribe to start, _action=poll to drain buffered pings, _action=stop to cancel.".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "_action": {
                        "type": "string",
                        "enum": ["subscribe", "poll", "stop"],
                        "description": "subscribe (default): start subscription and return subscription_id; poll: drain buffered events; stop: cancel subscription"
                    },
                    "subscription_id": {
                        "type": "integer",
                        "description": "Required for poll and stop. Returned by subscribe."
                    },
                    "limit": {
                        "type": "integer",
                        "description": "Maximum events to return per poll (default 10)"
                    }
                }
            }),
            procedure: "alerts.onPing".to_string(),
            kind: "subscription".to_string(),
        });

    agent_tool.actions.push(ActionDef {
        name: "ping".to_string(),
        description: "Check connectivity with a request-response ping. Returns pong on success.".to_string(),
        input_schema: serde_json::json!({ "type": "object" }),
        procedure: "agent.ping".to_string(),
        kind: "local".to_string(),
    });

    if let Some(enum_arr) = agent_tool
        .input_schema
        .pointer_mut("/properties/action/enum")
        .and_then(|v| v.as_array_mut())
    {
        for action in [
            "ping",
            "ping_subscription",
            "list_alerts",
            "register_alert",
            "unregister_alert",
        ] {
            if !enum_arr.iter().any(|v| v.as_str() == Some(action)) {
                enum_arr.push(serde_json::Value::String(action.to_string()));
            }
        }
    }

    agent_tool
        .description
        .push_str("\n• ping: Check connectivity with a request-response ping. Returns pong on success.");
    agent_tool.description.push_str("\n• ping_subscription: Subscribe to a periodic ping (every 5s) to verify subscription connectivity. Call with _action=subscribe, then _action=poll to receive events, _action=stop to cancel.");
    agent_tool.description.push_str("\n• list_alerts: List all currently active alert subscriptions with their IDs, event types, and delivery destinations.");
    agent_tool.description.push_str("\n• register_alert: Register an alert subscription. Read edge://alerts for available alert types and edge://alert-delivery for exact delivery method schemas such as webhook, Redis stream, or Telegram.");
    agent_tool
        .description
        .push_str("\n• unregister_alert: Stop and remove a previously registered alert subscription.");
}

/// Injects locally-managed resources into the manifest so they appear in MCP
/// `list_resources` responses. Called both at startup and after each manifest refresh.
pub fn inject_local_resources(manifest: &mut McpManifest) {
    const URI: &str = "edge://alert-delivery";
    if manifest.resources.iter().any(|r| r.uri == URI) {
        return;
    }
    manifest.resources.push(ResourceDef {
        uri: URI.to_string(),
        name: "Alert Delivery Methods".to_string(),
        description: "Supported delivery targets for alert registrations: webhook, Redis stream, and Telegram. Each entry includes the required fields and their types.".to_string(),
        mime_type: "application/json".to_string(),
        content: serde_json::json!([
            {
                "type": "webhook",
                "description": "POST alert payloads to an HTTPS endpoint.",
                "required": ["type", "url"],
                "fields": {
                    "type":   { "type": "string", "const": "webhook" },
                    "url":    { "type": "string", "description": "HTTPS endpoint that will receive POST requests" },
                    "secret": { "type": "string", "description": "Optional HMAC secret used to sign each request for verification" }
                }
            },
            {
                "type": "redis",
                "description": "Push alert events onto a Redis stream.",
                "required": ["type", "url", "channel"],
                "fields": {
                    "type":    { "type": "string", "const": "redis" },
                    "url":     { "type": "string", "description": "Redis connection URL (e.g. redis://host:6379)" },
                    "channel": { "type": "string", "description": "Redis stream key or channel name to publish events to" }
                }
            },
            {
                "type": "telegram",
                "description": "Send alert notifications to a Telegram chat or group.",
                "required": ["type", "bot_token", "chat_id"],
                "fields": {
                    "type":      { "type": "string", "const": "telegram" },
                    "bot_token": { "type": "string", "description": "Telegram bot API token from @BotFather" },
                    "chat_id":   { "type": "string", "description": "Telegram chat or group ID to receive alert messages" }
                }
            }
        ]),
    });
}
