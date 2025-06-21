//! FilamentContent - Defines the high-level narrative or belief content of a Filament entity. This is a quantum component.
//! 
//! Generated from: FilamentContent.schema.json
//! Category: components
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the high-level narrative or belief content of a Filament entity. This is a quantum component.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilamentContent {
    /// Defines the common physics-related properties for components and laws.
    pub physics_properties: serde_json::Value,
    /// A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.
    pub fields: serde_json::Value,
}

impl FilamentContent {
    /// Create a new FilamentContent with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for FilamentContent {
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
    fn test_filament_content_creation() {
        let instance = FilamentContent::new();
        assert_eq!(instance, FilamentContent::default());
    }

    #[test]
    fn test_filament_content_serialization() {
        let instance = FilamentContent::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: FilamentContent = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}