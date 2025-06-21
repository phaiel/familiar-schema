//! Moment - A classical entity representing a specific, objective event in the past. This is the atomic unit of episodic memory.
//! 
//! Generated from: Moment.schema.json
//! Category: entities
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A classical entity representing a specific, objective event in the past. This is the atomic unit of episodic memory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Moment {
    /// Component references temporarily disabled for pipeline testing
    pub components: serde_json::Value,
    /// A canonical enum of all 7 cognitive entity types.
    pub entity_type: String,
    pub physics_state: serde_json::Value,
}

impl Moment {
    /// Create a new Moment with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Moment {
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
    fn test_moment_creation() {
        let instance = Moment::new();
        assert_eq!(instance, Moment::default());
    }

    #[test]
    fn test_moment_serialization() {
        let instance = Moment::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Moment = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}