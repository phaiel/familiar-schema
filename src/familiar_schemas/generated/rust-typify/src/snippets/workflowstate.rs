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

#[doc = "Represents the state of a long-running, orchestrated workflow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/workflow/WorkflowState.v1.json\","]
#[doc = "  \"title\": \"Workflow State\","]
#[doc = "  \"description\": \"Represents the state of a long-running, orchestrated workflow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"current_step\","]
#[doc = "    \"last_updated\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"current_step\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"error_message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"last_updated\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "      \"title\": \"Nullable Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
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
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/workflow/WorkflowState.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WorkflowState {
    pub current_step: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for an optional ISO 8601 timestamp with timezone."]
    pub last_updated: ::std::option::Option<::std::string::String>,
    #[doc = "The current status of a task or process."]
    pub status: StatusField,
}
impl ::std::convert::From<&WorkflowState> for WorkflowState {
    fn from(value: &WorkflowState) -> Self {
        value.clone()
    }
}
impl WorkflowState {
    pub fn builder() -> builder::WorkflowState {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct WorkflowState {
        current_step: ::std::result::Result<::std::string::String, ::std::string::String>,
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        last_updated: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::StatusField, ::std::string::String>,
    }
    impl ::std::default::Default for WorkflowState {
        fn default() -> Self {
            Self {
                current_step: Err("no value supplied for current_step".to_string()),
                error_message: Ok(Default::default()),
                last_updated: Err("no value supplied for last_updated".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl WorkflowState {
        pub fn current_step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current_step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for current_step: {}", e));
            self
        }
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {}", e));
            self
        }
        pub fn last_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_updated: {}", e));
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
    impl ::std::convert::TryFrom<WorkflowState> for super::WorkflowState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkflowState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current_step: value.current_step?,
                error_message: value.error_message?,
                last_updated: value.last_updated?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::WorkflowState> for WorkflowState {
        fn from(value: super::WorkflowState) -> Self {
            Self {
                current_step: Ok(value.current_step),
                error_message: Ok(value.error_message),
                last_updated: Ok(value.last_updated),
                status: Ok(value.status),
            }
        }
    }
}
