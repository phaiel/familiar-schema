//! CoursePayload - Represents a high-level, long-running cognitive analysis task or workflow.
//! 
//! Generated from: CoursePayload.schema.json
//! Category: payloads
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents a high-level, long-running cognitive analysis task or workflow.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CoursePayload {
    pub user_context: serde_json::Value,
    pub data: serde_json::Value,
}

impl CoursePayload {
    /// Create a new CoursePayload with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CoursePayload {
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
    fn test_course_payload_creation() {
        let instance = CoursePayload::new();
        assert_eq!(instance, CoursePayload::default());
    }

    #[test]
    fn test_course_payload_serialization() {
        let instance = CoursePayload::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: CoursePayload = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}