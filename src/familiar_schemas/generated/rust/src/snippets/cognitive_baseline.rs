//! CognitiveBaseline - Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.
//! 
//! Generated from: CognitiveBaseline.schema.json
//! Category: snippets
//! Generated: 2025-06-20 02:54:12 UTC

use serde::{Deserialize, Serialize};

/// Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitiveBaseline {
    /// How strongly emotional events affect this person. (UI Label: Emotional Reactivity)
    pub emotional_volatility: f64,
    /// How energized they are by social interactions. (UI Label: Social Energy)
    pub social_energy_factor: f64,
    /// How quickly their memories become stable. (UI Label: Memory Consolidation)
    pub consolidation_rate_modifier: f64,
    /// How resistant their bonds are to change. (UI Label: Relationship Stability)
    pub bond_damping_factor: f64,
    /// Tendency for creative, associative thinking. (UI Label: Openness to Experience)
    pub exploration_bias: f64,
}

impl CognitiveBaseline {
    /// Create a new CognitiveBaseline with default values
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CognitiveBaseline {
    fn default() -> Self {
        Self {
            emotional_volatility: 0.0,
            social_energy_factor: 0.0,
            consolidation_rate_modifier: 0.0,
            bond_damping_factor: 0.0,
            exploration_bias: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_baseline_creation() {
        let instance = CognitiveBaseline::new();
        assert_eq!(instance, CognitiveBaseline::default());
    }

    #[test]
    fn test_cognitive_baseline_serialization() {
        let instance = CognitiveBaseline::default();
        
        // Test JSON serialization
        let json = serde_json::to_string(&instance).expect("Failed to serialize");
        let deserialized: CognitiveBaseline = serde_json::from_str(&json).expect("Failed to deserialize");
        
        assert_eq!(instance, deserialized);
    }
}