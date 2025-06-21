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
#[doc = "Defines the structured output an agent must produce. Enforces Rule 3 by separating deterministic metadata from human-readable narrative."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/AgentOutput.v1.schema.json\","]
#[doc = "  \"title\": \"Base Agent Output\","]
#[doc = "  \"description\": \"Defines the structured output an agent must produce. Enforces Rule 3 by separating deterministic metadata from human-readable narrative.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"confidence_score\","]
#[doc = "    \"metadata_for_physics\","]
#[doc = "    \"narrative_for_user\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"citations\": {"]
#[doc = "      \"description\": \"A list of memory and knowledge sources used to generate the response.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"title\": \"BaseagentoutputmetadataforphysicsItems\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"source_id\","]
#[doc = "          \"source_type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"source_id\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"source_type\": {"]
#[doc = "            \"enum\": ["]
#[doc = "              \"memory\","]
#[doc = "              \"knowledge_base\","]
#[doc = "              \"research_paper\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"confidence_score\": {"]
#[doc = "      \"description\": \"The agent's confidence in its analysis, used for HITL escalation.\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"metadata_for_physics\": {"]
#[doc = "      \"title\": \"BaseAgentOutputMetadataForPhysics\","]
#[doc = "      \"description\": \"The structured, strongly-typed data that will be used deterministically by the physics engine. This data MUST NOT contain any free-form text from the LLM.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"narrative_for_user\": {"]
#[doc = "      \"description\": \"The human-readable explanation of the agent's reasoning. This field MUST NOT be used for any physics calculations.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseAgentOutput.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseAgentOutput {
    #[doc = "A list of memory and knowledge sources used to generate the response."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub citations: ::std::vec::Vec<BaseagentoutputmetadataforphysicsItems>,
    pub confidence_score: f64,
    pub metadata_for_physics: BaseAgentOutputMetadataForPhysics,
    #[doc = "The human-readable explanation of the agent's reasoning. This field MUST NOT be used for any physics calculations."]
    pub narrative_for_user: ::std::string::String,
}
impl ::std::convert::From<&BaseAgentOutput> for BaseAgentOutput {
    fn from(value: &BaseAgentOutput) -> Self {
        value.clone()
    }
}
impl BaseAgentOutput {
    pub fn builder() -> builder::BaseAgentOutput {
        Default::default()
    }
}
#[doc = "The structured, strongly-typed data that will be used deterministically by the physics engine. This data MUST NOT contain any free-form text from the LLM."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BaseAgentOutputMetadataForPhysics\","]
#[doc = "  \"description\": \"The structured, strongly-typed data that will be used deterministically by the physics engine. This data MUST NOT contain any free-form text from the LLM.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BaseAgentOutputMetadataForPhysics {}
impl ::std::convert::From<&BaseAgentOutputMetadataForPhysics>
    for BaseAgentOutputMetadataForPhysics
{
    fn from(value: &BaseAgentOutputMetadataForPhysics) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BaseAgentOutputMetadataForPhysics {
    fn default() -> Self {
        Self {}
    }
}
impl BaseAgentOutputMetadataForPhysics {
    pub fn builder() -> builder::BaseAgentOutputMetadataForPhysics {
        Default::default()
    }
}
#[doc = "`BaseagentoutputmetadataforphysicsItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BaseagentoutputmetadataforphysicsItems\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"source_id\","]
#[doc = "    \"source_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"source_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"source_type\": {"]
#[doc = "      \"enum\": ["]
#[doc = "        \"memory\","]
#[doc = "        \"knowledge_base\","]
#[doc = "        \"research_paper\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseagentoutputmetadataforphysicsItems {
    pub source_id: ::std::string::String,
    pub source_type: BaseagentoutputmetadataforphysicsItemsSourceType,
}
impl ::std::convert::From<&BaseagentoutputmetadataforphysicsItems>
    for BaseagentoutputmetadataforphysicsItems
{
    fn from(value: &BaseagentoutputmetadataforphysicsItems) -> Self {
        value.clone()
    }
}
impl BaseagentoutputmetadataforphysicsItems {
    pub fn builder() -> builder::BaseagentoutputmetadataforphysicsItems {
        Default::default()
    }
}
#[doc = "`BaseagentoutputmetadataforphysicsItemsSourceType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    \"memory\","]
#[doc = "    \"knowledge_base\","]
#[doc = "    \"research_paper\""]
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
pub enum BaseagentoutputmetadataforphysicsItemsSourceType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "knowledge_base")]
    KnowledgeBase,
    #[serde(rename = "research_paper")]
    ResearchPaper,
}
impl ::std::convert::From<&Self> for BaseagentoutputmetadataforphysicsItemsSourceType {
    fn from(value: &BaseagentoutputmetadataforphysicsItemsSourceType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BaseagentoutputmetadataforphysicsItemsSourceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Memory => write!(f, "memory"),
            Self::KnowledgeBase => write!(f, "knowledge_base"),
            Self::ResearchPaper => write!(f, "research_paper"),
        }
    }
}
impl ::std::str::FromStr for BaseagentoutputmetadataforphysicsItemsSourceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "memory" => Ok(Self::Memory),
            "knowledge_base" => Ok(Self::KnowledgeBase),
            "research_paper" => Ok(Self::ResearchPaper),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BaseagentoutputmetadataforphysicsItemsSourceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
    for BaseagentoutputmetadataforphysicsItemsSourceType
{
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
    for BaseagentoutputmetadataforphysicsItemsSourceType
{
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
    pub struct BaseAgentOutput {
        citations: ::std::result::Result<
            ::std::vec::Vec<super::BaseagentoutputmetadataforphysicsItems>,
            ::std::string::String,
        >,
        confidence_score: ::std::result::Result<f64, ::std::string::String>,
        metadata_for_physics:
            ::std::result::Result<super::BaseAgentOutputMetadataForPhysics, ::std::string::String>,
        narrative_for_user: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseAgentOutput {
        fn default() -> Self {
            Self {
                citations: Ok(Default::default()),
                confidence_score: Err("no value supplied for confidence_score".to_string()),
                metadata_for_physics: Err("no value supplied for metadata_for_physics".to_string()),
                narrative_for_user: Err("no value supplied for narrative_for_user".to_string()),
            }
        }
    }
    impl BaseAgentOutput {
        pub fn citations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::BaseagentoutputmetadataforphysicsItems>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.citations = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for citations: {}", e));
            self
        }
        pub fn confidence_score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.confidence_score = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for confidence_score: {}",
                    e
                )
            });
            self
        }
        pub fn metadata_for_physics<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseAgentOutputMetadataForPhysics>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata_for_physics = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for metadata_for_physics: {}",
                    e
                )
            });
            self
        }
        pub fn narrative_for_user<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.narrative_for_user = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for narrative_for_user: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<BaseAgentOutput> for super::BaseAgentOutput {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseAgentOutput,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                citations: value.citations?,
                confidence_score: value.confidence_score?,
                metadata_for_physics: value.metadata_for_physics?,
                narrative_for_user: value.narrative_for_user?,
            })
        }
    }
    impl ::std::convert::From<super::BaseAgentOutput> for BaseAgentOutput {
        fn from(value: super::BaseAgentOutput) -> Self {
            Self {
                citations: Ok(value.citations),
                confidence_score: Ok(value.confidence_score),
                metadata_for_physics: Ok(value.metadata_for_physics),
                narrative_for_user: Ok(value.narrative_for_user),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BaseAgentOutputMetadataForPhysics {}
    impl ::std::default::Default for BaseAgentOutputMetadataForPhysics {
        fn default() -> Self {
            Self {}
        }
    }
    impl BaseAgentOutputMetadataForPhysics {}
    impl ::std::convert::TryFrom<BaseAgentOutputMetadataForPhysics>
        for super::BaseAgentOutputMetadataForPhysics
    {
        type Error = super::error::ConversionError;
        fn try_from(
            _value: BaseAgentOutputMetadataForPhysics,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {})
        }
    }
    impl ::std::convert::From<super::BaseAgentOutputMetadataForPhysics>
        for BaseAgentOutputMetadataForPhysics
    {
        fn from(_value: super::BaseAgentOutputMetadataForPhysics) -> Self {
            Self {}
        }
    }
    #[derive(Clone, Debug)]
    pub struct BaseagentoutputmetadataforphysicsItems {
        source_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        source_type: ::std::result::Result<
            super::BaseagentoutputmetadataforphysicsItemsSourceType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseagentoutputmetadataforphysicsItems {
        fn default() -> Self {
            Self {
                source_id: Err("no value supplied for source_id".to_string()),
                source_type: Err("no value supplied for source_type".to_string()),
            }
        }
    }
    impl BaseagentoutputmetadataforphysicsItems {
        pub fn source_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_id: {}", e));
            self
        }
        pub fn source_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseagentoutputmetadataforphysicsItemsSourceType>,
            T::Error: ::std::fmt::Display,
        {
            self.source_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseagentoutputmetadataforphysicsItems>
        for super::BaseagentoutputmetadataforphysicsItems
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseagentoutputmetadataforphysicsItems,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                source_id: value.source_id?,
                source_type: value.source_type?,
            })
        }
    }
    impl ::std::convert::From<super::BaseagentoutputmetadataforphysicsItems>
        for BaseagentoutputmetadataforphysicsItems
    {
        fn from(value: super::BaseagentoutputmetadataforphysicsItems) -> Self {
            Self {
                source_id: Ok(value.source_id),
                source_type: Ok(value.source_type),
            }
        }
    }
}
