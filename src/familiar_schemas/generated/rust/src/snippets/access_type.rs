#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AccessType {
    #[serde(rename = "DirectQuery")]
    DirectQuery,
    #[serde(rename = "SearchResult")]
    SearchResult,
    #[serde(rename = "ConsolidationProcess")]
    ConsolidationProcess,
    #[serde(rename = "AgentAnalysis")]
    AgentAnalysis,
    #[serde(rename = "UserInteraction")]
    UserInteraction,
}