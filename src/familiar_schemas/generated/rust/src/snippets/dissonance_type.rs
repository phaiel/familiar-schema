#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DissonanceType {
    #[serde(rename = "TemporalOverlap")]
    TemporalOverlap,
    #[serde(rename = "LogicalContradiction")]
    LogicalContradiction,
    #[serde(rename = "LowConfidencePattern")]
    LowConfidencePattern,
    #[serde(rename = "UnresolvedEntity")]
    UnresolvedEntity,
}