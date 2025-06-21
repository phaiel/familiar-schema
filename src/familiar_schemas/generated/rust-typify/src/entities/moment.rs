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
#[doc = "The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "  \"title\": \"Consolidation Rate Field\","]
#[doc = "  \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"default\": 0.0,"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0,"]
#[doc = "      \"x-python-type\": \"float\","]
#[doc = "      \"x-rust-type\": \"f64\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConsolidationRateField {
    pub value: f64,
}
impl ::std::convert::From<&ConsolidationRateField> for ConsolidationRateField {
    fn from(value: &ConsolidationRateField) -> Self {
        value.clone()
    }
}
impl ConsolidationRateField {
    pub fn builder() -> builder::ConsolidationRateField {
        Default::default()
    }
}
#[doc = "A canonical enum of all 7 cognitive entity types."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "  \"title\": \"Entity Type\","]
#[doc = "  \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Focus\","]
#[doc = "    \"Filament\","]
#[doc = "    \"Motif\","]
#[doc = "    \"Intent\","]
#[doc = "    \"Moment\","]
#[doc = "    \"Bond\","]
#[doc = "    \"Thread\""]
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
pub enum EntityType {
    Focus,
    Filament,
    Motif,
    Intent,
    Moment,
    Bond,
    Thread,
}
impl ::std::convert::From<&Self> for EntityType {
    fn from(value: &EntityType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Focus => write!(f, "Focus"),
            Self::Filament => write!(f, "Filament"),
            Self::Motif => write!(f, "Motif"),
            Self::Intent => write!(f, "Intent"),
            Self::Moment => write!(f, "Moment"),
            Self::Bond => write!(f, "Bond"),
            Self::Thread => write!(f, "Thread"),
        }
    }
}
impl ::std::str::FromStr for EntityType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Focus" => Ok(Self::Focus),
            "Filament" => Ok(Self::Filament),
            "Motif" => Ok(Self::Motif),
            "Intent" => Ok(Self::Intent),
            "Moment" => Ok(Self::Moment),
            "Bond" => Ok(Self::Bond),
            "Thread" => Ok(Self::Thread),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EntityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Component references temporarily disabled for pipeline testing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MomentComponents\","]
#[doc = "  \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_comment\": {"]
#[doc = "      \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/MomentContent.v1.schema.json\","]
#[doc = "      \"title\": \"Moment Content Component\","]
#[doc = "      \"description\": \"Defines the objective, factual content of a Moment entity, representing a specific event in time.\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"MomentContentFields\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"description\","]
#[doc = "            \"moment_type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"moment_type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/classification/MomentType.v1.json\","]
#[doc = "              \"title\": \"Moment Type\","]
#[doc = "              \"description\": \"A canonical enum for the classification of a Moment entity's content.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Event\","]
#[doc = "                \"Experience\","]
#[doc = "                \"Observation\","]
#[doc = "                \"Interaction\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"objective_facts\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/StringValueMap.v1.json\","]
#[doc = "              \"title\": \"String Value Map\","]
#[doc = "              \"description\": \"A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"type\": ["]
#[doc = "                  \"string\","]
#[doc = "                  \"number\","]
#[doc = "                  \"integer\","]
#[doc = "                  \"boolean\","]
#[doc = "                  \"object\","]
#[doc = "                  \"array\","]
#[doc = "                  \"null\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"participants\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "              \"title\": \"Entity ID List\","]
#[doc = "              \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"uniqueItems\": true"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.3.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MomentComponents {
    #[doc = "TODO: Re-enable component references when component schemas are available"]
    #[serde(
        rename = "_comment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comment: ::std::option::Option<::std::string::String>,
    pub content: MomentContentComponent,
}
impl ::std::convert::From<&MomentComponents> for MomentComponents {
    fn from(value: &MomentComponents) -> Self {
        value.clone()
    }
}
impl MomentComponents {
    pub fn builder() -> builder::MomentComponents {
        Default::default()
    }
}
#[doc = "Defines the objective, factual content of a Moment entity, representing a specific event in time."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/MomentContent.v1.schema.json\","]
#[doc = "  \"title\": \"Moment Content Component\","]
#[doc = "  \"description\": \"Defines the objective, factual content of a Moment entity, representing a specific event in time.\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"MomentContentFields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"description\","]
#[doc = "        \"moment_type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"description\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"moment_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/MomentType.v1.json\","]
#[doc = "          \"title\": \"Moment Type\","]
#[doc = "          \"description\": \"A canonical enum for the classification of a Moment entity's content.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Event\","]
#[doc = "            \"Experience\","]
#[doc = "            \"Observation\","]
#[doc = "            \"Interaction\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"objective_facts\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/StringValueMap.v1.json\","]
#[doc = "          \"title\": \"String Value Map\","]
#[doc = "          \"description\": \"A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": ["]
#[doc = "              \"string\","]
#[doc = "              \"number\","]
#[doc = "              \"integer\","]
#[doc = "              \"boolean\","]
#[doc = "              \"object\","]
#[doc = "              \"array\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"participants\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "          \"title\": \"Entity ID List\","]
#[doc = "          \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.3.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MomentContentComponent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<MomentContentFields>,
}
impl ::std::convert::From<&MomentContentComponent> for MomentContentComponent {
    fn from(value: &MomentContentComponent) -> Self {
        value.clone()
    }
}

impl MomentContentComponent {
    pub fn builder() -> builder::MomentContentComponent {
        Default::default()
    }
}
#[doc = "`MomentContentFields`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MomentContentFields\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"moment_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"moment_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/classification/MomentType.v1.json\","]
#[doc = "      \"title\": \"Moment Type\","]
#[doc = "      \"description\": \"A canonical enum for the classification of a Moment entity's content.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Event\","]
#[doc = "        \"Experience\","]
#[doc = "        \"Observation\","]
#[doc = "        \"Interaction\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"objective_facts\": {"]
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
#[doc = "    \"participants\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "      \"title\": \"Entity ID List\","]
#[doc = "      \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MomentContentFields {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub description: ::std::string::String,
    #[doc = "A canonical enum for the classification of a Moment entity's content."]
    pub moment_type: MomentType,
    #[doc = "A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub objective_facts: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    #[doc = "A canonical definition for a list of unique entity identifiers (UUIDs)."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub participants: ::std::option::Option<Vec<::std::string::String>>,
}
impl ::std::convert::From<&MomentContentFields> for MomentContentFields {
    fn from(value: &MomentContentFields) -> Self {
        value.clone()
    }
}
impl MomentContentFields {
    pub fn builder() -> builder::MomentContentFields {
        Default::default()
    }
}
#[doc = "A classical entity representing a specific, objective event in the past. This is the atomic unit of episodic memory."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/Moment.v1.schema.json\","]
#[doc = "  \"title\": \"Moment Entity\","]
#[doc = "  \"description\": \"A classical entity representing a specific, objective event in the past. This is the atomic unit of episodic memory.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"entity_type\","]
#[doc = "    \"physics_state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"MomentComponents\","]
#[doc = "      \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"content\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"_comment\": {"]
#[doc = "          \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"content\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/MomentContent.v1.schema.json\","]
#[doc = "          \"title\": \"Moment Content Component\","]
#[doc = "          \"description\": \"Defines the objective, factual content of a Moment entity, representing a specific event in time.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"MomentContentFields\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"description\","]
#[doc = "                \"moment_type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"description\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                  \"title\": \"Description Field\","]
#[doc = "                  \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                  \"default\": \"\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"moment_type\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/classification/MomentType.v1.json\","]
#[doc = "                  \"title\": \"Moment Type\","]
#[doc = "                  \"description\": \"A canonical enum for the classification of a Moment entity's content.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Event\","]
#[doc = "                    \"Experience\","]
#[doc = "                    \"Observation\","]
#[doc = "                    \"Interaction\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"objective_facts\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/StringValueMap.v1.json\","]
#[doc = "                  \"title\": \"String Value Map\","]
#[doc = "                  \"description\": \"A generic key-value map where keys are strings and values can be any JSON type. Used for flexible data structures like parameters or metadata.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"additionalProperties\": {"]
#[doc = "                    \"type\": ["]
#[doc = "                      \"string\","]
#[doc = "                      \"number\","]
#[doc = "                      \"integer\","]
#[doc = "                      \"boolean\","]
#[doc = "                      \"object\","]
#[doc = "                      \"array\","]
#[doc = "                      \"null\""]
#[doc = "                    ]"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"participants\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "                  \"title\": \"Entity ID List\","]
#[doc = "                  \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"uniqueItems\": true"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.3.0\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entity_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "      \"title\": \"Entity Type\","]
#[doc = "      \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Focus\","]
#[doc = "        \"Filament\","]
#[doc = "        \"Motif\","]
#[doc = "        \"Intent\","]
#[doc = "        \"Moment\","]
#[doc = "        \"Bond\","]
#[doc = "        \"Thread\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"physics_state\": {"]
#[doc = "      \"title\": \"MomentPhysicsState\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"manifold_position\","]
#[doc = "        \"universal\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"manifold_position\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/MemoryManifoldPosition.v1.schema.json\","]
#[doc = "          \"title\": \"Memory Manifold Position Component\","]
#[doc = "          \"description\": \"The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule 8).\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"autobiographical_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for External <-> Self-related.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"coherence_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for Fragmented <-> Unified.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"emotional_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for Negative <-> Positive.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"episodic_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for General <-> Specific.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"salience_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for Background <-> Foreground.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"semantic_coordinate\": {"]
#[doc = "              \"description\": \"Quantized coordinate for Abstract <-> Concrete.\","]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"temporal_coordinate\": {"]
#[doc = "              \"description\": \"Continuous time coordinate (t <= 0).\","]
#[doc = "              \"maximum\": 0,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.0.0\""]
#[doc = "        },"]
#[doc = "        \"universal\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\","]
#[doc = "          \"title\": \"Universal Physics State Component\","]
#[doc = "          \"description\": \"The transient, mutable physics state of an entity.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"cognitive_perspective\","]
#[doc = "                \"consolidation_rate\","]
#[doc = "                \"decay_rate\","]
#[doc = "                \"energy\","]
#[doc = "                \"entanglement_strength\","]
#[doc = "                \"momentum\","]
#[doc = "                \"quantum_coherence\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"cognitive_perspective\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "                  \"title\": \"Cognitive Perspective Field\","]
#[doc = "                  \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "                  \"ui_label\": \"Cognitive Perspective\""]
#[doc = "                },"]
#[doc = "                \"consolidation_rate\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "                  \"title\": \"Consolidation Rate Field\","]
#[doc = "                  \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"default\": 0.0,"]
#[doc = "                      \"type\": \"number\","]
#[doc = "                      \"format\": \"double\","]
#[doc = "                      \"maximum\": 1.0,"]
#[doc = "                      \"minimum\": 0.0,"]
#[doc = "                      \"x-python-type\": \"float\","]
#[doc = "                      \"x-rust-type\": \"f64\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                \"decay_rate\": {"]
#[doc = "                  \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"value\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"value\": {"]
#[doc = "                      \"default\": 0.01,"]
#[doc = "                      \"type\": \"number\","]
#[doc = "                      \"format\": \"double\","]
#[doc = "                      \"x-python-type\": \"float\","]
#[doc = "                      \"x-rust-type\": \"f64\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                \"energy\": {"]
#[doc = "                  \"description\": \"The current energy level of an entity.\","]
#[doc = "                  \"default\": 0.1,"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"entanglement_strength\": {"]
#[doc = "                  \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "                  \"default\": null,"]
#[doc = "                  \"type\": ["]
#[doc = "                    \"number\","]
#[doc = "                    \"null\""]
#[doc = "                  ],"]
#[doc = "                  \"maximum\": 1.0,"]
#[doc = "                  \"minimum\": 0.0"]
#[doc = "                },"]
#[doc = "                \"momentum\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "                  \"title\": \"6D Physics Vector\","]
#[doc = "                  \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  \"maxItems\": 6,"]
#[doc = "                  \"minItems\": 6"]
#[doc = "                },"]
#[doc = "                \"quantum_coherence\": {"]
#[doc = "                  \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "                  \"default\": null,"]
#[doc = "                  \"type\": ["]
#[doc = "                    \"number\","]
#[doc = "                    \"null\""]
#[doc = "                  ],"]
#[doc = "                  \"maximum\": 1.0,"]
#[doc = "                  \"minimum\": 0.0"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"schema_version\": \"3.0.0\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"source_file\": \"entities/Moment.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MomentEntity {
    pub components: MomentComponents,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    pub entity_type: EntityType,
    pub physics_state: MomentPhysicsState,
}
impl ::std::convert::From<&MomentEntity> for MomentEntity {
    fn from(value: &MomentEntity) -> Self {
        value.clone()
    }
}
impl MomentEntity {
    pub fn builder() -> builder::MomentEntity {
        Default::default()
    }
}
#[doc = "`MomentPhysicsState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MomentPhysicsState\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"manifold_position\","]
#[doc = "    \"universal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"manifold_position\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/MemoryManifoldPosition.v1.schema.json\","]
#[doc = "      \"title\": \"Memory Manifold Position Component\","]
#[doc = "      \"description\": \"The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule 8).\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"autobiographical_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for External <-> Self-related.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"coherence_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for Fragmented <-> Unified.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"emotional_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for Negative <-> Positive.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"episodic_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for General <-> Specific.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"salience_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for Background <-> Foreground.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"semantic_coordinate\": {"]
#[doc = "          \"description\": \"Quantized coordinate for Abstract <-> Concrete.\","]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"temporal_coordinate\": {"]
#[doc = "          \"description\": \"Continuous time coordinate (t <= 0).\","]
#[doc = "          \"maximum\": 0,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.0.0\""]
#[doc = "    },"]
#[doc = "    \"universal\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\","]
#[doc = "      \"title\": \"Universal Physics State Component\","]
#[doc = "      \"description\": \"The transient, mutable physics state of an entity.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"cognitive_perspective\","]
#[doc = "            \"consolidation_rate\","]
#[doc = "            \"decay_rate\","]
#[doc = "            \"energy\","]
#[doc = "            \"entanglement_strength\","]
#[doc = "            \"momentum\","]
#[doc = "            \"quantum_coherence\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"cognitive_perspective\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "              \"title\": \"Cognitive Perspective Field\","]
#[doc = "              \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "              \"ui_label\": \"Cognitive Perspective\""]
#[doc = "            },"]
#[doc = "            \"consolidation_rate\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "              \"title\": \"Consolidation Rate Field\","]
#[doc = "              \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"value\": {"]
#[doc = "                  \"default\": 0.0,"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"double\","]
#[doc = "                  \"maximum\": 1.0,"]
#[doc = "                  \"minimum\": 0.0,"]
#[doc = "                  \"x-python-type\": \"float\","]
#[doc = "                  \"x-rust-type\": \"f64\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"decay_rate\": {"]
#[doc = "              \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"value\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"value\": {"]
#[doc = "                  \"default\": 0.01,"]
#[doc = "                  \"type\": \"number\","]
#[doc = "                  \"format\": \"double\","]
#[doc = "                  \"x-python-type\": \"float\","]
#[doc = "                  \"x-rust-type\": \"f64\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"energy\": {"]
#[doc = "              \"description\": \"The current energy level of an entity.\","]
#[doc = "              \"default\": 0.1,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"entanglement_strength\": {"]
#[doc = "              \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "              \"default\": null,"]
#[doc = "              \"type\": ["]
#[doc = "                \"number\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"maximum\": 1.0,"]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            },"]
#[doc = "            \"momentum\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "              \"title\": \"6D Physics Vector\","]
#[doc = "              \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 6,"]
#[doc = "              \"minItems\": 6"]
#[doc = "            },"]
#[doc = "            \"quantum_coherence\": {"]
#[doc = "              \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "              \"default\": null,"]
#[doc = "              \"type\": ["]
#[doc = "                \"number\","]
#[doc = "                \"null\""]
#[doc = "              ],"]
#[doc = "              \"maximum\": 1.0,"]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"schema_version\": \"3.0.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MomentPhysicsState {
    #[doc = "The entity's position in the 7D cognitive manifold, enforcing quantized spatial coordinates (Rule 8)."]
    pub manifold_position: ::serde_json::Value,
    pub universal: UniversalPhysicsStateComponent,
}
impl ::std::convert::From<&MomentPhysicsState> for MomentPhysicsState {
    fn from(value: &MomentPhysicsState) -> Self {
        value.clone()
    }
}
impl MomentPhysicsState {
    pub fn builder() -> builder::MomentPhysicsState {
        Default::default()
    }
}
#[doc = "A canonical enum for the classification of a Moment entity's content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/classification/MomentType.v1.json\","]
#[doc = "  \"title\": \"Moment Type\","]
#[doc = "  \"description\": \"A canonical enum for the classification of a Moment entity's content.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Event\","]
#[doc = "    \"Experience\","]
#[doc = "    \"Observation\","]
#[doc = "    \"Interaction\""]
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
pub enum MomentType {
    Event,
    Experience,
    Observation,
    Interaction,
}
impl ::std::convert::From<&Self> for MomentType {
    fn from(value: &MomentType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MomentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Event => write!(f, "Event"),
            Self::Experience => write!(f, "Experience"),
            Self::Observation => write!(f, "Observation"),
            Self::Interaction => write!(f, "Interaction"),
        }
    }
}
impl ::std::str::FromStr for MomentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Event" => Ok(Self::Event),
            "Experience" => Ok(Self::Experience),
            "Observation" => Ok(Self::Observation),
            "Interaction" => Ok(Self::Interaction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MomentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MomentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MomentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The transient, mutable physics state of an entity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\","]
#[doc = "  \"title\": \"Universal Physics State Component\","]
#[doc = "  \"description\": \"The transient, mutable physics state of an entity.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"cognitive_perspective\","]
#[doc = "        \"consolidation_rate\","]
#[doc = "        \"decay_rate\","]
#[doc = "        \"energy\","]
#[doc = "        \"entanglement_strength\","]
#[doc = "        \"momentum\","]
#[doc = "        \"quantum_coherence\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"cognitive_perspective\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "          \"title\": \"Cognitive Perspective Field\","]
#[doc = "          \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "          \"ui_label\": \"Cognitive Perspective\""]
#[doc = "        },"]
#[doc = "        \"consolidation_rate\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "          \"title\": \"Consolidation Rate Field\","]
#[doc = "          \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"value\": {"]
#[doc = "              \"default\": 0.0,"]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"maximum\": 1.0,"]
#[doc = "              \"minimum\": 0.0,"]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"decay_rate\": {"]
#[doc = "          \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"value\": {"]
#[doc = "              \"default\": 0.01,"]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"energy\": {"]
#[doc = "          \"description\": \"The current energy level of an entity.\","]
#[doc = "          \"default\": 0.1,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"entanglement_strength\": {"]
#[doc = "          \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "          \"default\": null,"]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"momentum\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "          \"title\": \"6D Physics Vector\","]
#[doc = "          \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 6,"]
#[doc = "          \"minItems\": 6"]
#[doc = "        },"]
#[doc = "        \"quantum_coherence\": {"]
#[doc = "          \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "          \"default\": null,"]
#[doc = "          \"type\": ["]
#[doc = "            \"number\","]
#[doc = "            \"null\""]
#[doc = "          ],"]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"schema_version\": \"3.0.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UniversalPhysicsStateComponent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<UniversalPhysicsStateFields>,
}
impl ::std::convert::From<&UniversalPhysicsStateComponent> for UniversalPhysicsStateComponent {
    fn from(value: &UniversalPhysicsStateComponent) -> Self {
        value.clone()
    }
}

impl UniversalPhysicsStateComponent {
    pub fn builder() -> builder::UniversalPhysicsStateComponent {
        Default::default()
    }
}
#[doc = "`UniversalPhysicsStateFields`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cognitive_perspective\","]
#[doc = "    \"consolidation_rate\","]
#[doc = "    \"decay_rate\","]
#[doc = "    \"energy\","]
#[doc = "    \"entanglement_strength\","]
#[doc = "    \"momentum\","]
#[doc = "    \"quantum_coherence\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cognitive_perspective\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "      \"title\": \"Cognitive Perspective Field\","]
#[doc = "      \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "      \"ui_label\": \"Cognitive Perspective\""]
#[doc = "    },"]
#[doc = "    \"consolidation_rate\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "      \"title\": \"Consolidation Rate Field\","]
#[doc = "      \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"default\": 0.0,"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0,"]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"decay_rate\": {"]
#[doc = "      \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"default\": 0.01,"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"energy\": {"]
#[doc = "      \"description\": \"The current energy level of an entity.\","]
#[doc = "      \"default\": 0.1,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"entanglement_strength\": {"]
#[doc = "      \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"momentum\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "      \"title\": \"6D Physics Vector\","]
#[doc = "      \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"number\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 6,"]
#[doc = "      \"minItems\": 6"]
#[doc = "    },"]
#[doc = "    \"quantum_coherence\": {"]
#[doc = "      \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"number\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UniversalPhysicsStateFields {
    #[doc = "The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion)."]
    pub cognitive_perspective: ::serde_json::Value,
    pub consolidation_rate: ConsolidationRateField,
    pub decay_rate: UniversalPhysicsStateFieldsDecayRate,
    pub energy: f64,
    #[doc = "The overall entanglement strength of this entity with others. Null for classical entities."]
    pub entanglement_strength: ::std::option::Option<f64>,
    #[doc = "A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold."]
    pub momentum: [f64; 6usize],
    #[doc = "The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities."]
    pub quantum_coherence: ::std::option::Option<f64>,
}
impl ::std::convert::From<&UniversalPhysicsStateFields> for UniversalPhysicsStateFields {
    fn from(value: &UniversalPhysicsStateFields) -> Self {
        value.clone()
    }
}
impl UniversalPhysicsStateFields {
    pub fn builder() -> builder::UniversalPhysicsStateFields {
        Default::default()
    }
}
#[doc = "The base rate of energy or coherence decay for an entity, before multipliers are applied."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"default\": 0.01,"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"format\": \"double\","]
#[doc = "      \"x-python-type\": \"float\","]
#[doc = "      \"x-rust-type\": \"f64\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UniversalPhysicsStateFieldsDecayRate {
    pub value: f64,
}
impl ::std::convert::From<&UniversalPhysicsStateFieldsDecayRate>
    for UniversalPhysicsStateFieldsDecayRate
{
    fn from(value: &UniversalPhysicsStateFieldsDecayRate) -> Self {
        value.clone()
    }
}
impl UniversalPhysicsStateFieldsDecayRate {
    pub fn builder() -> builder::UniversalPhysicsStateFieldsDecayRate {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ConsolidationRateField {
        value: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for ConsolidationRateField {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ConsolidationRateField {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConsolidationRateField> for super::ConsolidationRateField {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConsolidationRateField,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ConsolidationRateField> for ConsolidationRateField {
        fn from(value: super::ConsolidationRateField) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentComponents {
        comment: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        content: ::std::result::Result<super::MomentContentComponent, ::std::string::String>,
    }
    impl ::std::default::Default for MomentComponents {
        fn default() -> Self {
            Self {
                comment: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
            }
        }
    }
    impl MomentComponents {
        pub fn comment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.comment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for comment: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MomentContentComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentComponents> for super::MomentComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comment: value.comment?,
                content: value.content?,
            })
        }
    }
    impl ::std::convert::From<super::MomentComponents> for MomentComponents {
        fn from(value: super::MomentComponents) -> Self {
            Self {
                comment: Ok(value.comment),
                content: Ok(value.content),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentContentComponent {
        fields: ::std::result::Result<
            ::std::option::Option<super::MomentContentFields>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MomentContentComponent {
        fn default() -> Self {
            Self {
                fields: Ok(Default::default()),
            }
        }
    }
    impl MomentContentComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MomentContentFields>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentContentComponent> for super::MomentContentComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentContentComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
            })
        }
    }
    impl ::std::convert::From<super::MomentContentComponent> for MomentContentComponent {
        fn from(value: super::MomentContentComponent) -> Self {
            Self {
                fields: Ok(value.fields),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentContentFields {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        moment_type: ::std::result::Result<super::MomentType, ::std::string::String>,
        objective_facts: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        participants: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MomentContentFields {
        fn default() -> Self {
            Self {
                description: Err("no value supplied for description".to_string()),
                moment_type: Err("no value supplied for moment_type".to_string()),
                objective_facts: Ok(Default::default()),
                participants: Ok(Default::default()),
            }
        }
    }
    impl MomentContentFields {
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
        pub fn moment_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MomentType>,
            T::Error: ::std::fmt::Display,
        {
            self.moment_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moment_type: {}", e));
            self
        }
        pub fn objective_facts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.objective_facts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for objective_facts: {}", e));
            self
        }
        pub fn participants<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.participants = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for participants: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentContentFields> for super::MomentContentFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentContentFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                moment_type: value.moment_type?,
                objective_facts: value.objective_facts?,
                participants: value.participants?,
            })
        }
    }
    impl ::std::convert::From<super::MomentContentFields> for MomentContentFields {
        fn from(value: super::MomentContentFields) -> Self {
            Self {
                description: Ok(value.description),
                moment_type: Ok(value.moment_type),
                objective_facts: Ok(value.objective_facts),
                participants: Ok(value.participants),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentEntity {
        components: ::std::result::Result<super::MomentComponents, ::std::string::String>,
        entity_type: ::std::result::Result<super::EntityType, ::std::string::String>,
        physics_state: ::std::result::Result<super::MomentPhysicsState, ::std::string::String>,
    }
    impl ::std::default::Default for MomentEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                physics_state: Err("no value supplied for physics_state".to_string()),
            }
        }
    }
    impl MomentEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MomentComponents>,
            T::Error: ::std::fmt::Display,
        {
            self.components = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for components: {}", e));
            self
        }
        pub fn entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EntityType>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity_type: {}", e));
            self
        }
        pub fn physics_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MomentPhysicsState>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for physics_state: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentEntity> for super::MomentEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                entity_type: value.entity_type?,
                physics_state: value.physics_state?,
            })
        }
    }
    impl ::std::convert::From<super::MomentEntity> for MomentEntity {
        fn from(value: super::MomentEntity) -> Self {
            Self {
                components: Ok(value.components),
                entity_type: Ok(value.entity_type),
                physics_state: Ok(value.physics_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MomentPhysicsState {
        manifold_position: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        universal:
            ::std::result::Result<super::UniversalPhysicsStateComponent, ::std::string::String>,
    }
    impl ::std::default::Default for MomentPhysicsState {
        fn default() -> Self {
            Self {
                manifold_position: Err("no value supplied for manifold_position".to_string()),
                universal: Err("no value supplied for universal".to_string()),
            }
        }
    }
    impl MomentPhysicsState {
        pub fn manifold_position<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.manifold_position = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for manifold_position: {}",
                    e
                )
            });
            self
        }
        pub fn universal<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UniversalPhysicsStateComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.universal = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for universal: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MomentPhysicsState> for super::MomentPhysicsState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MomentPhysicsState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                manifold_position: value.manifold_position?,
                universal: value.universal?,
            })
        }
    }
    impl ::std::convert::From<super::MomentPhysicsState> for MomentPhysicsState {
        fn from(value: super::MomentPhysicsState) -> Self {
            Self {
                manifold_position: Ok(value.manifold_position),
                universal: Ok(value.universal),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UniversalPhysicsStateComponent {
        fields: ::std::result::Result<
            ::std::option::Option<super::UniversalPhysicsStateFields>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for UniversalPhysicsStateComponent {
        fn default() -> Self {
            Self {
                fields: Ok(Default::default()),
            }
        }
    }
    impl UniversalPhysicsStateComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::UniversalPhysicsStateFields>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UniversalPhysicsStateComponent>
        for super::UniversalPhysicsStateComponent
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UniversalPhysicsStateComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
            })
        }
    }
    impl ::std::convert::From<super::UniversalPhysicsStateComponent>
        for UniversalPhysicsStateComponent
    {
        fn from(value: super::UniversalPhysicsStateComponent) -> Self {
            Self {
                fields: Ok(value.fields),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UniversalPhysicsStateFields {
        cognitive_perspective: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        consolidation_rate:
            ::std::result::Result<super::ConsolidationRateField, ::std::string::String>,
        decay_rate: ::std::result::Result<
            super::UniversalPhysicsStateFieldsDecayRate,
            ::std::string::String,
        >,
        energy: ::std::result::Result<f64, ::std::string::String>,
        entanglement_strength:
            ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        momentum: ::std::result::Result<[f64; 6usize], ::std::string::String>,
        quantum_coherence: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
    }
    impl ::std::default::Default for UniversalPhysicsStateFields {
        fn default() -> Self {
            Self {
                cognitive_perspective: Err(
                    "no value supplied for cognitive_perspective".to_string()
                ),
                consolidation_rate: Err("no value supplied for consolidation_rate".to_string()),
                decay_rate: Err("no value supplied for decay_rate".to_string()),
                energy: Err("no value supplied for energy".to_string()),
                entanglement_strength: Err(
                    "no value supplied for entanglement_strength".to_string()
                ),
                momentum: Err("no value supplied for momentum".to_string()),
                quantum_coherence: Err("no value supplied for quantum_coherence".to_string()),
            }
        }
    }
    impl UniversalPhysicsStateFields {
        pub fn cognitive_perspective<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.cognitive_perspective = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cognitive_perspective: {}",
                    e
                )
            });
            self
        }
        pub fn consolidation_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConsolidationRateField>,
            T::Error: ::std::fmt::Display,
        {
            self.consolidation_rate = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for consolidation_rate: {}",
                    e
                )
            });
            self
        }
        pub fn decay_rate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UniversalPhysicsStateFieldsDecayRate>,
            T::Error: ::std::fmt::Display,
        {
            self.decay_rate = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for decay_rate: {}", e));
            self
        }
        pub fn energy<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.energy = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for energy: {}", e));
            self
        }
        pub fn entanglement_strength<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.entanglement_strength = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for entanglement_strength: {}",
                    e
                )
            });
            self
        }
        pub fn momentum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<[f64; 6usize]>,
            T::Error: ::std::fmt::Display,
        {
            self.momentum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for momentum: {}", e));
            self
        }
        pub fn quantum_coherence<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.quantum_coherence = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for quantum_coherence: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<UniversalPhysicsStateFields> for super::UniversalPhysicsStateFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UniversalPhysicsStateFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cognitive_perspective: value.cognitive_perspective?,
                consolidation_rate: value.consolidation_rate?,
                decay_rate: value.decay_rate?,
                energy: value.energy?,
                entanglement_strength: value.entanglement_strength?,
                momentum: value.momentum?,
                quantum_coherence: value.quantum_coherence?,
            })
        }
    }
    impl ::std::convert::From<super::UniversalPhysicsStateFields> for UniversalPhysicsStateFields {
        fn from(value: super::UniversalPhysicsStateFields) -> Self {
            Self {
                cognitive_perspective: Ok(value.cognitive_perspective),
                consolidation_rate: Ok(value.consolidation_rate),
                decay_rate: Ok(value.decay_rate),
                energy: Ok(value.energy),
                entanglement_strength: Ok(value.entanglement_strength),
                momentum: Ok(value.momentum),
                quantum_coherence: Ok(value.quantum_coherence),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UniversalPhysicsStateFieldsDecayRate {
        value: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for UniversalPhysicsStateFieldsDecayRate {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl UniversalPhysicsStateFieldsDecayRate {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<UniversalPhysicsStateFieldsDecayRate>
        for super::UniversalPhysicsStateFieldsDecayRate
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UniversalPhysicsStateFieldsDecayRate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::UniversalPhysicsStateFieldsDecayRate>
        for UniversalPhysicsStateFieldsDecayRate
    {
        fn from(value: super::UniversalPhysicsStateFieldsDecayRate) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
}
