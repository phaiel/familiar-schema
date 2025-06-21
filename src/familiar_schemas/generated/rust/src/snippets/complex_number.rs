//! ComplexNumber - Represents a complex number with real and imaginary parts.
//! 
//! Generated from: ComplexNumber.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Represents a complex number with real and imaginary parts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexNumber {
    pub real: f64,
    pub imaginary: f64,
}

impl ComplexNumber {
    /// Create a new ComplexNumber with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ComplexNumber {
    fn default() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_number_creation() {
        let instance = ComplexNumber::new();
        assert_eq!(instance, ComplexNumber::default());
    }

    #[test]
    fn test_complex_number_serialization() {
        let instance = ComplexNumber::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: ComplexNumber = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}