//! TenantIdentity - Defines the core identity and metadata for a Tenant.
//! 
//! Generated from: TenantIdentity.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the core identity and metadata for a Tenant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenantIdentity {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl TenantIdentity {
    /// Create a new TenantIdentity with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TenantIdentity {
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
    fn test_tenant_identity_creation() {
        let instance = TenantIdentity::new();
        assert_eq!(instance, TenantIdentity::default());
    }

    #[test]
    fn test_tenant_identity_serialization() {
        let instance = TenantIdentity::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TenantIdentity = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}