//! ShuttlePayload - The state object that carries a WeaveUnit through the Heddle reconciliation pipeline.
//! 
//! Generated from: ShuttlePayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The state object that carries a WeaveUnit through the Heddle reconciliation pipeline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShuttlePayload {
    pub user_context: serde_json::Value,
    pub data: serde_json::Value,
}

impl ShuttlePayload {
    /// Create a new ShuttlePayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ShuttlePayload {
    fn default() -> Self {
        Self {
            user_context: serde_json::Value::Null,
            data: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shuttle_payload_creation() {
        let instance = ShuttlePayload::new();
        assert_eq!(instance, ShuttlePayload::default());
    }

    #[test]
    fn test_shuttle_payload_serialization() {
        let instance = ShuttlePayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ShuttlePayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}