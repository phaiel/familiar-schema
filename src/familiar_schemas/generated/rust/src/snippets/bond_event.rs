//! BondEvent - Represents a significant event that impacted a bond's strength or state.
//! 
//! Generated from: BondEvent.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents a significant event that impacted a bond's strength or state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BondEvent {
    pub moment_id: String,
    /// How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).
    pub impact: f64,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub timestamp: String,
}

impl BondEvent {
    /// Create a new BondEvent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BondEvent {
    fn default() -> Self {
        Self {
            moment_id: String::new(),
            impact: 0.0,
            timestamp: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bond_event_creation() {
        let instance = BondEvent::new();
        assert_eq!(instance, BondEvent::default());
    }

    #[test]
    fn test_bond_event_serialization() {
        let instance = BondEvent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: BondEvent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}