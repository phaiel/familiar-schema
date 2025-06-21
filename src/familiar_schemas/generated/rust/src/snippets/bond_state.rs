#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BondState {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Strained")]
    Strained,
    #[serde(rename = "Dissolved")]
    Dissolved,
    #[serde(rename = "Rekindled")]
    Rekindled,
}