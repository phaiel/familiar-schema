#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ThreadStateResponse {
    #[serde(rename = "UserMarkedInactive")]
    UserMarkedInactive,
    #[serde(rename = "UserMarkedDeceased")]
    UserMarkedDeceased,
    #[serde(rename = "SystemDetectedInactivity")]
    SystemDetectedInactivity,
    #[serde(rename = "LifecycleCompleted")]
    LifecycleCompleted,
}