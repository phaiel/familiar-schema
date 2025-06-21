//! WeavePayload - Represents a complete, raw text input from a user, ready for ingestion.
//! 
//! Generated from: WeavePayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents a complete, raw text input from a user, ready for ingestion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeavePayload {
    pub user_context: serde_json::Value,
    pub data: serde_json::Value,
}

impl WeavePayload {
    /// Create a new WeavePayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for WeavePayload {
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
    fn test_weave_payload_creation() {
        let instance = WeavePayload::new();
        assert_eq!(instance, WeavePayload::default());
    }

    #[test]
    fn test_weave_payload_serialization() {
        let instance = WeavePayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: WeavePayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}