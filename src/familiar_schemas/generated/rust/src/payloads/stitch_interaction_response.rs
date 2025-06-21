//! StitchInteractionResponse - Schema for the Redpanda event carrying the user's response back to the system to resolve a cognitive dissonance.
//! 
//! Generated from: StitchInteractionResponse.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Schema for the Redpanda event carrying the user's response back to the system to resolve a cognitive dissonance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StitchInteractionResponse {
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub stitch_id: String,
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub interaction_id: String,
    /// A canonical definition for a Universally Unique Identifier (UUID).
    pub user_id: String,
    pub response_payload: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub timestamp: String,
}

impl StitchInteractionResponse {
    /// Create a new StitchInteractionResponse with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StitchInteractionResponse {
    fn default() -> Self {
        Self {
            stitch_id: String::new(),
            interaction_id: String::new(),
            user_id: String::new(),
            response_payload: String::new(),
            timestamp: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stitch_interaction_response_creation() {
        let instance = StitchInteractionResponse::new();
        assert_eq!(instance, StitchInteractionResponse::default());
    }

    #[test]
    fn test_stitch_interaction_response_serialization() {
        let instance = StitchInteractionResponse::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: StitchInteractionResponse = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}