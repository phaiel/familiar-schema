//! EndDate - A canonical definition for an optional ISO 8601 timestamp with timezone.
//! 
//! Generated from: EndDate.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an optional ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndDate {
    // Empty struct - no properties defined in schema
}

impl EndDate {
    /// Create a new EndDate with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for EndDate {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_end_date_creation() {
        let instance = EndDate::new();
        assert_eq!(instance, EndDate::default());
    }

    #[test]
    fn test_end_date_serialization() {
        let instance = EndDate::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: EndDate = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}