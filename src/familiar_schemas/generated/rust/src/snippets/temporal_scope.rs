#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TemporalScope {
    #[serde(rename = "Daily")]
    Daily,
    #[serde(rename = "Weekly")]
    Weekly,
    #[serde(rename = "Monthly")]
    Monthly,
    #[serde(rename = "Quarterly")]
    Quarterly,
    #[serde(rename = "Yearly")]
    Yearly,
    #[serde(rename = "Ongoing")]
    Ongoing,
}