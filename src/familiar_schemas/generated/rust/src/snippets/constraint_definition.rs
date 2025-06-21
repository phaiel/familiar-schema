//! ConstraintDefinition - A canonical definition for field validation constraints.
//! 
//! Generated from: ConstraintDefinition.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A canonical definition for field validation constraints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstraintDefinition {
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    #[serde(rename = "minLength")]
    pub min_length: Option<i64>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<i64>,
    pub pattern: Option<String>,
    pub enum: Option<Vec<String>>,
}

impl ConstraintDefinition {
    /// Create a new ConstraintDefinition with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ConstraintDefinition {
    fn default() -> Self {
        Self {
            minimum: None,
            maximum: None,
            min_length: None,
            max_length: None,
            pattern: None,
            enum: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_definition_creation() {
        let instance = ConstraintDefinition::new();
        assert_eq!(instance, ConstraintDefinition::default());
    }

    #[test]
    fn test_constraint_definition_serialization() {
        let instance = ConstraintDefinition::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ConstraintDefinition = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}