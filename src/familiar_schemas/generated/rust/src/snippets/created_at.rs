//! CreatedAt - A canonical definition for an ISO 8601 timestamp with timezone.
//! 
//! Generated from: CreatedAt.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatedAt {
    // Empty struct - no properties defined in schema
}

impl CreatedAt {
    /// Create a new CreatedAt with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CreatedAt {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_created_at_creation() {
        let instance = CreatedAt::new();
        assert_eq!(instance, CreatedAt::default());
    }

    #[test]
    fn test_created_at_serialization() {
        let instance = CreatedAt::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: CreatedAt = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}