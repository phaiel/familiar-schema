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
#[doc = "Defines the structure for a single physics profile, which contains multipliers that modify base constants for a given classification."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/PhysicsProfile.v1.schema.json\","]
#[doc = "  \"title\": \"Base Physics Profile\","]
#[doc = "  \"description\": \"Defines the structure for a single physics profile, which contains multipliers that modify base constants for a given classification.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"multipliers\","]
#[doc = "    \"profile_id\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"multipliers\": {"]
#[doc = "      \"title\": \"BasePhysicsProfileMultipliers\","]
#[doc = "      \"description\": \"A map of constant names to their multiplier values (e.g., 'decay_rate': 0.8).\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"profile_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BasePhysicsProfile.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BasePhysicsProfile {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "A map of constant names to their multiplier values (e.g., 'decay_rate': 0.8)."]
    pub multipliers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub profile_id: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BasePhysicsProfile> for BasePhysicsProfile {
    fn from(value: &BasePhysicsProfile) -> Self {
        value.clone()
    }
}
impl BasePhysicsProfile {
    pub fn builder() -> builder::BasePhysicsProfile {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BasePhysicsProfile {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        multipliers: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        profile_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BasePhysicsProfile {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                multipliers: Err("no value supplied for multipliers".to_string()),
                profile_id: Err("no value supplied for profile_id".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BasePhysicsProfile {
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
        pub fn multipliers<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.multipliers = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for multipliers: {}", e));
            self
        }
        pub fn profile_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.profile_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profile_id: {}", e));
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
    }
    impl ::std::convert::TryFrom<BasePhysicsProfile> for super::BasePhysicsProfile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BasePhysicsProfile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                multipliers: value.multipliers?,
                profile_id: value.profile_id?,
                schema_version: value.schema_version?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BasePhysicsProfile> for BasePhysicsProfile {
        fn from(value: super::BasePhysicsProfile) -> Self {
            Self {
                description: Ok(value.description),
                multipliers: Ok(value.multipliers),
                profile_id: Ok(value.profile_id),
                schema_version: Ok(value.schema_version),
                title: Ok(value.title),
            }
        }
    }
}
