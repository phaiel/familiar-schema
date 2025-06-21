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
#[doc = "A canonical definition for a single task within a Windmill workflow (DAG)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/workflow/TaskDefinition.v1.json\","]
#[doc = "  \"title\": \"Task Definition\","]
#[doc = "  \"description\": \"A canonical definition for a single task within a Windmill workflow (DAG).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"args\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/StringValueMap.v1.json\","]
#[doc = "      \"title\": \"String Value Map\","]
#[doc = "      \"description\": \"A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-zA-Z0-9_]+$\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "          \"title\": \"Any Value\","]
#[doc = "          \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"number\","]
#[doc = "            \"integer\","]
#[doc = "            \"boolean\","]
#[doc = "            \"object\","]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"condition\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"depends_on\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"path\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"script\","]
#[doc = "        \"rust_script\","]
#[doc = "        \"python_script\","]
#[doc = "        \"suspend\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/workflow/TaskDefinition.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskDefinition {
    #[doc = "A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub args: ::std::collections::HashMap<TaskDefinitionArgsKey, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub condition: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub depends_on: ::std::vec::Vec<::std::string::String>,
    pub id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub path: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: TaskDefinitionType,
}
impl ::std::convert::From<&TaskDefinition> for TaskDefinition {
    fn from(value: &TaskDefinition) -> Self {
        value.clone()
    }
}
impl TaskDefinition {
    pub fn builder() -> builder::TaskDefinition {
        Default::default()
    }
}
#[doc = "`TaskDefinitionArgsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-zA-Z0-9_]+$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TaskDefinitionArgsKey(::std::string::String);
impl ::std::ops::Deref for TaskDefinitionArgsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TaskDefinitionArgsKey> for ::std::string::String {
    fn from(value: TaskDefinitionArgsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TaskDefinitionArgsKey> for TaskDefinitionArgsKey {
    fn from(value: &TaskDefinitionArgsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TaskDefinitionArgsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-zA-Z0-9_]+$").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-zA-Z0-9_]+$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TaskDefinitionArgsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TaskDefinitionArgsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskDefinitionArgsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TaskDefinitionArgsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`TaskDefinitionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"script\","]
#[doc = "    \"rust_script\","]
#[doc = "    \"python_script\","]
#[doc = "    \"suspend\""]
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
pub enum TaskDefinitionType {
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "rust_script")]
    RustScript,
    #[serde(rename = "python_script")]
    PythonScript,
    #[serde(rename = "suspend")]
    Suspend,
}
impl ::std::convert::From<&Self> for TaskDefinitionType {
    fn from(value: &TaskDefinitionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TaskDefinitionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Script => write!(f, "script"),
            Self::RustScript => write!(f, "rust_script"),
            Self::PythonScript => write!(f, "python_script"),
            Self::Suspend => write!(f, "suspend"),
        }
    }
}
impl ::std::str::FromStr for TaskDefinitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "script" => Ok(Self::Script),
            "rust_script" => Ok(Self::RustScript),
            "python_script" => Ok(Self::PythonScript),
            "suspend" => Ok(Self::Suspend),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TaskDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TaskDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TaskDefinitionType {
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
    pub struct TaskDefinition {
        args: ::std::result::Result<
            ::std::collections::HashMap<super::TaskDefinitionArgsKey, ::serde_json::Value>,
            ::std::string::String,
        >,
        condition: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        depends_on:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        path: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::TaskDefinitionType, ::std::string::String>,
    }
    impl ::std::default::Default for TaskDefinition {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                condition: Ok(Default::default()),
                depends_on: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                path: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TaskDefinition {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<super::TaskDefinitionArgsKey, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn condition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.condition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for condition: {}", e));
            self
        }
        pub fn depends_on<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.depends_on = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depends_on: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TaskDefinitionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TaskDefinition> for super::TaskDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TaskDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                condition: value.condition?,
                depends_on: value.depends_on?,
                id: value.id?,
                path: value.path?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TaskDefinition> for TaskDefinition {
        fn from(value: super::TaskDefinition) -> Self {
            Self {
                args: Ok(value.args),
                condition: Ok(value.condition),
                depends_on: Ok(value.depends_on),
                id: Ok(value.id),
                path: Ok(value.path),
                type_: Ok(value.type_),
            }
        }
    }
}
