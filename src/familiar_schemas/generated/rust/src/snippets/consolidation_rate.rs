//! ConsolidationRate - The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.
//! 
//! Generated from: ConsolidationRate.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsolidationRate {
    // Empty struct - no properties defined in schema
}

impl ConsolidationRate {
    /// Create a new ConsolidationRate with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ConsolidationRate {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidation_rate_creation() {
        let instance = ConsolidationRate::new();
        assert_eq!(instance, ConsolidationRate::default());
    }

    #[test]
    fn test_consolidation_rate_serialization() {
        let instance = ConsolidationRate::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ConsolidationRate = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}