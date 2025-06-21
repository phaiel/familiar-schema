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
#[doc = "The base schema for a file that defines a single level of the universal classification taxonomy. It is a collection of taxonomy nodes."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/TaxonomyLevel.v1.schema.json\","]
#[doc = "  \"title\": \"Base Taxonomy Level\","]
#[doc = "  \"description\": \"The base schema for a file that defines a single level of the universal classification taxonomy. It is a collection of taxonomy nodes.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$id\": \"https://familiar.dev/schemas/base/TaxonomyNode.v1.schema.json\","]
#[doc = "    \"title\": \"Base Taxonomy Node\","]
#[doc = "    \"description\": \"Defines the structure for a single node within any given taxonomy.\","]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"required\": ["]
#[doc = "      \"description\","]
#[doc = "      \"id\","]
#[doc = "      \"label\","]
#[doc = "      \"level\""]
#[doc = "    ],"]
#[doc = "    \"properties\": {"]
#[doc = "      \"description\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "        \"title\": \"Description Field\","]
#[doc = "        \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "        \"default\": \"\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"embedding_context\": {"]
#[doc = "        \"description\": \"OPTIONAL: Rich text fed into a Sentence Transformer to generate the canonical embedding for this concept. Omit for rule-based classifications.\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"id\": {"]
#[doc = "        \"description\": \"The unique ID for this node within its level (e.g., 'simple', 'individual', 'sleep').\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"label\": {"]
#[doc = "        \"description\": \"The human-readable label (e.g., 'Simple', 'Individual', 'Sleep').\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"level\": {"]
#[doc = "        \"description\": \"The name of the taxonomy level this node belongs to (e.g., 'cognitive_complexity').\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"metadata\": {"]
#[doc = "        \"title\": \"BaseTaxonomyNodeMetadata\","]
#[doc = "        \"description\": \"Optional key-value metadata for this node, such as a physics profile affinity or rule-based triggers.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": true"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "  },"]
#[doc = "  \"minItems\": 1,"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"example\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"For direct logging, immediate recording, and factual capture of events.\","]
#[doc = "      \"id\": \"simple\","]
#[doc = "      \"label\": \"Simple\","]
#[doc = "      \"level\": \"cognitive_complexity\","]
#[doc = "      \"physics_profile_multiplier\": \"complexity_simple_v1\","]
#[doc = "      \"search_method_affinity\": \"GraphQL\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"For pattern recognition, correlation analysis, and trend identification.\","]
#[doc = "      \"id\": \"moderate\","]
#[doc = "      \"label\": \"Moderate\","]
#[doc = "      \"level\": \"cognitive_complexity\","]
#[doc = "      \"physics_profile_multiplier\": \"complexity_moderate_v1\","]
#[doc = "      \"search_method_affinity\": \"PgVectorScale\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"For self-reflection, introspection, and meta-cognition. High quantum coherence requirements.\","]
#[doc = "      \"id\": \"complex\","]
#[doc = "      \"label\": \"Complex\","]
#[doc = "      \"level\": \"cognitive_complexity\","]
#[doc = "      \"physics_profile_multiplier\": \"complexity_complex_v1\","]
#[doc = "      \"search_method_affinity\": \"MemoryManifold\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"source_file\": \"_base/BaseTaxonomyLevel.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct BaseTaxonomyLevel(pub ::std::vec::Vec<BaseTaxonomyNode>);
impl ::std::ops::Deref for BaseTaxonomyLevel {
    type Target = ::std::vec::Vec<BaseTaxonomyNode>;
    fn deref(&self) -> &::std::vec::Vec<BaseTaxonomyNode> {
        &self.0
    }
}
impl ::std::convert::From<BaseTaxonomyLevel> for ::std::vec::Vec<BaseTaxonomyNode> {
    fn from(value: BaseTaxonomyLevel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&BaseTaxonomyLevel> for BaseTaxonomyLevel {
    fn from(value: &BaseTaxonomyLevel) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<BaseTaxonomyNode>> for BaseTaxonomyLevel {
    fn from(value: ::std::vec::Vec<BaseTaxonomyNode>) -> Self {
        Self(value)
    }
}
#[doc = "Defines the structure for a single node within any given taxonomy."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/TaxonomyNode.v1.schema.json\","]
#[doc = "  \"title\": \"Base Taxonomy Node\","]
#[doc = "  \"description\": \"Defines the structure for a single node within any given taxonomy.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"id\","]
#[doc = "    \"label\","]
#[doc = "    \"level\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"embedding_context\": {"]
#[doc = "      \"description\": \"OPTIONAL: Rich text fed into a Sentence Transformer to generate the canonical embedding for this concept. Omit for rule-based classifications.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"The unique ID for this node within its level (e.g., 'simple', 'individual', 'sleep').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"description\": \"The human-readable label (e.g., 'Simple', 'Individual', 'Sleep').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"level\": {"]
#[doc = "      \"description\": \"The name of the taxonomy level this node belongs to (e.g., 'cognitive_complexity').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"BaseTaxonomyNodeMetadata\","]
#[doc = "      \"description\": \"Optional key-value metadata for this node, such as a physics profile affinity or rule-based triggers.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": true"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseTaxonomyNode {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub description: ::std::string::String,
    #[doc = "OPTIONAL: Rich text fed into a Sentence Transformer to generate the canonical embedding for this concept. Omit for rule-based classifications."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub embedding_context: ::std::option::Option<::std::string::String>,
    #[doc = "The unique ID for this node within its level (e.g., 'simple', 'individual', 'sleep')."]
    pub id: ::std::string::String,
    #[doc = "The human-readable label (e.g., 'Simple', 'Individual', 'Sleep')."]
    pub label: ::std::string::String,
    #[doc = "The name of the taxonomy level this node belongs to (e.g., 'cognitive_complexity')."]
    pub level: ::std::string::String,
    #[doc = "Optional key-value metadata for this node, such as a physics profile affinity or rule-based triggers."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub metadata: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
}
impl ::std::convert::From<&BaseTaxonomyNode> for BaseTaxonomyNode {
    fn from(value: &BaseTaxonomyNode) -> Self {
        value.clone()
    }
}
impl BaseTaxonomyNode {
    pub fn builder() -> builder::BaseTaxonomyNode {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseTaxonomyNode {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        embedding_context: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
        level: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseTaxonomyNode {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                embedding_context: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                label: Err("no value supplied for label".to_string()),
                level: Err("no value supplied for level".to_string()),
                metadata: Ok(Default::default()),
            }
        }
    }
    impl BaseTaxonomyNode {
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
        pub fn embedding_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.embedding_context = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for embedding_context: {}",
                    e
                )
            });
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
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseTaxonomyNode> for super::BaseTaxonomyNode {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseTaxonomyNode,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                embedding_context: value.embedding_context?,
                id: value.id?,
                label: value.label?,
                level: value.level?,
                metadata: value.metadata?,
            })
        }
    }
    impl ::std::convert::From<super::BaseTaxonomyNode> for BaseTaxonomyNode {
        fn from(value: super::BaseTaxonomyNode) -> Self {
            Self {
                description: Ok(value.description),
                embedding_context: Ok(value.embedding_context),
                id: Ok(value.id),
                label: Ok(value.label),
                level: Ok(value.level),
                metadata: Ok(value.metadata),
            }
        }
    }
}
