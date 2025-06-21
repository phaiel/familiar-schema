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
#[doc = "`BaseUserContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BaseUserContext\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"user_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"session_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseUserContext {
    #[doc = "A reusable definition for a unique entity identifier."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub user_id: ::std::string::String,
}
impl ::std::convert::From<&BaseUserContext> for BaseUserContext {
    fn from(value: &BaseUserContext) -> Self {
        value.clone()
    }
}
impl BaseUserContext {
    pub fn builder() -> builder::BaseUserContext {
        Default::default()
    }
}
#[doc = "`ReconciliationResultData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReconciliationResultData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"confidence_score\","]
#[doc = "    \"selected_candidate_id\","]
#[doc = "    \"task_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent_reasoning\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"confidence_score\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"selected_candidate_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"task_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "      \"title\": \"Reconciliation Task Type\","]
#[doc = "      \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Structural\","]
#[doc = "        \"Emotional\","]
#[doc = "        \"Identity\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReconciliationResultData {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_reasoning: ::std::option::Option<::std::string::String>,
    pub confidence_score: f64,
    pub selected_candidate_id: ::std::string::String,
    #[doc = "A canonical enum for the types of reconciliation tasks performed by the Heddle engine."]
    pub task_type: ReconciliationTaskType,
}
impl ::std::convert::From<&ReconciliationResultData> for ReconciliationResultData {
    fn from(value: &ReconciliationResultData) -> Self {
        value.clone()
    }
}
impl ReconciliationResultData {
    pub fn builder() -> builder::ReconciliationResultData {
        Default::default()
    }
}
#[doc = "The structured output from a single Heddle reconciliation task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/ReconciliationResultPayload.v1.schema.json\","]
#[doc = "  \"title\": \"Reconciliation Result Payload\","]
#[doc = "  \"description\": \"The structured output from a single Heddle reconciliation task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"ReconciliationResultData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"confidence_score\","]
#[doc = "        \"selected_candidate_id\","]
#[doc = "        \"task_type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent_reasoning\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"confidence_score\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"selected_candidate_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"task_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "          \"title\": \"Reconciliation Task Type\","]
#[doc = "          \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Structural\","]
#[doc = "            \"Emotional\","]
#[doc = "            \"Identity\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"user_context\": {"]
#[doc = "      \"title\": \"BaseUserContext\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"user_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"session_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"payloads\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"payloads/ReconciliationResultPayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReconciliationResultPayload {
    pub data: ReconciliationResultData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&ReconciliationResultPayload> for ReconciliationResultPayload {
    fn from(value: &ReconciliationResultPayload) -> Self {
        value.clone()
    }
}
impl ReconciliationResultPayload {
    pub fn builder() -> builder::ReconciliationResultPayload {
        Default::default()
    }
}
#[doc = "A canonical enum for the types of reconciliation tasks performed by the Heddle engine."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "  \"title\": \"Reconciliation Task Type\","]
#[doc = "  \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Structural\","]
#[doc = "    \"Emotional\","]
#[doc = "    \"Identity\""]
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
pub enum ReconciliationTaskType {
    Structural,
    Emotional,
    Identity,
}
impl ::std::convert::From<&Self> for ReconciliationTaskType {
    fn from(value: &ReconciliationTaskType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ReconciliationTaskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Structural => write!(f, "Structural"),
            Self::Emotional => write!(f, "Emotional"),
            Self::Identity => write!(f, "Identity"),
        }
    }
}
impl ::std::str::FromStr for ReconciliationTaskType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Structural" => Ok(Self::Structural),
            "Emotional" => Ok(Self::Emotional),
            "Identity" => Ok(Self::Identity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReconciliationTaskType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReconciliationTaskType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReconciliationTaskType {
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
    pub struct BaseUserContext {
        session_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseUserContext {
        fn default() -> Self {
            Self {
                session_id: Ok(Default::default()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl BaseUserContext {
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseUserContext> for super::BaseUserContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseUserContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::BaseUserContext> for BaseUserContext {
        fn from(value: super::BaseUserContext) -> Self {
            Self {
                session_id: Ok(value.session_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReconciliationResultData {
        agent_reasoning: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        confidence_score: ::std::result::Result<f64, ::std::string::String>,
        selected_candidate_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        task_type: ::std::result::Result<super::ReconciliationTaskType, ::std::string::String>,
    }
    impl ::std::default::Default for ReconciliationResultData {
        fn default() -> Self {
            Self {
                agent_reasoning: Ok(Default::default()),
                confidence_score: Err("no value supplied for confidence_score".to_string()),
                selected_candidate_id: Err(
                    "no value supplied for selected_candidate_id".to_string()
                ),
                task_type: Err("no value supplied for task_type".to_string()),
            }
        }
    }
    impl ReconciliationResultData {
        pub fn agent_reasoning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_reasoning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_reasoning: {}", e));
            self
        }
        pub fn confidence_score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence_score = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for confidence_score: {}",
                    e
                )
            });
            self
        }
        pub fn selected_candidate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.selected_candidate_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for selected_candidate_id: {}",
                    e
                )
            });
            self
        }
        pub fn task_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReconciliationTaskType>,
            T::Error: ::std::fmt::Display,
        {
            self.task_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ReconciliationResultData> for super::ReconciliationResultData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReconciliationResultData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_reasoning: value.agent_reasoning?,
                confidence_score: value.confidence_score?,
                selected_candidate_id: value.selected_candidate_id?,
                task_type: value.task_type?,
            })
        }
    }
    impl ::std::convert::From<super::ReconciliationResultData> for ReconciliationResultData {
        fn from(value: super::ReconciliationResultData) -> Self {
            Self {
                agent_reasoning: Ok(value.agent_reasoning),
                confidence_score: Ok(value.confidence_score),
                selected_candidate_id: Ok(value.selected_candidate_id),
                task_type: Ok(value.task_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReconciliationResultPayload {
        data: ::std::result::Result<super::ReconciliationResultData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for ReconciliationResultPayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl ReconciliationResultPayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReconciliationResultData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn user_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseUserContext>,
            T::Error: ::std::fmt::Display,
        {
            self.user_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_context: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ReconciliationResultPayload> for super::ReconciliationResultPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReconciliationResultPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::ReconciliationResultPayload> for ReconciliationResultPayload {
        fn from(value: super::ReconciliationResultPayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
}
