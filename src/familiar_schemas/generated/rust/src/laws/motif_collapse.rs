//! MotifCollapse - Performs a quantum measurement on a Motif entity, collapsing its superposition into a single classical state. This is a high-priority, user-responsive law.
//! 
//! Generated from: MotifCollapse.schema.json
//! Category: laws
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Performs a quantum measurement on a Motif entity, collapsing its superposition into a single classical state. This is a high-priority, user-responsive law.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MotifCollapse {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    pub execution_envelope: serde_json::Value,
    /// A list of component schema IDs that this law reads from or writes to.
    pub affected_components: Vec<String>,
}

impl MotifCollapse {
    /// Create a new MotifCollapse with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MotifCollapse {
    fn default() -> Self {
        Self {
            physics_properties: serde_json::Value::Null,
            execution_envelope: serde_json::Value::Null,
            affected_components: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motif_collapse_creation() {
        let instance = MotifCollapse::new();
        assert_eq!(instance, MotifCollapse::default());
    }

    #[test]
    fn test_motif_collapse_serialization() {
        let instance = MotifCollapse::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: MotifCollapse = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}