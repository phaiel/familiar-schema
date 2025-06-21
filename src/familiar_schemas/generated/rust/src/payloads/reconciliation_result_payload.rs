//! ReconciliationResultPayload - The structured output from a single Heddle reconciliation task.
//! 
//! Generated from: ReconciliationResultPayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// The structured output from a single Heddle reconciliation task.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReconciliationResultPayload {
    pub user_context: serde_json::Value,
    pub data: serde_json::Value,
}

impl ReconciliationResultPayload {
    /// Create a new ReconciliationResultPayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ReconciliationResultPayload {
    fn default() -> Self {
        Self {
            user_context: serde_json::Value::Null,
            data: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconciliation_result_payload_creation() {
        let instance = ReconciliationResultPayload::new();
        assert_eq!(instance, ReconciliationResultPayload::default());
    }

    #[test]
    fn test_reconciliation_result_payload_serialization() {
        let instance = ReconciliationResultPayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ReconciliationResultPayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}