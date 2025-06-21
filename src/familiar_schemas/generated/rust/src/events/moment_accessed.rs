//! MomentAccessed - Published when a Moment entity is accessed, triggering the MemoryConsolidation law.
//! 
//! Generated from: MomentAccessed.schema.json
//! Category: events
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Published when a Moment entity is accessed, triggering the MemoryConsolidation law.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MomentAccessed {
    /// A reusable definition for a unique entity identifier.
    pub event_id: String,
    /// A reusable definition for a unique entity identifier.
    pub trace_id: String,
    /// A canonical definition for an ISO 8601 timestamp with timezone.
    pub timestamp: String,
    pub source_service: String,
    pub payload: serde_json::Value,
}

impl MomentAccessed {
    /// Create a new MomentAccessed with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for MomentAccessed {
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
    fn test_moment_accessed_creation() {
        let instance = MomentAccessed::new();
        assert_eq!(instance, MomentAccessed::default());
    }

    #[test]
    fn test_moment_accessed_serialization() {
        let instance = MomentAccessed::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: MomentAccessed = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}