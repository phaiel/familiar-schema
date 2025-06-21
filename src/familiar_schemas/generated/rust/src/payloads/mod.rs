//! Payloads module
//!
//! Generated Rust structs for payloads schemas.
//! Generated: 2025-06-20 02:54:12 UTC

pub mod course_payload;
pub mod draft_payload;
pub mod reconciliation_result_payload;
pub mod shuttle_payload;
pub mod stitch_interaction_request;
pub mod stitch_interaction_response;
pub mod weave_payload;
pub mod weave_unit_payload;

// Re-export all types
pub use course_payload::CoursePayload;
pub use draft_payload::DraftPayload;
pub use reconciliation_result_payload::ReconciliationResultPayload;
pub use shuttle_payload::ShuttlePayload;
pub use stitch_interaction_request::StitchInteractionRequest;
pub use stitch_interaction_response::StitchInteractionResponse;
pub use weave_payload::WeavePayload;
pub use weave_unit_payload::WeaveUnitPayload;
