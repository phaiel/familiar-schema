//! Bond - A classical entity representing a persistent relationship between two Thread entities.
//! 
//! Generated from: Bond.schema.json
//! Category: entities
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A classical entity representing a persistent relationship between two Thread entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    /// Component references temporarily disabled for pipeline testing
    pub components: serde_json::Value,
    /// A canonical enum of all 7 cognitive entity types.
    pub entity_type: String,
    /// A reusable definition for a unique entity identifier.
    pub thread_a_id: String,
    /// A reusable definition for a unique entity identifier.
    pub thread_b_id: String,
    pub physics_state: serde_json::Value,
}

impl Bond {
    /// Create a new Bond with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Bond {
    fn default() -> Self {
        Self {
            components: serde_json::Value::Null,
            entity_type: String::new(),
            thread_a_id: String::new(),
            thread_b_id: String::new(),
            physics_state: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_creation() {
        let instance = Bond::new();
        assert_eq!(instance, Bond::default());
    }

    #[test]
    fn test_bond_serialization() {
        let instance = Bond::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Bond = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}