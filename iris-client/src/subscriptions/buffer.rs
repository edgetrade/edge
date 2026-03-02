use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::Mutex;

const MAX_BUFFER_SIZE: usize = 1000;

#[derive(Clone)]
pub struct SubscriptionManager {
    buffers: Arc<Mutex<HashMap<String, VecDeque<Value>>>>,
}

impl SubscriptionManager {
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_subscription(&self, subscription_id: String) {
        let mut buffers = self.buffers.lock().await;
        buffers.insert(subscription_id, VecDeque::new());
    }

    pub async fn push_event(&self, subscription_id: &str, event: Value) {
        let mut buffers = self.buffers.lock().await;
        if let Some(buffer) = buffers.get_mut(subscription_id) {
            if buffer.len() >= MAX_BUFFER_SIZE {
                buffer.pop_front();
            }
            buffer.push_back(event);
        }
    }

    pub async fn poll_events(&self, subscription_id: &str, limit: usize) -> Vec<Value> {
        let mut buffers = self.buffers.lock().await;
        if let Some(buffer) = buffers.get_mut(subscription_id) {
            let count = limit.min(buffer.len());
            buffer.drain(..count).collect()
        } else {
            Vec::new()
        }
    }

    pub async fn remove_subscription(&self, subscription_id: &str) {
        let mut buffers = self.buffers.lock().await;
        buffers.remove(subscription_id);
    }
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        Self::new()
    }
}
