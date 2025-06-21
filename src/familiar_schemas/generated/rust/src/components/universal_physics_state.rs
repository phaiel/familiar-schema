//! UniversalPhysicsState - The transient, mutable physics state of an entity.
//! 
//! Generated from: UniversalPhysicsState.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The transient, mutable physics state of an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniversalPhysicsState {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl UniversalPhysicsState {
    /// Create a new UniversalPhysicsState with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for UniversalPhysicsState {
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
    fn test_universal_physics_state_creation() {
        let instance = UniversalPhysicsState::new();
        assert_eq!(instance, UniversalPhysicsState::default());
    }

    #[test]
    fn test_universal_physics_state_serialization() {
        let instance = UniversalPhysicsState::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: UniversalPhysicsState = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}