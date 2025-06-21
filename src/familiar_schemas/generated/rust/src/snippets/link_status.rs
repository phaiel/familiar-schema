#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum LinkStatus {
    #[serde(rename = "PendingApproval")]
    PendingApproval,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Revoked")]
    Revoked,
    #[serde(rename = "Declined")]
    Declined,
}