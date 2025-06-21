//! Tenant - Canonical schema for a Tenant, the root container for all user data and configuration. This is a System Entity and does not participate in physics simulations.
//! 
//! Generated from: Tenant.schema.json
//! Category: entities
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Canonical schema for a Tenant, the root container for all user data and configuration. This is a System Entity and does not participate in physics simulations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tenant {
    /// A reusable definition for a unique entity identifier.
    pub entity_id: String,
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub tenant_id: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub created_at: String,
    /// The canonical type of the system entity.
    pub entity_type: String,
    /// Represents the state of a long-running, orchestrated workflow.
    pub workflow_state: Option<serde_json::Value>,
    pub components: serde_json::Value,
}

impl Tenant {
    /// Create a new Tenant with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Tenant {
    fn default() -> Self {
        Self {
            entity_id: String::new(),
            tenant_id: String::new(),
            created_at: String::new(),
            entity_type: String::new(),
            workflow_state: None,
            components: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_creation() {
        let instance = Tenant::new();
        assert_eq!(instance, Tenant::default());
    }

    #[test]
    fn test_tenant_serialization() {
        let instance = Tenant::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Tenant = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}