//! TenantMembers - Manages the users and their roles within a Tenant.
//! 
//! Generated from: TenantMembers.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Manages the users and their roles within a Tenant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenantMembers {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl TenantMembers {
    /// Create a new TenantMembers with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TenantMembers {
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
    fn test_tenant_members_creation() {
        let instance = TenantMembers::new();
        assert_eq!(instance, TenantMembers::default());
    }

    #[test]
    fn test_tenant_members_serialization() {
        let instance = TenantMembers::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TenantMembers = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}