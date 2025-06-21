//! Aliases - A list of alternative names for this Thread (e.g., nicknames).
//! 
//! Generated from: Aliases.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A list of alternative names for this Thread (e.g., nicknames).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aliases {
    // Empty struct - no properties defined in schema
}

impl Aliases {
    /// Create a new Aliases with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Aliases {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aliases_creation() {
        let instance = Aliases::new();
        assert_eq!(instance, Aliases::default());
    }

    #[test]
    fn test_aliases_serialization() {
        let instance = Aliases::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Aliases = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}