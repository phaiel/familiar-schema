//! WeaveUnitPayload - A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.
//! 
//! Generated from: WeaveUnitPayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaveUnitPayload {
    pub user_context: serde_json::Value,
    pub data: serde_json::Value,
}

impl WeaveUnitPayload {
    /// Create a new WeaveUnitPayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for WeaveUnitPayload {
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
    fn test_weave_unit_payload_creation() {
        let instance = WeaveUnitPayload::new();
        assert_eq!(instance, WeaveUnitPayload::default());
    }

    #[test]
    fn test_weave_unit_payload_serialization() {
        let instance = WeaveUnitPayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: WeaveUnitPayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}