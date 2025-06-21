//! IntentContent - Defines the content of a specific task or intention.
//! 
//! Generated from: IntentContent.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the content of a specific task or intention.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntentContent {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl IntentContent {
    /// Create a new IntentContent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for IntentContent {
    fn default() -> Self {
        Self {
            physics_properties: serde_json::Value::Null,
            fields: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_content_creation() {
        let instance = IntentContent::new();
        assert_eq!(instance, IntentContent::default());
    }

    #[test]
    fn test_intent_content_serialization() {
        let instance = IntentContent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: IntentContent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}