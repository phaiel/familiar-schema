//! StartDate - A canonical definition for an ISO 8601 timestamp with timezone.
//! 
//! Generated from: StartDate.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for an ISO 8601 timestamp with timezone.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartDate {
    // Empty struct - no properties defined in schema
}

impl StartDate {
    /// Create a new StartDate with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StartDate {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_date_creation() {
        let instance = StartDate::new();
        assert_eq!(instance, StartDate::default());
    }

    #[test]
    fn test_start_date_serialization() {
        let instance = StartDate::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: StartDate = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}