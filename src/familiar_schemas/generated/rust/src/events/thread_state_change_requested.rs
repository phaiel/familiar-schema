//! ThreadStateChangeRequested - An event published to request a change in a Thread's lifecycle state.
//! 
//! Generated from: ThreadStateChangeRequested.schema.json
//! Category: events
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// An event published to request a change in a Thread's lifecycle state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadStateChangeRequested {
    /// The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').
    pub title: String,
    /// A clear, complete sentence explaining the object's purpose and function within the system.
    pub description: String,
    /// The semantic version of this schema definition (e.g., '1.0.0').
    pub schema_version: String,
    /// A reusable definition for a unique entity identifier.
    pub event_id: String,
    /// A reusable definition for a unique entity identifier.
    pub trace_id: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub timestamp: String,
    pub source_service: String,
    pub payload: serde_json::Value,
}

impl ThreadStateChangeRequested {
    /// Create a new ThreadStateChangeRequested with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ThreadStateChangeRequested {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            schema_version: String::new(),
            event_id: String::new(),
            trace_id: String::new(),
            timestamp: String::new(),
            source_service: String::new(),
            payload: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_state_change_requested_creation() {
        let instance = ThreadStateChangeRequested::new();
        assert_eq!(instance, ThreadStateChangeRequested::default());
    }

    #[test]
    fn test_thread_state_change_requested_serialization() {
        let instance = ThreadStateChangeRequested::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ThreadStateChangeRequested = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}