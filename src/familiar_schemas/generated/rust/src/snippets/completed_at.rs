//! CompletedAt - A canonical definition for an optional ISO 8601 timestamp with timezone.
//! 
//! Generated from: CompletedAt.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an optional ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompletedAt {
    // Empty struct - no properties defined in schema
}

impl CompletedAt {
    /// Create a new CompletedAt with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CompletedAt {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completed_at_creation() {
        let instance = CompletedAt::new();
        assert_eq!(instance, CompletedAt::default());
    }

    #[test]
    fn test_completed_at_serialization() {
        let instance = CompletedAt::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: CompletedAt = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}