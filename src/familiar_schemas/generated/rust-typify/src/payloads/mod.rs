//! Payloads module
//!
//! Generated Rust structs for payloads schemas.

pub mod coursepayload;
pub mod draftpayload;
pub mod reconciliationresultpayload;
pub mod shuttlepayload;
pub mod stitchinteractionrequest;
pub mod stitchinteractionresponse;
pub mod weavepayload;
pub mod weaveunitpayload;

// Re-export all types
pub use coursepayload::{BaseUserContext, ConversionError, CourseData, CourseDataAnalysisType, CoursePayload, StatusField};
pub use draftpayload::*;
pub use shuttlepayload::{BaseUserContext, ConversionError, EntityType, IngestionShuttleData, IngestionShuttlePayload, ReconciliationResultData, ReconciliationResultPayload, ReconciliationTaskType, WeaveUnitData, WeaveUnitPayload};
pub use stitchinteractionrequest::{ConversionError, StitchInteractionRequest, StitchInteractionRequestInteractionType, StitchInteractionRequestOneofItemItems, StitchInteractionType};
pub use stitchinteractionresponse::{ConversionError, StitchInteractionResponse, StitchInteractionResponseResponsePayload, StitchResponsePayload};
pub use weavepayload::{BaseUserContext, ConversionError, WeaveData, WeavePayload, WeaveType};
