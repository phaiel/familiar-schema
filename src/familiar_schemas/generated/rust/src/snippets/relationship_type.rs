#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RelationshipType {
    #[serde(rename = "Family")]
    Family,
    #[serde(rename = "Friend")]
    Friend,
    #[serde(rename = "Romantic")]
    Romantic,
    #[serde(rename = "Professional")]
    Professional,
    #[serde(rename = "Acquaintance")]
    Acquaintance,
    #[serde(rename = "Adversarial")]
    Adversarial,
}