#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConsolidationLevel {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Yearly")]
    Yearly,
    #[serde(rename = "Archived")]
    Archived,
}