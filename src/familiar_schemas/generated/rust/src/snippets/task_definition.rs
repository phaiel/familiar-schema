//! TaskDefinition - A canonical definition for a single task within a Windmill workflow (DAG).
//! 
//! Generated from: TaskDefinition.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for a single task within a Windmill workflow (DAG).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDefinition {
    pub id: String,
    pub type: String,
    pub path: Option<String>,
    /// A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.
    pub args: Option<serde_json::Value>,
    pub depends_on: Option<Vec<String>>,
    pub condition: Option<String>,
}

impl TaskDefinition {
    /// Create a new TaskDefinition with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TaskDefinition {
    fn default() -> Self {
        Self {
            id: String::new(),
            type: String::new(),
            path: None,
            args: None,
            depends_on: None,
            condition: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_definition_creation() {
        let instance = TaskDefinition::new();
        assert_eq!(instance, TaskDefinition::default());
    }

    #[test]
    fn test_task_definition_serialization() {
        let instance = TaskDefinition::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TaskDefinition = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}