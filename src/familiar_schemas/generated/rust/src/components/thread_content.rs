//! ThreadContent - Defines the core, immutable content of a Thread entity, such as its name and type.
//! 
//! Generated from: ThreadContent.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the core, immutable content of a Thread entity, such as its name and type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThreadContent {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl ThreadContent {
    /// Create a new ThreadContent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ThreadContent {
    fn default() -> Self {
        Self {
            physics_properties: serde_json::Value::Null,
            fields: serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_content_creation() {
        let instance = ThreadContent::new();
        assert_eq!(instance, ThreadContent::default());
    }

    #[test]
    fn test_thread_content_serialization() {
        let instance = ThreadContent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ThreadContent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}