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
#[doc = "The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft entity ready for creation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/workflows/IngestionWorkflow.v1.schema.json\","]
#[doc = "  \"title\": \"Ingestion Workflow\","]
#[doc = "  \"description\": \"The canonical DAG for processing a user's Weave, performing reconciliation, and producing a Draft entity ready for creation.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"tasks\","]
#[doc = "    \"title\""]
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
#[doc = "    \"tasks\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/TaskList.v1.json\","]
#[doc = "      \"title\": \"Task List\","]
#[doc = "      \"description\": \"A list of task definitions for a Windmill workflow.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/workflow/TaskDefinition.v1.json\","]
#[doc = "        \"title\": \"Task Definition\","]
#[doc = "        \"description\": \"A canonical definition for a single task within a Windmill workflow (DAG).\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"id\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"args\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/StringValueMap.v1.json\","]
#[doc = "            \"title\": \"String Value Map\","]
#[doc = "            \"description\": \"A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"number\","]
#[doc = "                \"integer\","]
#[doc = "                \"boolean\","]
#[doc = "                \"object\","]
#[doc = "                \"array\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"condition\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"depends_on\": {"]
#[doc = "            \"type\": \"array\","]
#[doc = "            \"items\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"path\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"script\","]
#[doc = "              \"rust_script\","]
#[doc = "              \"python_script\","]
#[doc = "              \"suspend\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"workflows\","]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"workflows/IngestionWorkflow.schema.json\","]
#[doc = "  \"tasks\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Task for the Penates agent. Deconstructs a raw WeavePayload into one or more WeaveUnitPayloads.\","]
#[doc = "      \"id\": \"penates_spooling\","]
#[doc = "      \"input_schema\": \"https://familiar.dev/schemas/payloads/WeavePayload.v1.schema.json\","]
#[doc = "      \"output_schema\": \"https://familiar.dev/schemas/payloads/WeaveUnitPayload.v1.schema.json\","]
#[doc = "      \"path\": \"cognitive/penates_spooler.py\","]
#[doc = "      \"type\": \"script\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"depends_on\": ["]
#[doc = "        \"penates_spooling\""]
#[doc = "      ],"]
#[doc = "      \"description\": \"The Heddle engine. Runs the three reconciliation tasks simultaneously for a given WeaveUnit.\","]
#[doc = "      \"id\": \"parallel_reconciliation\","]
#[doc = "      \"tasks\": ["]
#[doc = "        {"]
#[doc = "          \"args\": {"]
#[doc = "            \"config_path\": \"config/reconciliation/structural_reconciliation.yaml\""]
#[doc = "          },"]
#[doc = "          \"id\": \"structural_reconciliation\","]
#[doc = "          \"path\": \"cognitive/heddle_reconciliation.py\","]
#[doc = "          \"type\": \"script\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"args\": {"]
#[doc = "            \"config_path\": \"config/reconciliation/emotional_reconciliation.yaml\""]
#[doc = "          },"]
#[doc = "          \"id\": \"emotional_reconciliation\","]
#[doc = "          \"path\": \"cognitive/heddle_reconciliation.py\","]
#[doc = "          \"type\": \"script\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"args\": {"]
#[doc = "            \"config_path\": \"config/reconciliation/identity_reconciliation.yaml\""]
#[doc = "          },"]
#[doc = "          \"id\": \"identity_reconciliation\","]
#[doc = "          \"path\": \"cognitive/heddle_reconciliation.py\","]
#[doc = "          \"type\": \"script\""]
#[doc = "        }"]
#[doc = "      ],"]
#[doc = "      \"type\": \"parallel\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"depends_on\": ["]
#[doc = "        \"parallel_reconciliation\""]
#[doc = "      ],"]
#[doc = "      \"description\": \"Task for the Decima agent. Takes a fully reconciled ShuttlePayload and produces a final DraftPayload.\","]
#[doc = "      \"id\": \"decima_beater\","]
#[doc = "      \"input_schema\": \"https://familiar.dev/schemas/payloads/ShuttlePayload.v1.schema.json\","]
#[doc = "      \"output_schema\": \"https://familiar.dev/schemas/payloads/DraftPayload.v1.schema.json\","]
#[doc = "      \"path\": \"cognitive/decima_beater.py\","]
#[doc = "      \"type\": \"script\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"depends_on\": ["]
#[doc = "        \"decima_beater\""]
#[doc = "      ],"]
#[doc = "      \"description\": \"The final step. A system (non-agent) task that takes a DraftPayload and performs the database writes to create the entity in the ECS.\","]
#[doc = "      \"id\": \"commit_to_ecs\","]
#[doc = "      \"input_schema\": \"https://familiar.dev/schemas/payloads/DraftPayload.v1.schema.json\","]
#[doc = "      \"path\": \"physics/commit_draft.rs\","]
#[doc = "      \"type\": \"script\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IngestionWorkflow {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "A list of task definitions for a Windmill workflow."]
    pub tasks: ::std::vec::Vec<TaskDefinition>,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&IngestionWorkflow> for IngestionWorkflow {
    fn from(value: &IngestionWorkflow) -> Self {
        value.clone()
    }
}
impl IngestionWorkflow {
    pub fn builder() -> builder::IngestionWorkflow {
        Default::default()
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
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\","]
#[doc = "          \"number\","]
#[doc = "          \"integer\","]
#[doc = "          \"boolean\","]
#[doc = "          \"object\","]
#[doc = "          \"array\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      }"]
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
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TaskDefinition {
    #[doc = "A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub args: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
    pub struct IngestionWorkflow {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        tasks: ::std::result::Result<::std::vec::Vec<super::TaskDefinition>, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for IngestionWorkflow {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                tasks: Err("no value supplied for tasks".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl IngestionWorkflow {
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
        pub fn tasks<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::TaskDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.tasks = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tasks: {}", e));
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
    impl ::std::convert::TryFrom<IngestionWorkflow> for super::IngestionWorkflow {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IngestionWorkflow,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                schema_version: value.schema_version?,
                tasks: value.tasks?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::IngestionWorkflow> for IngestionWorkflow {
        fn from(value: super::IngestionWorkflow) -> Self {
            Self {
                description: Ok(value.description),
                schema_version: Ok(value.schema_version),
                tasks: Ok(value.tasks),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TaskDefinition {
        args: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
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
