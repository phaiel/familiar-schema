#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MomentType {
    #[serde(rename = "Event")]
    Event,
    #[serde(rename = "Experience")]
    Experience,
    #[serde(rename = "Observation")]
    Observation,
    #[serde(rename = "Interaction")]
    Interaction,
}