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
#[doc = "The base schema for defining an agent persona, including its role, system prompt, and associated tools."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/AgentPersona.v1.schema.json\","]
#[doc = "  \"title\": \"Base Agent Persona\","]
#[doc = "  \"description\": \"The base schema for defining an agent persona, including its role, system prompt, and associated tools.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"role\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"system_prompt\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowed_tools\": {"]
#[doc = "      \"description\": \"A list of tool schema IDs that this persona is permitted to use.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"description\": \"A short, machine-readable description of the agent's role (e.g., 'structural_resolver', 'emotional_analyst').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"system_prompt\": {"]
#[doc = "      \"description\": \"The detailed system prompt that instructs the LLM on how to behave when adopting this persona.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseAgentPersona.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseAgentPersona {
    #[doc = "A list of tool schema IDs that this persona is permitted to use."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub allowed_tools: ::std::vec::Vec<::std::string::String>,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "A short, machine-readable description of the agent's role (e.g., 'structural_resolver', 'emotional_analyst')."]
    pub role: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The detailed system prompt that instructs the LLM on how to behave when adopting this persona."]
    pub system_prompt: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseAgentPersona> for BaseAgentPersona {
    fn from(value: &BaseAgentPersona) -> Self {
        value.clone()
    }
}
impl BaseAgentPersona {
    pub fn builder() -> builder::BaseAgentPersona {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseAgentPersona {
        allowed_tools:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        system_prompt: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseAgentPersona {
        fn default() -> Self {
            Self {
                allowed_tools: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                role: Err("no value supplied for role".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                system_prompt: Err("no value supplied for system_prompt".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseAgentPersona {
        pub fn allowed_tools<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed_tools = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed_tools: {}", e));
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
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
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
        pub fn system_prompt<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.system_prompt = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system_prompt: {}", e));
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
    impl ::std::convert::TryFrom<BaseAgentPersona> for super::BaseAgentPersona {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseAgentPersona,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed_tools: value.allowed_tools?,
                description: value.description?,
                role: value.role?,
                schema_version: value.schema_version?,
                system_prompt: value.system_prompt?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseAgentPersona> for BaseAgentPersona {
        fn from(value: super::BaseAgentPersona) -> Self {
            Self {
                allowed_tools: Ok(value.allowed_tools),
                description: Ok(value.description),
                role: Ok(value.role),
                schema_version: Ok(value.schema_version),
                system_prompt: Ok(value.system_prompt),
                title: Ok(value.title),
            }
        }
    }
}
