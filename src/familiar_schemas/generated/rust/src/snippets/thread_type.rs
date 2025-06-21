#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ThreadType {
    #[serde(rename = "Person")]
    Person,
    #[serde(rename = "Place")]
    Place,
    #[serde(rename = "Concept")]
    Concept,
    #[serde(rename = "GenericObject")]
    GenericObject,
}