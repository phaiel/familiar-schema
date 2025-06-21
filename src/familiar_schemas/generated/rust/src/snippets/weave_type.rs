#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WeaveType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "system_event")]
    Systemevent,
}