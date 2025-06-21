#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "A canonical enum of all possible lifecycle states for a Thread entity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "  \"title\": \"Thread State\","]
#[doc = "  \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Active\","]
#[doc = "    \"Inactive\","]
#[doc = "    \"Fading\","]
#[doc = "    \"Archived\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadState {
    Active,
    Inactive,
    Fading,
    Archived,
}
impl ::std::convert::From<&Self> for ThreadState {
    fn from(value: &ThreadState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
            Self::Fading => write!(f, "Fading"),
            Self::Archived => write!(f, "Archived"),
        }
    }
}
impl ::std::str::FromStr for ThreadState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Active" => Ok(Self::Active),
            "Inactive" => Ok(Self::Inactive),
            "Fading" => Ok(Self::Fading),
            "Archived" => Ok(Self::Archived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An event published to request a change in a Thread's lifecycle state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/events/ThreadStateChangeRequested.v1.schema.json\","]
#[doc = "  \"title\": \"Thread State Change Requested Event\","]
#[doc = "  \"description\": \"An event published to request a change in a Thread's lifecycle state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"event_id\","]
#[doc = "    \"payload\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"source_service\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"title\","]
#[doc = "    \"trace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"event_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"payload\": {"]
#[doc = "      \"title\": \"ThreadStateChangeRequestedPayload\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"new_state\","]
#[doc = "        \"source\","]
#[doc = "        \"thread_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"new_state\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "          \"title\": \"Thread State\","]
#[doc = "          \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Active\","]
#[doc = "            \"Inactive\","]
#[doc = "            \"Fading\","]
#[doc = "            \"Archived\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "          \"title\": \"Thread State Reason\","]
#[doc = "          \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"UserMarkedInactive\","]
#[doc = "            \"UserMarkedDeceased\","]
#[doc = "            \"SystemDetectedInactivity\","]
#[doc = "            \"LifecycleCompleted\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"User\","]
#[doc = "            \"System\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"thread_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_service\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trace_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"events\","]
#[doc = "  \"schema_version\": \"1.2.0\","]
#[doc = "  \"source_file\": \"events/ThreadStateChangeRequested.event.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStateChangeRequestedEvent {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub event_id: ::std::string::String,
    pub payload: ThreadStateChangeRequestedPayload,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    pub source_service: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub trace_id: ::std::string::String,
}
impl ::std::convert::From<&ThreadStateChangeRequestedEvent> for ThreadStateChangeRequestedEvent {
    fn from(value: &ThreadStateChangeRequestedEvent) -> Self {
        value.clone()
    }
}
impl ThreadStateChangeRequestedEvent {
    pub fn builder() -> builder::ThreadStateChangeRequestedEvent {
        Default::default()
    }
}
#[doc = "`ThreadStateChangeRequestedPayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStateChangeRequestedPayload\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"new_state\","]
#[doc = "    \"source\","]
#[doc = "    \"thread_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"new_state\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "      \"title\": \"Thread State\","]
#[doc = "      \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Active\","]
#[doc = "        \"Inactive\","]
#[doc = "        \"Fading\","]
#[doc = "        \"Archived\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "      \"title\": \"Thread State Reason\","]
#[doc = "      \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"UserMarkedInactive\","]
#[doc = "        \"UserMarkedDeceased\","]
#[doc = "        \"SystemDetectedInactivity\","]
#[doc = "        \"LifecycleCompleted\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"User\","]
#[doc = "        \"System\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"thread_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStateChangeRequestedPayload {
    #[doc = "A canonical enum of all possible lifecycle states for a Thread entity."]
    pub new_state: ThreadState,
    #[doc = "A canonical enum of the machine-readable reasons for a Thread state change."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<ThreadStateReason>,
    pub source: ThreadStateChangeRequestedPayloadSource,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub thread_id: ::std::string::String,
}
impl ::std::convert::From<&ThreadStateChangeRequestedPayload>
    for ThreadStateChangeRequestedPayload
{
    fn from(value: &ThreadStateChangeRequestedPayload) -> Self {
        value.clone()
    }
}
impl ThreadStateChangeRequestedPayload {
    pub fn builder() -> builder::ThreadStateChangeRequestedPayload {
        Default::default()
    }
}
#[doc = "`ThreadStateChangeRequestedPayloadSource`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"User\","]
#[doc = "    \"System\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadStateChangeRequestedPayloadSource {
    User,
    System,
}
impl ::std::convert::From<&Self> for ThreadStateChangeRequestedPayloadSource {
    fn from(value: &ThreadStateChangeRequestedPayloadSource) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadStateChangeRequestedPayloadSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::User => write!(f, "User"),
            Self::System => write!(f, "System"),
        }
    }
}
impl ::std::str::FromStr for ThreadStateChangeRequestedPayloadSource {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "User" => Ok(Self::User),
            "System" => Ok(Self::System),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadStateChangeRequestedPayloadSource {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadStateChangeRequestedPayloadSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadStateChangeRequestedPayloadSource {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A canonical enum of the machine-readable reasons for a Thread state change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "  \"title\": \"Thread State Reason\","]
#[doc = "  \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserMarkedInactive\","]
#[doc = "    \"UserMarkedDeceased\","]
#[doc = "    \"SystemDetectedInactivity\","]
#[doc = "    \"LifecycleCompleted\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadStateReason {
    UserMarkedInactive,
    UserMarkedDeceased,
    SystemDetectedInactivity,
    LifecycleCompleted,
}
impl ::std::convert::From<&Self> for ThreadStateReason {
    fn from(value: &ThreadStateReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserMarkedInactive => write!(f, "UserMarkedInactive"),
            Self::UserMarkedDeceased => write!(f, "UserMarkedDeceased"),
            Self::SystemDetectedInactivity => write!(f, "SystemDetectedInactivity"),
            Self::LifecycleCompleted => write!(f, "LifecycleCompleted"),
        }
    }
}
impl ::std::str::FromStr for ThreadStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UserMarkedInactive" => Ok(Self::UserMarkedInactive),
            "UserMarkedDeceased" => Ok(Self::UserMarkedDeceased),
            "SystemDetectedInactivity" => Ok(Self::SystemDetectedInactivity),
            "LifecycleCompleted" => Ok(Self::LifecycleCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ThreadStateChangeRequestedEvent {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        payload:
            ::std::result::Result<super::ThreadStateChangeRequestedPayload, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_service: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        trace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadStateChangeRequestedEvent {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                event_id: Err("no value supplied for event_id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                source_service: Err("no value supplied for source_service".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                title: Err("no value supplied for title".to_string()),
                trace_id: Err("no value supplied for trace_id".to_string()),
            }
        }
    }
    impl ThreadStateChangeRequestedEvent {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadStateChangeRequestedPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema_version: {}", e));
            self
        }
        pub fn source_service<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_service = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_service: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn trace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.trace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadStateChangeRequestedEvent>
        for super::ThreadStateChangeRequestedEvent
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadStateChangeRequestedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                event_id: value.event_id?,
                payload: value.payload?,
                schema_version: value.schema_version?,
                source_service: value.source_service?,
                timestamp: value.timestamp?,
                title: value.title?,
                trace_id: value.trace_id?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadStateChangeRequestedEvent>
        for ThreadStateChangeRequestedEvent
    {
        fn from(value: super::ThreadStateChangeRequestedEvent) -> Self {
            Self {
                description: Ok(value.description),
                event_id: Ok(value.event_id),
                payload: Ok(value.payload),
                schema_version: Ok(value.schema_version),
                source_service: Ok(value.source_service),
                timestamp: Ok(value.timestamp),
                title: Ok(value.title),
                trace_id: Ok(value.trace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadStateChangeRequestedPayload {
        new_state: ::std::result::Result<super::ThreadState, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::ThreadStateReason>,
            ::std::string::String,
        >,
        source: ::std::result::Result<
            super::ThreadStateChangeRequestedPayloadSource,
            ::std::string::String,
        >,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadStateChangeRequestedPayload {
        fn default() -> Self {
            Self {
                new_state: Err("no value supplied for new_state".to_string()),
                reason: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
            }
        }
    }
    impl ThreadStateChangeRequestedPayload {
        pub fn new_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadState>,
            T::Error: ::std::fmt::Display,
        {
            self.new_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for new_state: {}", e));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ThreadStateReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadStateChangeRequestedPayloadSource>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadStateChangeRequestedPayload>
        for super::ThreadStateChangeRequestedPayload
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadStateChangeRequestedPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                new_state: value.new_state?,
                reason: value.reason?,
                source: value.source?,
                thread_id: value.thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadStateChangeRequestedPayload>
        for ThreadStateChangeRequestedPayload
    {
        fn from(value: super::ThreadStateChangeRequestedPayload) -> Self {
            Self {
                new_state: Ok(value.new_state),
                reason: Ok(value.reason),
                source: Ok(value.source),
                thread_id: Ok(value.thread_id),
            }
        }
    }
}
