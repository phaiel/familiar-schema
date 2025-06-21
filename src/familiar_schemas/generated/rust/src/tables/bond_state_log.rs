//! Bond_state_log - Defines the schema for the 'bond_state_log' database table, which stores an append-only history of a Bond's lifecycle state.
//! 
//! Generated from: Bond_state_log.schema.json
//! Category: tables
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the schema for the 'bond_state_log' database table, which stores an append-only history of a Bond's lifecycle state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond_state_log {
    /// The physical name of the table in the database.
    #[serde(rename = "tableName")]
    pub table_name: String,
    /// A map of column names to their definitions.
    pub columns: serde_json::Value,
}

impl Bond_state_log {
    /// Create a new Bond_state_log with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Bond_state_log {
    fn default() -> Self {
        Self {
            table_name: String::new(),
            columns: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_state_log_creation() {
        let instance = Bond_state_log::new();
        assert_eq!(instance, Bond_state_log::default());
    }

    #[test]
    fn test_bond_state_log_serialization() {
        let instance = Bond_state_log::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Bond_state_log = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}