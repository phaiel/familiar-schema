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
#[doc = "`CourseData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"CourseData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"analysis_type\","]
#[doc = "    \"query\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active_shuttle_ids\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "      \"title\": \"Entity ID List\","]
#[doc = "      \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"analysis_type\": {"]
#[doc = "      \"description\": \"The type of deep analysis to be performed.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"CorrelationAnalysis\","]
#[doc = "        \"StructuralResonance\","]
#[doc = "        \"DensityIntelligence\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "      \"title\": \"Status Field\","]
#[doc = "      \"description\": \"The current status of a task or process.\","]
#[doc = "      \"default\": \"Pending\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Pending\","]
#[doc = "        \"InProgress\","]
#[doc = "        \"Completed\","]
#[doc = "        \"Cancelled\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CourseData {
    #[doc = "A canonical definition for a list of unique entity identifiers (UUIDs)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub active_shuttle_ids: ::std::option::Option<Vec<::std::string::String>>,
    #[doc = "The type of deep analysis to be performed."]
    pub analysis_type: CourseDataAnalysisType,
    pub query: ::std::string::String,
    #[doc = "The current status of a task or process."]
    pub status: StatusField,
}
impl ::std::convert::From<&CourseData> for CourseData {
    fn from(value: &CourseData) -> Self {
        value.clone()
    }
}
impl CourseData {
    pub fn builder() -> builder::CourseData {
        Default::default()
    }
}
#[doc = "The type of deep analysis to be performed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The type of deep analysis to be performed.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"CorrelationAnalysis\","]
#[doc = "    \"StructuralResonance\","]
#[doc = "    \"DensityIntelligence\""]
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
pub enum CourseDataAnalysisType {
    CorrelationAnalysis,
    StructuralResonance,
    DensityIntelligence,
}
impl ::std::convert::From<&Self> for CourseDataAnalysisType {
    fn from(value: &CourseDataAnalysisType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CourseDataAnalysisType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::CorrelationAnalysis => write!(f, "CorrelationAnalysis"),
            Self::StructuralResonance => write!(f, "StructuralResonance"),
            Self::DensityIntelligence => write!(f, "DensityIntelligence"),
        }
    }
}
impl ::std::str::FromStr for CourseDataAnalysisType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CorrelationAnalysis" => Ok(Self::CorrelationAnalysis),
            "StructuralResonance" => Ok(Self::StructuralResonance),
            "DensityIntelligence" => Ok(Self::DensityIntelligence),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CourseDataAnalysisType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CourseDataAnalysisType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CourseDataAnalysisType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Represents a high-level, long-running cognitive analysis task or workflow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/CoursePayload.v1.schema.json\","]
#[doc = "  \"title\": \"Course Payload\","]
#[doc = "  \"description\": \"Represents a high-level, long-running cognitive analysis task or workflow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"CourseData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"analysis_type\","]
#[doc = "        \"query\","]
#[doc = "        \"status\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"active_shuttle_ids\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "          \"title\": \"Entity ID List\","]
#[doc = "          \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"analysis_type\": {"]
#[doc = "          \"description\": \"The type of deep analysis to be performed.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"CorrelationAnalysis\","]
#[doc = "            \"StructuralResonance\","]
#[doc = "            \"DensityIntelligence\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"query\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "          \"title\": \"Status Field\","]
#[doc = "          \"description\": \"The current status of a task or process.\","]
#[doc = "          \"default\": \"Pending\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Pending\","]
#[doc = "            \"InProgress\","]
#[doc = "            \"Completed\","]
#[doc = "            \"Cancelled\""]
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
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"payloads/CoursePayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CoursePayload {
    pub data: CourseData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&CoursePayload> for CoursePayload {
    fn from(value: &CoursePayload) -> Self {
        value.clone()
    }
}
impl CoursePayload {
    pub fn builder() -> builder::CoursePayload {
        Default::default()
    }
}
#[doc = "The current status of a task or process."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "  \"title\": \"Status Field\","]
#[doc = "  \"description\": \"The current status of a task or process.\","]
#[doc = "  \"default\": \"Pending\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Pending\","]
#[doc = "    \"InProgress\","]
#[doc = "    \"Completed\","]
#[doc = "    \"Cancelled\""]
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
pub enum StatusField {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}
impl ::std::convert::From<&Self> for StatusField {
    fn from(value: &StatusField) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StatusField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => write!(f, "Pending"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Completed => write!(f, "Completed"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}
impl ::std::str::FromStr for StatusField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Pending" => Ok(Self::Pending),
            "InProgress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for StatusField {
    fn default() -> Self {
        StatusField::Pending
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
    pub struct CourseData {
        active_shuttle_ids: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        analysis_type: ::std::result::Result<super::CourseDataAnalysisType, ::std::string::String>,
        query: ::std::result::Result<::std::string::String, ::std::string::String>,
        status: ::std::result::Result<super::StatusField, ::std::string::String>,
    }
    impl ::std::default::Default for CourseData {
        fn default() -> Self {
            Self {
                active_shuttle_ids: Ok(Default::default()),
                analysis_type: Err("no value supplied for analysis_type".to_string()),
                query: Err("no value supplied for query".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl CourseData {
        pub fn active_shuttle_ids<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.active_shuttle_ids = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for active_shuttle_ids: {}",
                    e
                )
            });
            self
        }
        pub fn analysis_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CourseDataAnalysisType>,
            T::Error: ::std::fmt::Display,
        {
            self.analysis_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for analysis_type: {}", e));
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for query: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StatusField>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<CourseData> for super::CourseData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CourseData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active_shuttle_ids: value.active_shuttle_ids?,
                analysis_type: value.analysis_type?,
                query: value.query?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::CourseData> for CourseData {
        fn from(value: super::CourseData) -> Self {
            Self {
                active_shuttle_ids: Ok(value.active_shuttle_ids),
                analysis_type: Ok(value.analysis_type),
                query: Ok(value.query),
                status: Ok(value.status),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CoursePayload {
        data: ::std::result::Result<super::CourseData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for CoursePayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl CoursePayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CourseData>,
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
    impl ::std::convert::TryFrom<CoursePayload> for super::CoursePayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CoursePayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::CoursePayload> for CoursePayload {
        fn from(value: super::CoursePayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
}
