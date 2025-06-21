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
#[doc = "The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output contracts."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Tool.v1.schema.json\","]
#[doc = "  \"title\": \"Base Agent Tool\","]
#[doc = "  \"description\": \"The base schema for a tool that can be used by a LlamaIndex agent, defining its input and output contracts.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"input_schema\","]
#[doc = "    \"output_schema\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"title\","]
#[doc = "    \"tool_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"input_schema\": {"]
#[doc = "      \"description\": \"A reference to the JSON schema for the tool's input parameters.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"output_schema\": {"]
#[doc = "      \"description\": \"A reference to the JSON schema for the tool's return value.\","]
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
#[doc = "    \"tool_name\": {"]
#[doc = "      \"description\": \"The unique, machine-readable name of the tool.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseTool.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseAgentTool {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "A reference to the JSON schema for the tool's input parameters."]
    pub input_schema: ::std::string::String,
    #[doc = "A reference to the JSON schema for the tool's return value."]
    pub output_schema: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
    #[doc = "The unique, machine-readable name of the tool."]
    pub tool_name: ::std::string::String,
}
impl ::std::convert::From<&BaseAgentTool> for BaseAgentTool {
    fn from(value: &BaseAgentTool) -> Self {
        value.clone()
    }
}
impl BaseAgentTool {
    pub fn builder() -> builder::BaseAgentTool {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseAgentTool {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        input_schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        output_schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
        tool_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseAgentTool {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                input_schema: Err("no value supplied for input_schema".to_string()),
                output_schema: Err("no value supplied for output_schema".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                title: Err("no value supplied for title".to_string()),
                tool_name: Err("no value supplied for tool_name".to_string()),
            }
        }
    }
    impl BaseAgentTool {
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
        pub fn input_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.input_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for input_schema: {}", e));
            self
        }
        pub fn output_schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.output_schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for output_schema: {}", e));
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
        pub fn tool_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tool_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tool_name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseAgentTool> for super::BaseAgentTool {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseAgentTool,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                input_schema: value.input_schema?,
                output_schema: value.output_schema?,
                schema_version: value.schema_version?,
                title: value.title?,
                tool_name: value.tool_name?,
            })
        }
    }
    impl ::std::convert::From<super::BaseAgentTool> for BaseAgentTool {
        fn from(value: super::BaseAgentTool) -> Self {
            Self {
                description: Ok(value.description),
                input_schema: Ok(value.input_schema),
                output_schema: Ok(value.output_schema),
                schema_version: Ok(value.schema_version),
                title: Ok(value.title),
                tool_name: Ok(value.tool_name),
            }
        }
    }
}
