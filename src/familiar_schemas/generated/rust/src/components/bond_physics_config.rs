//! BondPhysicsConfig - Configuration that defines the physics model for a Bond (Dynamic or Static).
//! 
//! Generated from: BondPhysicsConfig.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Configuration that defines the physics model for a Bond (Dynamic or Static).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BondPhysicsConfig {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl BondPhysicsConfig {
    /// Create a new BondPhysicsConfig with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BondPhysicsConfig {
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
    fn test_bond_physics_config_creation() {
        let instance = BondPhysicsConfig::new();
        assert_eq!(instance, BondPhysicsConfig::default());
    }

    #[test]
    fn test_bond_physics_config_serialization() {
        let instance = BondPhysicsConfig::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: BondPhysicsConfig = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}