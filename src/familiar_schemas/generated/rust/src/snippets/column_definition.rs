//! ColumnDefinition - Schema for defining a database table column.
//! 
//! Generated from: ColumnDefinition.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Schema for defining a database table column.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnDefinition {
    /// The primary, human-readable name of an entity.
    pub name: String,
    pub type: String,
    pub nullable: Option<bool>,
    pub primary_key: Option<bool>,
    pub unique: Option<bool>,
}

impl ColumnDefinition {
    /// Create a new ColumnDefinition with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ColumnDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            type: String::new(),
            nullable: None,
            primary_key: None,
            unique: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_definition_creation() {
        let instance = ColumnDefinition::new();
        assert_eq!(instance, ColumnDefinition::default());
    }

    #[test]
    fn test_column_definition_serialization() {
        let instance = ColumnDefinition::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ColumnDefinition = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}