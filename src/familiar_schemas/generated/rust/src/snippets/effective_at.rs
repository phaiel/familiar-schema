//! EffectiveAt - A canonical definition for an ISO 8601 timestamp with timezone.
//! 
//! Generated from: EffectiveAt.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectiveAt {
    // Empty struct - no properties defined in schema
}

impl EffectiveAt {
    /// Create a new EffectiveAt with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EffectiveAt {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_at_creation() {
        let instance = EffectiveAt::new();
        assert_eq!(instance, EffectiveAt::default());
    }

    #[test]
    fn test_effective_at_serialization() {
        let instance = EffectiveAt::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: EffectiveAt = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}