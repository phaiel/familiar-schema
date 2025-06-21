//! DraftPayload - A fully specified but not-yet-created cognitive entity, ready for the Decima agent to commit to the ECS.
//! 
//! Generated from: DraftPayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A fully specified but not-yet-created cognitive entity, ready for the Decima agent to commit to the ECS.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DraftPayload {
    /// The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').
    pub title: String,
    /// A clear, complete sentence explaining the object's purpose and function within the system.
    pub description: String,
    /// The semantic version of this schema definition (e.g., '1.0.0').
    pub schema_version: String,
    pub user_context: serde_json::Value,
    /// This payload's data is a valid, complete entity schema.
    pub data: String,
}

impl DraftPayload {
    /// Create a new DraftPayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DraftPayload {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            schema_version: String::new(),
            user_context: serde_json::Value::Null,
            data: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draft_payload_creation() {
        let instance = DraftPayload::new();
        assert_eq!(instance, DraftPayload::default());
    }

    #[test]
    fn test_draft_payload_serialization() {
        let instance = DraftPayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: DraftPayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}