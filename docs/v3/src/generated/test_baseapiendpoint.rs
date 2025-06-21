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
#[doc = "The base schema for defining API endpoints, including path, method, and request/response schemas."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/ApiEndpoint.v1.schema.json\","]
#[doc = "  \"title\": \"Base API Endpoint\","]
#[doc = "  \"description\": \"The base schema for defining API endpoints, including path, method, and request/response schemas.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"method\","]
#[doc = "    \"path\","]
#[doc = "    \"response_schema\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"method\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"GET\","]
#[doc = "        \"POST\","]
#[doc = "        \"PUT\","]
#[doc = "        \"DELETE\","]
#[doc = "        \"PATCH\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"request_schema\": {"]
#[doc = "      \"description\": \"A reference to the JSON schema for the request body or parameters.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"response_schema\": {"]
#[doc = "      \"description\": \"A reference to the JSON schema for the success response body.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"security\": {"]
#[doc = "      \"description\": \"Security requirements for this endpoint.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseApiEndpoint.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseApiEndpoint {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    pub method: BaseApiEndpointMethod,
    pub path: ::std::string::String,
    #[doc = "A reference to the JSON schema for the request body or parameters."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub request_schema: ::std::option::Option<::std::string::String>,
    #[doc = "A reference to the JSON schema for the success response body."]
    pub response_schema: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "Security requirements for this endpoint."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub security: ::std::vec::Vec<::std::string::String>,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseApiEndpoint> for BaseApiEndpoint {
    fn from(value: &BaseApiEndpoint) -> Self {
        value.clone()
    }
}
impl BaseApiEndpoint {
    pub fn builder() -> builder::BaseApiEndpoint {
        Default::default()
    }
}
#[doc = "`BaseApiEndpointMethod`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"GET\","]
#[doc = "    \"POST\","]
#[doc = "    \"PUT\","]
#[doc = "    \"DELETE\","]
#[doc = "    \"PATCH\""]
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
pub enum BaseApiEndpointMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "PATCH")]
    Patch,
}
impl ::std::convert::From<&Self> for BaseApiEndpointMethod {
    fn from(value: &BaseApiEndpointMethod) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BaseApiEndpointMethod {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Patch => write!(f, "PATCH"),
        }
    }
}
impl ::std::str::FromStr for BaseApiEndpointMethod {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "DELETE" => Ok(Self::Delete),
            "PATCH" => Ok(Self::Patch),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BaseApiEndpointMethod {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BaseApiEndpointMethod {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BaseApiEndpointMethod {
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
    pub struct BaseApiEndpoint {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        method: ::std::result::Result<super::BaseApiEndpointMethod, ::std::string::String>,
        path: ::std::result::Result<::std::string::String, ::std::string::String>,
        request_schema: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        response_schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        security:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseApiEndpoint {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                method: Err("no value supplied for method".to_string()),
                path: Err("no value supplied for path".to_string()),
                request_schema: Ok(Default::default()),
                response_schema: Err("no value supplied for response_schema".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                security: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseApiEndpoint {
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
        pub fn method<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseApiEndpointMethod>,
            T::Error: ::std::fmt::Display,
        {
            self.method = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for method: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn request_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.request_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for request_schema: {}", e));
            self
        }
        pub fn response_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.response_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for response_schema: {}", e));
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
        pub fn security<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.security = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for security: {}", e));
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
    }
    impl ::std::convert::TryFrom<BaseApiEndpoint> for super::BaseApiEndpoint {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseApiEndpoint,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                method: value.method?,
                path: value.path?,
                request_schema: value.request_schema?,
                response_schema: value.response_schema?,
                schema_version: value.schema_version?,
                security: value.security?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseApiEndpoint> for BaseApiEndpoint {
        fn from(value: super::BaseApiEndpoint) -> Self {
            Self {
                description: Ok(value.description),
                method: Ok(value.method),
                path: Ok(value.path),
                request_schema: Ok(value.request_schema),
                response_schema: Ok(value.response_schema),
                schema_version: Ok(value.schema_version),
                security: Ok(value.security),
                title: Ok(value.title),
            }
        }
    }
}
