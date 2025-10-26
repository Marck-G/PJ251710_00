use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MQEvent<T> {
    pub event_type: String,
    pub payload: T,
    pub timestamp: DateTime<Utc>,
     #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Uuid>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl <T> MQEvent<T> {
    pub fn new(event_type: String, payload: T) -> Self {
        MQEvent {
            message_id: Some(Uuid::new_v4()),
            correlation_id: None,
            event_type,
            payload,
            timestamp: Utc::now(),
            metadata: None,
        }
    }
    pub fn with_correlation_id(mut self, correlation_id: Uuid) -> Self {
        self.correlation_id = Some(correlation_id);
        self
    }

    pub fn with_message_id(mut self, message_id: Uuid) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        if let Some(ref mut meta) = self.metadata {
            meta.insert(key, value);
        } else {
            let mut meta = HashMap::new();
            meta.insert(key, value);
            self.metadata = Some(meta);
        }
        self
    }
    pub fn get_metadata(&self, key: &str) -> Option<&String> {
        if let Some(ref meta) = self.metadata {
            meta.get(key)
        } else {
            None
        }
    }
    pub fn remove_metadata(mut self, key: &str) -> Self {
        if let Some(ref mut meta) = self.metadata {
            meta.remove(key);
        }
        self
    }

}