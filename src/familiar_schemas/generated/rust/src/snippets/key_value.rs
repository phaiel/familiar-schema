//! KeyValue - A flexible but structured key-value map for attaching arbitrary, namespaced metadata to an entity or component.
//! 
//! Generated from: KeyValue.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A flexible but structured key-value map for attaching arbitrary, namespaced metadata to an entity or component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValue {
    // Empty struct - no properties defined in schema
}

impl KeyValue {
    /// Create a new KeyValue with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for KeyValue {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_creation() {
        let instance = KeyValue::new();
        assert_eq!(instance, KeyValue::default());
    }

    #[test]
    fn test_key_value_serialization() {
        let instance = KeyValue::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: KeyValue = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}