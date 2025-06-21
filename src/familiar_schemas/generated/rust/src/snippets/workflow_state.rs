//! WorkflowState - Represents the state of a long-running, orchestrated workflow.
//! 
//! Generated from: WorkflowState.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents the state of a long-running, orchestrated workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowState {
    /// The current status of a task or process.
    pub status: String,
    pub current_step: String,
    pub error_message: Option<String>,
    /// A canonical definition for an optional ISO 8601 timestamp with timezone.
    pub last_updated: Option<String>,
}

impl WorkflowState {
    /// Create a new WorkflowState with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for WorkflowState {
    fn default() -> Self {
        Self {
            status: String::new(),
            current_step: String::new(),
            error_message: None,
            last_updated: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_state_creation() {
        let instance = WorkflowState::new();
        assert_eq!(instance, WorkflowState::default());
    }

    #[test]
    fn test_workflow_state_serialization() {
        let instance = WorkflowState::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: WorkflowState = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}