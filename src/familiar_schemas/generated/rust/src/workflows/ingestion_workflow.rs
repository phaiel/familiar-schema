//! IngestionWorkflow - The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft entity ready for creation.
//! 
//! Generated from: IngestionWorkflow.schema.json
//! Category: workflows
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft entity ready for creation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IngestionWorkflow {
    /// The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').
    pub title: String,
    /// A clear, complete sentence explaining the object's purpose and function within the system.
    pub description: String,
    /// The semantic version of this schema definition (e.g., '1.0.0').
    pub schema_version: String,
    /// A list of task definitions for a Windmill workflow.
    pub tasks: Vec<serde_json::Value>,
}

impl IngestionWorkflow {
    /// Create a new IngestionWorkflow with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for IngestionWorkflow {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            schema_version: String::new(),
            tasks: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ingestion_workflow_creation() {
        let instance = IngestionWorkflow::new();
        assert_eq!(instance, IngestionWorkflow::default());
    }

    #[test]
    fn test_ingestion_workflow_serialization() {
        let instance = IngestionWorkflow::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: IngestionWorkflow = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}