//! Taxonomy module
//!
//! Generated Rust structs for taxonomy schemas.

pub mod emotionalvalence;
pub mod physicsprofile;

// Re-export all types
pub use emotionalvalence::{ConversionError, EmotionalValenceTaxonomy};
pub use physicsprofile::{ConversionError, PhysicsProfileTaxonomy};
