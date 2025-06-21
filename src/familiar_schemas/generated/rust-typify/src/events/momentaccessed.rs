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
#[doc = "Describes the method or reason for an entity access event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/fields/AccessType.v1.json\","]
#[doc = "  \"title\": \"Access Type Field\","]
#[doc = "  \"description\": \"Describes the method or reason for an entity access event.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"DirectQuery\","]
#[doc = "    \"SearchResult\","]
#[doc = "    \"ConsolidationProcess\","]
#[doc = "    \"AgentAnalysis\","]
#[doc = "    \"UserInteraction\""]
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
pub enum AccessTypeField {
    DirectQuery,
    SearchResult,
    ConsolidationProcess,
    AgentAnalysis,
    UserInteraction,
}
impl ::std::convert::From<&Self> for AccessTypeField {
    fn from(value: &AccessTypeField) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessTypeField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DirectQuery => write!(f, "DirectQuery"),
            Self::SearchResult => write!(f, "SearchResult"),
            Self::ConsolidationProcess => write!(f, "ConsolidationProcess"),
            Self::AgentAnalysis => write!(f, "AgentAnalysis"),
            Self::UserInteraction => write!(f, "UserInteraction"),
        }
    }
}
impl ::std::str::FromStr for AccessTypeField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DirectQuery" => Ok(Self::DirectQuery),
            "SearchResult" => Ok(Self::SearchResult),
            "ConsolidationProcess" => Ok(Self::ConsolidationProcess),
            "AgentAnalysis" => Ok(Self::AgentAnalysis),
            "UserInteraction" => Ok(Self::UserInteraction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Published when a Moment entity is accessed, triggering the MemoryConsolidation law."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/events/MomentAccessed.v1.schema.json\","]
#[doc = "  \"title\": \"Moment Accessed Event\","]
#[doc = "  \"description\": \"Published when a Moment entity is accessed, triggering the MemoryConsolidation law.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event_id\","]
#[doc = "    \"payload\","]
#[doc = "    \"source_service\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"trace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"event_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"payload\": {"]
#[doc = "      \"title\": \"MomentAccessedPayload\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"access_type\","]
#[doc = "        \"accessed_by_user_id\","]
#[doc = "        \"moment_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"access_context\": {"]
#[doc = "          \"description\": \"Optional context about the access, like the search query ID or agent task ID.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"access_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/AccessType.v1.json\","]
#[doc = "          \"title\": \"Access Type Field\","]
#[doc = "          \"description\": \"Describes the method or reason for an entity access event.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"DirectQuery\","]
#[doc = "            \"SearchResult\","]
#[doc = "            \"ConsolidationProcess\","]
#[doc = "            \"AgentAnalysis\","]
#[doc = "            \"UserInteraction\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"accessed_by_user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"moment_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
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
#[doc = "    \"trace_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"events\","]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"events/MomentAccessed.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MomentAccessedEvent {
    #[doc = "A reusable definition for a unique entity identifier."]
    pub event_id: ::std::string::String,
    pub payload: MomentAccessedPayload,
    pub source_service: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub trace_id: ::std::string::String,
}
impl ::std::convert::From<&MomentAccessedEvent> for MomentAccessedEvent {
    fn from(value: &MomentAccessedEvent) -> Self {
        value.clone()
    }
}
impl MomentAccessedEvent {
    pub fn builder() -> builder::MomentAccessedEvent {
        Default::default()
    }
}
#[doc = "`MomentAccessedPayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MomentAccessedPayload\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"access_type\","]
#[doc = "    \"accessed_by_user_id\","]
#[doc = "    \"moment_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"access_context\": {"]
#[doc = "      \"description\": \"Optional context about the access, like the search query ID or agent task ID.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"access_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/AccessType.v1.json\","]
#[doc = "      \"title\": \"Access Type Field\","]
#[doc = "      \"description\": \"Describes the method or reason for an entity access event.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"DirectQuery\","]
#[doc = "        \"SearchResult\","]
#[doc = "        \"ConsolidationProcess\","]
#[doc = "        \"AgentAnalysis\","]
#[doc = "        \"UserInteraction\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"accessed_by_user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"moment_id\": {"]
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
pub struct MomentAccessedPayload {
    #[doc = "Optional context about the access, like the search query ID or agent task ID."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub access_context: ::std::option::Option<::std::string::String>,
    #[doc = "Describes the method or reason for an entity access event."]
    pub access_type: AccessTypeField,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub accessed_by_user_id: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub moment_id: ::std::string::String,
}
impl ::std::convert::From<&MomentAccessedPayload> for MomentAccessedPayload {
    fn from(value: &MomentAccessedPayload) -> Self {
        value.clone()
    }
}
impl MomentAccessedPayload {
    pub fn builder() -> builder::MomentAccessedPayload {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct MomentAccessedEvent {
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        payload: ::std::result::Result<super::MomentAccessedPayload, ::std::string::String>,
        source_service: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
        trace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MomentAccessedEvent {
        fn default() -> Self {
            Self {
                event_id: Err("no value supplied for event_id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
                source_service: Err("no value supplied for source_service".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                trace_id: Err("no value supplied for trace_id".to_string()),
            }
        }
    }
    impl MomentAccessedEvent {
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
            T: ::std::convert::TryInto<super::MomentAccessedPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
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
    impl ::std::convert::TryFrom<MomentAccessedEvent> for super::MomentAccessedEvent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentAccessedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                payload: value.payload?,
                source_service: value.source_service?,
                timestamp: value.timestamp?,
                trace_id: value.trace_id?,
            })
        }
    }
    impl ::std::convert::From<super::MomentAccessedEvent> for MomentAccessedEvent {
        fn from(value: super::MomentAccessedEvent) -> Self {
            Self {
                event_id: Ok(value.event_id),
                payload: Ok(value.payload),
                source_service: Ok(value.source_service),
                timestamp: Ok(value.timestamp),
                trace_id: Ok(value.trace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentAccessedPayload {
        access_context: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        access_type: ::std::result::Result<super::AccessTypeField, ::std::string::String>,
        accessed_by_user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        moment_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for MomentAccessedPayload {
        fn default() -> Self {
            Self {
                access_context: Ok(Default::default()),
                access_type: Err("no value supplied for access_type".to_string()),
                accessed_by_user_id: Err("no value supplied for accessed_by_user_id".to_string()),
                moment_id: Err("no value supplied for moment_id".to_string()),
            }
        }
    }
    impl MomentAccessedPayload {
        pub fn access_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.access_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access_context: {}", e));
            self
        }
        pub fn access_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AccessTypeField>,
            T::Error: ::std::fmt::Display,
        {
            self.access_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for access_type: {}", e));
            self
        }
        pub fn accessed_by_user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.accessed_by_user_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for accessed_by_user_id: {}",
                    e
                )
            });
            self
        }
        pub fn moment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.moment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moment_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentAccessedPayload> for super::MomentAccessedPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentAccessedPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                access_context: value.access_context?,
                access_type: value.access_type?,
                accessed_by_user_id: value.accessed_by_user_id?,
                moment_id: value.moment_id?,
            })
        }
    }
    impl ::std::convert::From<super::MomentAccessedPayload> for MomentAccessedPayload {
        fn from(value: super::MomentAccessedPayload) -> Self {
            Self {
                access_context: Ok(value.access_context),
                access_type: Ok(value.access_type),
                accessed_by_user_id: Ok(value.accessed_by_user_id),
                moment_id: Ok(value.moment_id),
            }
        }
    }
}
