//! Intent - A classical entity representing a specific, user-declared future action or task.
//! 
//! Generated from: Intent.schema.json
//! Category: entities
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A classical entity representing a specific, user-declared future action or task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Intent {
    /// Component references temporarily disabled for pipeline testing
    pub components: serde_json::Value,
    /// A canonical enum of all 7 cognitive entity types.
    pub entity_type: String,
    pub physics_state: serde_json::Value,
}

impl Intent {
    /// Create a new Intent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Intent {
    fn default() -> Self {
        Self {
            components: serde_json::Value::Null,
            entity_type: String::new(),
            physics_state: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_creation() {
        let instance = Intent::new();
        assert_eq!(instance, Intent::default());
    }

    #[test]
    fn test_intent_serialization() {
        let instance = Intent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Intent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}