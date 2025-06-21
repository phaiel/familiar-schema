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
#[doc = "A structured definition for a set of physics constants."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/physics/PhysicsConstants.v1.json\","]
#[doc = "  \"title\": \"Physics Constants Definition\","]
#[doc = "  \"description\": \"A structured definition for a set of physics constants.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"decay_rate\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"max_tension\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"spring_constant\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": true,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/physics/PhysicsConstants.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PhysicsConstantsDefinition {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub decay_rate: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max_tension: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub spring_constant: ::std::option::Option<f64>,
}
impl ::std::convert::From<&PhysicsConstantsDefinition> for PhysicsConstantsDefinition {
    fn from(value: &PhysicsConstantsDefinition) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for PhysicsConstantsDefinition {
    fn default() -> Self {
        Self {
            decay_rate: Default::default(),
            max_tension: Default::default(),
            spring_constant: Default::default(),
        }
    }
}
impl PhysicsConstantsDefinition {
    pub fn builder() -> builder::PhysicsConstantsDefinition {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct PhysicsConstantsDefinition {
        decay_rate: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        max_tension: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        spring_constant: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for PhysicsConstantsDefinition {
        fn default() -> Self {
            Self {
                decay_rate: Ok(Default::default()),
                max_tension: Ok(Default::default()),
                spring_constant: Ok(Default::default()),
            }
        }
    }
    impl PhysicsConstantsDefinition {
        pub fn decay_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.decay_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for decay_rate: {}", e));
            self
        }
        pub fn max_tension<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_tension = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_tension: {}", e));
            self
        }
        pub fn spring_constant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.spring_constant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for spring_constant: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<PhysicsConstantsDefinition> for super::PhysicsConstantsDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PhysicsConstantsDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                decay_rate: value.decay_rate?,
                max_tension: value.max_tension?,
                spring_constant: value.spring_constant?,
            })
        }
    }
    impl ::std::convert::From<super::PhysicsConstantsDefinition> for PhysicsConstantsDefinition {
        fn from(value: super::PhysicsConstantsDefinition) -> Self {
            Self {
                decay_rate: Ok(value.decay_rate),
                max_tension: Ok(value.max_tension),
                spring_constant: Ok(value.spring_constant),
            }
        }
    }
}
