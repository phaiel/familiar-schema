#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BondPermissions {
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "Write")]
    Write,
    #[serde(rename = "Delete")]
    Delete,
    #[serde(rename = "Share")]
    Share,
}