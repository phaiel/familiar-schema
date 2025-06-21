#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum FilamentType {
    #[serde(rename = "PersonalityTrait")]
    PersonalityTrait,
    #[serde(rename = "Worldview")]
    Worldview,
    #[serde(rename = "Relationship_Pattern")]
    RelationshipPattern,
    #[serde(rename = "Life_Theme")]
    LifeTheme,
}