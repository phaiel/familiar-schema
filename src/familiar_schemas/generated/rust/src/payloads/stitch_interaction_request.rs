//! StitchInteractionRequest - The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.
//! 
//! Generated from: StitchInteractionRequest.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StitchInteractionRequest {
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub interaction_id: String,
    /// A canonical definition for a short, human-readable UI label or title.
    pub prompt_title: String,
    /// A canonical, reusable definition for a human-readable description field.
    pub prompt_details: String,
    pub interaction_type: String,
    /// A canonical, reusable definition for a human-readable description field.
    pub agent_context: Option<String>,
}

impl StitchInteractionRequest {
    /// Create a new StitchInteractionRequest with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StitchInteractionRequest {
    fn default() -> Self {
        Self {
            interaction_id: String::new(),
            prompt_title: String::new(),
            prompt_details: String::new(),
            interaction_type: String::new(),
            agent_context: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stitch_interaction_request_creation() {
        let instance = StitchInteractionRequest::new();
        assert_eq!(instance, StitchInteractionRequest::default());
    }

    #[test]
    fn test_stitch_interaction_request_serialization() {
        let instance = StitchInteractionRequest::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: StitchInteractionRequest = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}