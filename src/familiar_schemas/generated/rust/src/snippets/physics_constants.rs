//! PhysicsConstants - A structured definition for a set of physics constants.
//! 
//! Generated from: PhysicsConstants.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A structured definition for a set of physics constants.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicsConstants {
    pub decay_rate: Option<f64>,
    pub spring_constant: Option<f64>,
    pub max_tension: Option<f64>,
}

impl PhysicsConstants {
    /// Create a new PhysicsConstants with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        Self {
            decay_rate: None,
            spring_constant: None,
            max_tension: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physics_constants_creation() {
        let instance = PhysicsConstants::new();
        assert_eq!(instance, PhysicsConstants::default());
    }

    #[test]
    fn test_physics_constants_serialization() {
        let instance = PhysicsConstants::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: PhysicsConstants = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}