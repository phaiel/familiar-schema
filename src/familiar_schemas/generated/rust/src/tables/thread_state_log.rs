//! Thread_state_log - Defines the schema for the 'thread_state_log' database table...
//! 
//! Generated from: Thread_state_log.schema.json
//! Category: tables
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the schema for the 'thread_state_log' database table...
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread_state_log {
    /// The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').
    pub title: String,
    /// A clear, complete sentence explaining the object's purpose and function within the system.
    pub description: String,
    /// The semantic version of this schema definition (e.g., '1.0.0').
    pub schema_version: String,
    /// The physical name of the table in the database.
    #[serde(rename = "tableName")]
    pub table_name: String,
    /// A map of column names to their definitions.
    pub columns: serde_json::Value,
}

impl Thread_state_log {
    /// Create a new Thread_state_log with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Thread_state_log {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            schema_version: String::new(),
            table_name: String::new(),
            columns: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_state_log_creation() {
        let instance = Thread_state_log::new();
        assert_eq!(instance, Thread_state_log::default());
    }

    #[test]
    fn test_thread_state_log_serialization() {
        let instance = Thread_state_log::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Thread_state_log = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}