//! CrossTenantLink - Manages a link to a Thread in another tenant, enabling federated relationships.
//! 
//! Generated from: CrossTenantLink.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Manages a link to a Thread in another tenant, enabling federated relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrossTenantLink {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl CrossTenantLink {
    /// Create a new CrossTenantLink with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CrossTenantLink {
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
    fn test_cross_tenant_link_creation() {
        let instance = CrossTenantLink::new();
        assert_eq!(instance, CrossTenantLink::default());
    }

    #[test]
    fn test_cross_tenant_link_serialization() {
        let instance = CrossTenantLink::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: CrossTenantLink = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}