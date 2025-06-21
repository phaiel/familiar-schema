//! TaskStatus - Tracks the completion status of an Intent.
//! 
//! Generated from: TaskStatus.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Tracks the completion status of an Intent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskStatus {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl TaskStatus {
    /// Create a new TaskStatus with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self {
            physics_properties: serde_json::Value::Null,
            fields: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_creation() {
        let instance = TaskStatus::new();
        assert_eq!(instance, TaskStatus::default());
    }

    #[test]
    fn test_task_status_serialization() {
        let instance = TaskStatus::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: TaskStatus = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}