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
#[doc = "Defines the structure for a complete, named classification taxonomy, such as for physics profiles or emotional valence."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Taxonomy.v1.schema.json\","]
#[doc = "  \"title\": \"Base Taxonomy\","]
#[doc = "  \"description\": \"Defines the structure for a complete, named classification taxonomy, such as for physics profiles or emotional valence.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"levels\","]
#[doc = "    \"nodes\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"taxonomy_id\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"levels\": {"]
#[doc = "      \"description\": \"An ordered list of the levels in this taxonomy.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"nodes\": {"]
#[doc = "      \"description\": \"A list of all possible nodes within this taxonomy.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/base/TaxonomyNode.v1.schema.json\","]
#[doc = "        \"title\": \"Base Taxonomy Node\","]
#[doc = "        \"description\": \"Defines the structure for a single node within any given taxonomy.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"description\","]
#[doc = "          \"id\","]
#[doc = "          \"label\","]
#[doc = "          \"level\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"description\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "            \"title\": \"Description Field\","]
#[doc = "            \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "            \"default\": \"\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"embedding_context\": {"]
#[doc = "            \"description\": \"OPTIONAL: Rich text fed into a Sentence Transformer to generate the canonical embedding for this concept. Omit for rule-based classifications.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"id\": {"]
#[doc = "            \"description\": \"The unique ID for this node within its level (e.g., 'simple', 'individual', 'sleep').\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"label\": {"]
#[doc = "            \"description\": \"The human-readable label (e.g., 'Simple', 'Individual', 'Sleep').\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"level\": {"]
#[doc = "            \"description\": \"The name of the taxonomy level this node belongs to (e.g., 'cognitive_complexity').\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"metadata\": {"]
#[doc = "            \"title\": \"BaseTaxonomyNodeMetadata\","]
#[doc = "            \"description\": \"Optional key-value metadata for this node, such as a physics profile affinity or rule-based triggers.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": true"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"taxonomy_id\": {"]
#[doc = "      \"description\": \"A unique, machine-readable ID for this taxonomy (e.g., 'physics_profile_v1').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseTaxonomy.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseTaxonomy {
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "An ordered list of the levels in this taxonomy."]
    pub levels: ::std::vec::Vec<::std::string::String>,
    #[doc = "A list of all possible nodes within this taxonomy."]
    pub nodes: ::std::vec::Vec<BaseTaxonomyNode>,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "A unique, machine-readable ID for this taxonomy (e.g., 'physics_profile_v1')."]
    pub taxonomy_id: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseTaxonomy> for BaseTaxonomy {
    fn from(value: &BaseTaxonomy) -> Self {
        value.clone()
    }
}
impl BaseTaxonomy {
    pub fn builder() -> builder::BaseTaxonomy {
        Default::default()
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
    pub struct BaseTaxonomy {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        levels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        nodes:
            ::std::result::Result<::std::vec::Vec<super::BaseTaxonomyNode>, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        taxonomy_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseTaxonomy {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                levels: Err("no value supplied for levels".to_string()),
                nodes: Err("no value supplied for nodes".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                taxonomy_id: Err("no value supplied for taxonomy_id".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseTaxonomy {
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
        pub fn levels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.levels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for levels: {}", e));
            self
        }
        pub fn nodes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BaseTaxonomyNode>>,
            T::Error: ::std::fmt::Display,
        {
            self.nodes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nodes: {}", e));
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
        pub fn taxonomy_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.taxonomy_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for taxonomy_id: {}", e));
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
    impl ::std::convert::TryFrom<BaseTaxonomy> for super::BaseTaxonomy {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseTaxonomy,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                levels: value.levels?,
                nodes: value.nodes?,
                schema_version: value.schema_version?,
                taxonomy_id: value.taxonomy_id?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseTaxonomy> for BaseTaxonomy {
        fn from(value: super::BaseTaxonomy) -> Self {
            Self {
                description: Ok(value.description),
                levels: Ok(value.levels),
                nodes: Ok(value.nodes),
                schema_version: Ok(value.schema_version),
                taxonomy_id: Ok(value.taxonomy_id),
                title: Ok(value.title),
            }
        }
    }
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
