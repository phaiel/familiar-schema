//! BondStateChangeRequested - An event published to request a change in a Bond's lifecycle state.
//! 
//! Generated from: BondStateChangeRequested.schema.json
//! Category: events
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// An event published to request a change in a Bond's lifecycle state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BondStateChangeRequested {
    /// A reusable definition for a unique entity identifier.
    pub event_id: String,
    /// A reusable definition for a unique entity identifier.
    pub trace_id: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub timestamp: String,
    pub source_service: String,
    pub payload: serde_json::Value,
}

impl BondStateChangeRequested {
    /// Create a new BondStateChangeRequested with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BondStateChangeRequested {
    fn default() -> Self {
        Self {
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
    fn test_bond_state_change_requested_creation() {
        let instance = BondStateChangeRequested::new();
        assert_eq!(instance, BondStateChangeRequested::default());
    }

    #[test]
    fn test_bond_state_change_requested_serialization() {
        let instance = BondStateChangeRequested::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: BondStateChangeRequested = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}