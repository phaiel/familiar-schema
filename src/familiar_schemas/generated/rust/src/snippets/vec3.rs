//! Vec3 - A reusable data type for a vector of three f64 numbers, representing a physical property like position or velocity.
//! 
//! Generated from: Vec3.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// A reusable data type for a vector of three f64 numbers, representing a physical property like position or velocity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    // Empty struct - no properties defined in schema
}

impl Vec3 {
    /// Create a new Vec3 with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_creation() {
        let instance = Vec3::new();
        assert_eq!(instance, Vec3::default());
    }

    #[test]
    fn test_vec3_serialization() {
        let instance = Vec3::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: Vec3 = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}