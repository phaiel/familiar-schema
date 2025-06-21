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
#[doc = "`WeaveData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WeaveData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source_type\","]
#[doc = "    \"text_content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/classification/WeaveType.v1.json\","]
#[doc = "      \"title\": \"Weave Type\","]
#[doc = "      \"description\": \"Classifies the source format of a user's input 'Weave'.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"text\","]
#[doc = "        \"audio\","]
#[doc = "        \"image\","]
#[doc = "        \"document\","]
#[doc = "        \"system_event\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text_content\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeaveData {
    #[doc = "Classifies the source format of a user's input 'Weave'."]
    pub source_type: WeaveType,
    pub text_content: ::std::string::String,
}
impl ::std::convert::From<&WeaveData> for WeaveData {
    fn from(value: &WeaveData) -> Self {
        value.clone()
    }
}
impl WeaveData {
    pub fn builder() -> builder::WeaveData {
        Default::default()
    }
}
#[doc = "Represents a complete, raw text input from a user, ready for ingestion."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/WeavePayload.v1.schema.json\","]
#[doc = "  \"title\": \"Weave Payload\","]
#[doc = "  \"description\": \"Represents a complete, raw text input from a user, ready for ingestion.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"WeaveData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"source_type\","]
#[doc = "        \"text_content\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"source_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/WeaveType.v1.json\","]
#[doc = "          \"title\": \"Weave Type\","]
#[doc = "          \"description\": \"Classifies the source format of a user's input 'Weave'.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"text\","]
#[doc = "            \"audio\","]
#[doc = "            \"image\","]
#[doc = "            \"document\","]
#[doc = "            \"system_event\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"text_content\": {"]
#[doc = "          \"type\": \"string\""]
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
#[doc = "  \"source_file\": \"payloads/WeavePayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeavePayload {
    pub data: WeaveData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&WeavePayload> for WeavePayload {
    fn from(value: &WeavePayload) -> Self {
        value.clone()
    }
}
impl WeavePayload {
    pub fn builder() -> builder::WeavePayload {
        Default::default()
    }
}
#[doc = "Classifies the source format of a user's input 'Weave'."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/classification/WeaveType.v1.json\","]
#[doc = "  \"title\": \"Weave Type\","]
#[doc = "  \"description\": \"Classifies the source format of a user's input 'Weave'.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"text\","]
#[doc = "    \"audio\","]
#[doc = "    \"image\","]
#[doc = "    \"document\","]
#[doc = "    \"system_event\""]
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
    SystemEvent,
}
impl ::std::convert::From<&Self> for WeaveType {
    fn from(value: &WeaveType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WeaveType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Text => write!(f, "text"),
            Self::Audio => write!(f, "audio"),
            Self::Image => write!(f, "image"),
            Self::Document => write!(f, "document"),
            Self::SystemEvent => write!(f, "system_event"),
        }
    }
}
impl ::std::str::FromStr for WeaveType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "text" => Ok(Self::Text),
            "audio" => Ok(Self::Audio),
            "image" => Ok(Self::Image),
            "document" => Ok(Self::Document),
            "system_event" => Ok(Self::SystemEvent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WeaveType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WeaveType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WeaveType {
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
    pub struct WeaveData {
        source_type: ::std::result::Result<super::WeaveType, ::std::string::String>,
        text_content: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WeaveData {
        fn default() -> Self {
            Self {
                source_type: Err("no value supplied for source_type".to_string()),
                text_content: Err("no value supplied for text_content".to_string()),
            }
        }
    }
    impl WeaveData {
        pub fn source_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeaveType>,
            T::Error: ::std::fmt::Display,
        {
            self.source_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_type: {}", e));
            self
        }
        pub fn text_content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text_content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_content: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeaveData> for super::WeaveData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeaveData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source_type: value.source_type?,
                text_content: value.text_content?,
            })
        }
    }
    impl ::std::convert::From<super::WeaveData> for WeaveData {
        fn from(value: super::WeaveData) -> Self {
            Self {
                source_type: Ok(value.source_type),
                text_content: Ok(value.text_content),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeavePayload {
        data: ::std::result::Result<super::WeaveData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for WeavePayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl WeavePayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeaveData>,
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
    impl ::std::convert::TryFrom<WeavePayload> for super::WeavePayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeavePayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::WeavePayload> for WeavePayload {
        fn from(value: super::WeavePayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
}
