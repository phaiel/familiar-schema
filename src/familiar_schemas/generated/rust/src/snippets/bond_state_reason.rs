#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BondStateReason {
    #[serde(rename = "UserInitiated")]
    UserInitiated,
    #[serde(rename = "SystemDetectedStrain")]
    SystemDetectedStrain,
    #[serde(rename = "PartnerAccountErased")]
    PartnerAccountErased,
    #[serde(rename = "RelationshipEnded")]
    RelationshipEnded,
}