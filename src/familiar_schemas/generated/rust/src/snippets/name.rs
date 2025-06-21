//! Name - The primary, human-readable name of an entity.
//! 
//! Generated from: Name.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The primary, human-readable name of an entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Name {
    // Empty struct - no properties defined in schema
}

impl Name {
    /// Create a new Name with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Name {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name_creation() {
        let instance = Name::new();
        assert_eq!(instance, Name::default());
    }

    #[test]
    fn test_name_serialization() {
        let instance = Name::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Name = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}