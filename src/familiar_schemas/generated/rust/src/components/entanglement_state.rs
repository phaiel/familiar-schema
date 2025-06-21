//! EntanglementState - Captures the subjective experience of a Moment by a specific Thread, linking them with quantum properties.
//! 
//! Generated from: EntanglementState.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Captures the subjective experience of a Moment by a specific Thread, linking them with quantum properties.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntanglementState {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl EntanglementState {
    /// Create a new EntanglementState with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EntanglementState {
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
    fn test_entanglement_state_creation() {
        let instance = EntanglementState::new();
        assert_eq!(instance, EntanglementState::default());
    }

    #[test]
    fn test_entanglement_state_serialization() {
        let instance = EntanglementState::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: EntanglementState = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}