//! TemporalAnchor - Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Block Universe.
//! 
//! Generated from: TemporalAnchor.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Block Universe.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TemporalAnchor {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl TemporalAnchor {
    /// Create a new TemporalAnchor with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TemporalAnchor {
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
    fn test_temporal_anchor_creation() {
        let instance = TemporalAnchor::new();
        assert_eq!(instance, TemporalAnchor::default());
    }

    #[test]
    fn test_temporal_anchor_serialization() {
        let instance = TemporalAnchor::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TemporalAnchor = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}