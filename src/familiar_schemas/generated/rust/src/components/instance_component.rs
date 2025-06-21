//! InstanceComponent - Captures the specific attributes of a generic object within the context of a single event (Entanglement).
//! 
//! Generated from: InstanceComponent.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Captures the specific attributes of a generic object within the context of a single event (Entanglement).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstanceComponent {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl InstanceComponent {
    /// Create a new InstanceComponent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for InstanceComponent {
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
    fn test_instance_component_creation() {
        let instance = InstanceComponent::new();
        assert_eq!(instance, InstanceComponent::default());
    }

    #[test]
    fn test_instance_component_serialization() {
        let instance = InstanceComponent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: InstanceComponent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}