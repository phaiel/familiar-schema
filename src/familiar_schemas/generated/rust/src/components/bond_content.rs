//! BondContent - Defines the descriptive content and history of a relationship Bond.
//! 
//! Generated from: BondContent.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the descriptive content and history of a relationship Bond.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BondContent {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl BondContent {
    /// Create a new BondContent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BondContent {
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
    fn test_bond_content_creation() {
        let instance = BondContent::new();
        assert_eq!(instance, BondContent::default());
    }

    #[test]
    fn test_bond_content_serialization() {
        let instance = BondContent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: BondContent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}