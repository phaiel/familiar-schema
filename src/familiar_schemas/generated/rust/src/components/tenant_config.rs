//! TenantConfig - Stores tenant-specific settings and feature flag overrides.
//! 
//! Generated from: TenantConfig.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Stores tenant-specific settings and feature flag overrides.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TenantConfig {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl TenantConfig {
    /// Create a new TenantConfig with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TenantConfig {
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
    fn test_tenant_config_creation() {
        let instance = TenantConfig::new();
        assert_eq!(instance, TenantConfig::default());
    }

    #[test]
    fn test_tenant_config_serialization() {
        let instance = TenantConfig::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TenantConfig = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}