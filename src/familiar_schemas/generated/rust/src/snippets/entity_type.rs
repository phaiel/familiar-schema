#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EntityType {
    #[serde(rename = "Focus")]
    Focus,
    #[serde(rename = "Filament")]
    Filament,
    #[serde(rename = "Motif")]
    Motif,
    #[serde(rename = "Intent")]
    Intent,
    #[serde(rename = "Moment")]
    Moment,
    #[serde(rename = "Bond")]
    Bond,
    #[serde(rename = "Thread")]
    Thread,
}