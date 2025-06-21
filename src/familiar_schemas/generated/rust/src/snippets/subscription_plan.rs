#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum SubscriptionPlan {
    #[serde(rename = "Free")]
    Free,
    #[serde(rename = "Personal")]
    Personal,
    #[serde(rename = "FamilyPlus")]
    FamilyPlus,
    #[serde(rename = "Enterprise")]
    Enterprise,
}