//! AnyValue - Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.
//! 
//! Generated from: AnyValue.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnyValue {
    // Empty struct - no properties defined in schema
}

impl AnyValue {
    /// Create a new AnyValue with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for AnyValue {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_any_value_creation() {
        let instance = AnyValue::new();
        assert_eq!(instance, AnyValue::default());
    }

    #[test]
    fn test_any_value_serialization() {
        let instance = AnyValue::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: AnyValue = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}