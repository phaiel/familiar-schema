//! NullableTimestamp - A canonical definition for an optional ISO 8601 timestamp with timezone.
//! 
//! Generated from: NullableTimestamp.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an optional ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NullableTimestamp {
    // Empty struct - no properties defined in schema
}

impl NullableTimestamp {
    /// Create a new NullableTimestamp with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for NullableTimestamp {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nullable_timestamp_creation() {
        let instance = NullableTimestamp::new();
        assert_eq!(instance, NullableTimestamp::default());
    }

    #[test]
    fn test_nullable_timestamp_serialization() {
        let instance = NullableTimestamp::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: NullableTimestamp = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}