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
#[doc = "Defines the structure for the canonical physics_constants.yaml file, which is the single source of truth for all base physics values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/ConstantsFile.v1.schema.json\","]
#[doc = "  \"title\": \"Base Constants File\","]
#[doc = "  \"description\": \"Defines the structure for the canonical physics_constants.yaml file, which is the single source of truth for all base physics values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"classical_physics_constants\","]
#[doc = "    \"description\","]
#[doc = "    \"quantum_physics_constants\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"classical_physics_constants\": {"]
#[doc = "      \"title\": \"BaseConstantsFileClassicalPhysicsConstants\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"cognitive_enhancement_parameters\": {"]
#[doc = "      \"title\": \"BaseConstantsFileCognitiveEnhancementParameters\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"quantum_physics_constants\": {"]
#[doc = "      \"title\": \"BaseConstantsFileQuantumPhysicsConstants\","]
#[doc = "      \"type\": \"object\""]
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
#[doc = "  \"source_file\": \"_base/BaseConstantsFile.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseConstantsFile {
    pub classical_physics_constants: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub cognitive_enhancement_parameters:
        ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    pub quantum_physics_constants: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseConstantsFile> for BaseConstantsFile {
    fn from(value: &BaseConstantsFile) -> Self {
        value.clone()
    }
}
impl BaseConstantsFile {
    pub fn builder() -> builder::BaseConstantsFile {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseConstantsFile {
        classical_physics_constants: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        cognitive_enhancement_parameters: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        quantum_physics_constants: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseConstantsFile {
        fn default() -> Self {
            Self {
                classical_physics_constants: Err(
                    "no value supplied for classical_physics_constants".to_string(),
                ),
                cognitive_enhancement_parameters: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                quantum_physics_constants: Err(
                    "no value supplied for quantum_physics_constants".to_string()
                ),
                schema_version: Err("no value supplied for schema_version".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseConstantsFile {
        pub fn classical_physics_constants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.classical_physics_constants = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for classical_physics_constants: {}",
                    e
                )
            });
            self
        }
        pub fn cognitive_enhancement_parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.cognitive_enhancement_parameters = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cognitive_enhancement_parameters: {}",
                    e
                )
            });
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
        pub fn quantum_physics_constants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.quantum_physics_constants = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for quantum_physics_constants: {}",
                    e
                )
            });
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
    impl ::std::convert::TryFrom<BaseConstantsFile> for super::BaseConstantsFile {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseConstantsFile,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                classical_physics_constants: value.classical_physics_constants?,
                cognitive_enhancement_parameters: value.cognitive_enhancement_parameters?,
                description: value.description?,
                quantum_physics_constants: value.quantum_physics_constants?,
                schema_version: value.schema_version?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseConstantsFile> for BaseConstantsFile {
        fn from(value: super::BaseConstantsFile) -> Self {
            Self {
                classical_physics_constants: Ok(value.classical_physics_constants),
                cognitive_enhancement_parameters: Ok(value.cognitive_enhancement_parameters),
                description: Ok(value.description),
                quantum_physics_constants: Ok(value.quantum_physics_constants),
                schema_version: Ok(value.schema_version),
                title: Ok(value.title),
            }
        }
    }
}
