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
#[doc = "A canonical definition for field validation constraints."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "  \"title\": \"Constraint Definition\","]
#[doc = "  \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"maxLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"maximum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"minLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minimum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/validation/ConstraintDefinition.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstraintDefinition {
    #[serde(
        rename = "enum",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "maxLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<f64>,
    #[serde(
        rename = "minLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pattern: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ConstraintDefinition> for ConstraintDefinition {
    fn from(value: &ConstraintDefinition) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConstraintDefinition {
    fn default() -> Self {
        Self {
            enum_: Default::default(),
            max_length: Default::default(),
            maximum: Default::default(),
            min_length: Default::default(),
            minimum: Default::default(),
            pattern: Default::default(),
        }
    }
}
impl ConstraintDefinition {
    pub fn builder() -> builder::ConstraintDefinition {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ConstraintDefinition {
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        max_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        maximum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        minimum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        pattern: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstraintDefinition {
        fn default() -> Self {
            Self {
                enum_: Ok(Default::default()),
                max_length: Ok(Default::default()),
                maximum: Ok(Default::default()),
                min_length: Ok(Default::default()),
                minimum: Ok(Default::default()),
                pattern: Ok(Default::default()),
            }
        }
    }
    impl ConstraintDefinition {
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {}", e));
            self
        }
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum: {}", e));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {}", e));
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minimum: {}", e));
            self
        }
        pub fn pattern<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pattern: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstraintDefinition> for super::ConstraintDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstraintDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enum_: value.enum_?,
                max_length: value.max_length?,
                maximum: value.maximum?,
                min_length: value.min_length?,
                minimum: value.minimum?,
                pattern: value.pattern?,
            })
        }
    }
    impl ::std::convert::From<super::ConstraintDefinition> for ConstraintDefinition {
        fn from(value: super::ConstraintDefinition) -> Self {
            Self {
                enum_: Ok(value.enum_),
                max_length: Ok(value.max_length),
                maximum: Ok(value.maximum),
                min_length: Ok(value.min_length),
                minimum: Ok(value.minimum),
                pattern: Ok(value.pattern),
            }
        }
    }
}
