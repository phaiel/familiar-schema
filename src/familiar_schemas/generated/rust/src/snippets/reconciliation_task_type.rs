#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ReconciliationTaskType {
    #[serde(rename = "Structural")]
    Structural,
    #[serde(rename = "Emotional")]
    Emotional,
    #[serde(rename = "Identity")]
    Identity,
}