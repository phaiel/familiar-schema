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
#[doc = "The base schema for all event payloads, ensuring they contain essential context."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Payload.v1.schema.json\","]
#[doc = "  \"title\": \"Base Event Payload\","]
#[doc = "  \"description\": \"The base schema for all event payloads, ensuring they contain essential context.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"description\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"title\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
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
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"_base/BasePayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseEventPayload {
    pub data: ::serde_json::Value,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&BaseEventPayload> for BaseEventPayload {
    fn from(value: &BaseEventPayload) -> Self {
        value.clone()
    }
}
impl BaseEventPayload {
    pub fn builder() -> builder::BaseEventPayload {
        Default::default()
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseEventPayload {
        data: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for BaseEventPayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                description: Err("no value supplied for description".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                title: Err("no value supplied for title".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl BaseEventPayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
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
    impl ::std::convert::TryFrom<BaseEventPayload> for super::BaseEventPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseEventPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                description: value.description?,
                schema_version: value.schema_version?,
                title: value.title?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::BaseEventPayload> for BaseEventPayload {
        fn from(value: super::BaseEventPayload) -> Self {
            Self {
                data: Ok(value.data),
                description: Ok(value.description),
                schema_version: Ok(value.schema_version),
                title: Ok(value.title),
                user_context: Ok(value.user_context),
            }
        }
    }
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
}
