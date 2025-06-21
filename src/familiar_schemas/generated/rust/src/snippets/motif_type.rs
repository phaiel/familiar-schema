#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MotifType {
    #[serde(rename = "Behavioral")]
    Behavioral,
    #[serde(rename = "Emotional")]
    Emotional,
    #[serde(rename = "Situational")]
    Situational,
    #[serde(rename = "Temporal")]
    Temporal,
}