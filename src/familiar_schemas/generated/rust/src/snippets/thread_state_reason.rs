#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ThreadStateReason {
    #[serde(rename = "UserMarkedInactive")]
    UserMarkedInactive,
    #[serde(rename = "UserMarkedDeceased")]
    UserMarkedDeceased,
    #[serde(rename = "SystemDetectedInactivity")]
    SystemDetectedInactivity,
    #[serde(rename = "LifecycleCompleted")]
    LifecycleCompleted,
}