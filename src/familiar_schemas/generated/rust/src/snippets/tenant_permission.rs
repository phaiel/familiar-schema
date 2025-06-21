#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TenantPermission {
    #[serde(rename = "Read")]
    Read,
    #[serde(rename = "Write")]
    Write,
    #[serde(rename = "Contribute")]
    Contribute,
    #[serde(rename = "Administer")]
    Administer,
}