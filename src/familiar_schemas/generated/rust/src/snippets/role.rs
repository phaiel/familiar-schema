#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Role {
    #[serde(rename = "Admin")]
    Admin,
    #[serde(rename = "Member")]
    Member,
    #[serde(rename = "Child")]
    Child,
    #[serde(rename = "Guardian")]
    Guardian,
    #[serde(rename = "ReadOnly")]
    ReadOnly,
}