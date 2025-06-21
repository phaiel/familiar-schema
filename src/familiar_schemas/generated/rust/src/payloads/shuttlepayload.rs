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
#[doc = "`BaseUserContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BaseUserContext\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"user_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"session_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseUserContext {
    #[doc = "A reusable definition for a unique entity identifier."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub user_id: ::std::string::String,
}
impl ::std::convert::From<&BaseUserContext> for BaseUserContext {
    fn from(value: &BaseUserContext) -> Self {
        value.clone()
    }
}
impl BaseUserContext {
    pub fn builder() -> builder::BaseUserContext {
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
#[doc = "`IngestionShuttleData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"IngestionShuttleData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"active_weave_unit\","]
#[doc = "    \"parent_course_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"active_weave_unit\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/payloads/WeaveUnitPayload.v1.schema.json\","]
#[doc = "      \"title\": \"Weave Unit Payload\","]
#[doc = "      \"description\": \"A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"data\","]
#[doc = "        \"user_context\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"data\": {"]
#[doc = "          \"title\": \"WeaveUnitData\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"parent_weave_id\","]
#[doc = "            \"text\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"parent_weave_id\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "              \"title\": \"Entity ID Field\","]
#[doc = "              \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"suggested_entity_type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "              \"title\": \"Entity Type\","]
#[doc = "              \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Focus\","]
#[doc = "                \"Filament\","]
#[doc = "                \"Motif\","]
#[doc = "                \"Intent\","]
#[doc = "                \"Moment\","]
#[doc = "                \"Bond\","]
#[doc = "                \"Thread\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"text\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"user_context\": {"]
#[doc = "          \"title\": \"BaseUserContext\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"user_id\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"session_id\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "              \"title\": \"Entity ID Field\","]
#[doc = "              \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"user_id\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "              \"title\": \"UUID\","]
#[doc = "              \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"schema_version\": \"1.1.0\""]
#[doc = "    },"]
#[doc = "    \"is_ready_for_creation\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"parent_course_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"reconciliation_results\": {"]
#[doc = "      \"title\": \"IngestionshuttledataReconciliationResults\","]
#[doc = "      \"description\": \"A map of reconciliation task types to their results. This is extensible for new reconciliation steps.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/payloads/ReconciliationResultPayload.v1.schema.json\","]
#[doc = "        \"title\": \"Reconciliation Result Payload\","]
#[doc = "        \"description\": \"The structured output from a single Heddle reconciliation task.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"data\","]
#[doc = "          \"user_context\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"data\": {"]
#[doc = "            \"title\": \"ReconciliationResultData\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"confidence_score\","]
#[doc = "              \"selected_candidate_id\","]
#[doc = "              \"task_type\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"agent_reasoning\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"confidence_score\": {"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"selected_candidate_id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"task_type\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "                \"title\": \"Reconciliation Task Type\","]
#[doc = "                \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "                \"type\": \"string\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"Structural\","]
#[doc = "                  \"Emotional\","]
#[doc = "                  \"Identity\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"user_context\": {"]
#[doc = "            \"title\": \"BaseUserContext\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"user_id\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"session_id\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "                \"title\": \"Entity ID Field\","]
#[doc = "                \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"user_id\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "                \"title\": \"UUID\","]
#[doc = "                \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "        \"schema_version\": \"1.1.0\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IngestionShuttleData {
    pub active_weave_unit: WeaveUnitPayload,
    #[serde(default)]
    pub is_ready_for_creation: bool,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub parent_course_id: ::std::string::String,
    #[doc = "A map of reconciliation task types to their results. This is extensible for new reconciliation steps."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub reconciliation_results:
        ::std::collections::HashMap<::std::string::String, ReconciliationResultPayload>,
}
impl ::std::convert::From<&IngestionShuttleData> for IngestionShuttleData {
    fn from(value: &IngestionShuttleData) -> Self {
        value.clone()
    }
}
impl IngestionShuttleData {
    pub fn builder() -> builder::IngestionShuttleData {
        Default::default()
    }
}
#[doc = "The state object that carries a WeaveUnit through the Heddle reconciliation pipeline."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/ShuttlePayload.v1.schema.json\","]
#[doc = "  \"title\": \"Ingestion Shuttle Payload\","]
#[doc = "  \"description\": \"The state object that carries a WeaveUnit through the Heddle reconciliation pipeline.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"IngestionShuttleData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"active_weave_unit\","]
#[doc = "        \"parent_course_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"active_weave_unit\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/payloads/WeaveUnitPayload.v1.schema.json\","]
#[doc = "          \"title\": \"Weave Unit Payload\","]
#[doc = "          \"description\": \"A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"data\","]
#[doc = "            \"user_context\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"data\": {"]
#[doc = "              \"title\": \"WeaveUnitData\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"parent_weave_id\","]
#[doc = "                \"text\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"parent_weave_id\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "                  \"title\": \"Entity ID Field\","]
#[doc = "                  \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"suggested_entity_type\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "                  \"title\": \"Entity Type\","]
#[doc = "                  \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Focus\","]
#[doc = "                    \"Filament\","]
#[doc = "                    \"Motif\","]
#[doc = "                    \"Intent\","]
#[doc = "                    \"Moment\","]
#[doc = "                    \"Bond\","]
#[doc = "                    \"Thread\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"text\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"user_context\": {"]
#[doc = "              \"title\": \"BaseUserContext\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"user_id\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"session_id\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "                  \"title\": \"Entity ID Field\","]
#[doc = "                  \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"user_id\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "                  \"title\": \"UUID\","]
#[doc = "                  \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"schema_version\": \"1.1.0\""]
#[doc = "        },"]
#[doc = "        \"is_ready_for_creation\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"parent_course_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"reconciliation_results\": {"]
#[doc = "          \"title\": \"IngestionshuttledataReconciliationResults\","]
#[doc = "          \"description\": \"A map of reconciliation task types to their results. This is extensible for new reconciliation steps.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/payloads/ReconciliationResultPayload.v1.schema.json\","]
#[doc = "            \"title\": \"Reconciliation Result Payload\","]
#[doc = "            \"description\": \"The structured output from a single Heddle reconciliation task.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"data\","]
#[doc = "              \"user_context\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"data\": {"]
#[doc = "                \"title\": \"ReconciliationResultData\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"confidence_score\","]
#[doc = "                  \"selected_candidate_id\","]
#[doc = "                  \"task_type\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"agent_reasoning\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"confidence_score\": {"]
#[doc = "                    \"type\": \"number\""]
#[doc = "                  },"]
#[doc = "                  \"selected_candidate_id\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"task_type\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "                    \"title\": \"Reconciliation Task Type\","]
#[doc = "                    \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "                    \"type\": \"string\","]
#[doc = "                    \"enum\": ["]
#[doc = "                      \"Structural\","]
#[doc = "                      \"Emotional\","]
#[doc = "                      \"Identity\""]
#[doc = "                    ]"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"user_context\": {"]
#[doc = "                \"title\": \"BaseUserContext\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"user_id\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"session_id\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "                    \"title\": \"Entity ID Field\","]
#[doc = "                    \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"user_id\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "                    \"title\": \"UUID\","]
#[doc = "                    \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "            \"schema_version\": \"1.1.0\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"user_context\": {"]
#[doc = "      \"title\": \"BaseUserContext\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"user_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"session_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"payloads\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"payloads/ShuttlePayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IngestionShuttlePayload {
    pub data: IngestionShuttleData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&IngestionShuttlePayload> for IngestionShuttlePayload {
    fn from(value: &IngestionShuttlePayload) -> Self {
        value.clone()
    }
}
impl IngestionShuttlePayload {
    pub fn builder() -> builder::IngestionShuttlePayload {
        Default::default()
    }
}
#[doc = "`ReconciliationResultData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ReconciliationResultData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"confidence_score\","]
#[doc = "    \"selected_candidate_id\","]
#[doc = "    \"task_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent_reasoning\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"confidence_score\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"selected_candidate_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"task_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "      \"title\": \"Reconciliation Task Type\","]
#[doc = "      \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Structural\","]
#[doc = "        \"Emotional\","]
#[doc = "        \"Identity\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReconciliationResultData {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub agent_reasoning: ::std::option::Option<::std::string::String>,
    pub confidence_score: f64,
    pub selected_candidate_id: ::std::string::String,
    #[doc = "A canonical enum for the types of reconciliation tasks performed by the Heddle engine."]
    pub task_type: ReconciliationTaskType,
}
impl ::std::convert::From<&ReconciliationResultData> for ReconciliationResultData {
    fn from(value: &ReconciliationResultData) -> Self {
        value.clone()
    }
}
impl ReconciliationResultData {
    pub fn builder() -> builder::ReconciliationResultData {
        Default::default()
    }
}
#[doc = "The structured output from a single Heddle reconciliation task."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/ReconciliationResultPayload.v1.schema.json\","]
#[doc = "  \"title\": \"Reconciliation Result Payload\","]
#[doc = "  \"description\": \"The structured output from a single Heddle reconciliation task.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"ReconciliationResultData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"confidence_score\","]
#[doc = "        \"selected_candidate_id\","]
#[doc = "        \"task_type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent_reasoning\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"confidence_score\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"selected_candidate_id\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"task_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "          \"title\": \"Reconciliation Task Type\","]
#[doc = "          \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Structural\","]
#[doc = "            \"Emotional\","]
#[doc = "            \"Identity\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"user_context\": {"]
#[doc = "      \"title\": \"BaseUserContext\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"user_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"session_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"schema_version\": \"1.1.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ReconciliationResultPayload {
    pub data: ReconciliationResultData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&ReconciliationResultPayload> for ReconciliationResultPayload {
    fn from(value: &ReconciliationResultPayload) -> Self {
        value.clone()
    }
}
impl ReconciliationResultPayload {
    pub fn builder() -> builder::ReconciliationResultPayload {
        Default::default()
    }
}
#[doc = "A canonical enum for the types of reconciliation tasks performed by the Heddle engine."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/classification/ReconciliationTaskType.v1.json\","]
#[doc = "  \"title\": \"Reconciliation Task Type\","]
#[doc = "  \"description\": \"A canonical enum for the types of reconciliation tasks performed by the Heddle engine.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Structural\","]
#[doc = "    \"Emotional\","]
#[doc = "    \"Identity\""]
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
pub enum ReconciliationTaskType {
    Structural,
    Emotional,
    Identity,
}
impl ::std::convert::From<&Self> for ReconciliationTaskType {
    fn from(value: &ReconciliationTaskType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ReconciliationTaskType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Structural => write!(f, "Structural"),
            Self::Emotional => write!(f, "Emotional"),
            Self::Identity => write!(f, "Identity"),
        }
    }
}
impl ::std::str::FromStr for ReconciliationTaskType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Structural" => Ok(Self::Structural),
            "Emotional" => Ok(Self::Emotional),
            "Identity" => Ok(Self::Identity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReconciliationTaskType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReconciliationTaskType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReconciliationTaskType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`WeaveUnitData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WeaveUnitData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"parent_weave_id\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"parent_weave_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"suggested_entity_type\": {"]
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
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeaveUnitData {
    #[doc = "A reusable definition for a unique entity identifier."]
    pub parent_weave_id: ::std::string::String,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub suggested_entity_type: ::std::option::Option<EntityType>,
    pub text: ::std::string::String,
}
impl ::std::convert::From<&WeaveUnitData> for WeaveUnitData {
    fn from(value: &WeaveUnitData) -> Self {
        value.clone()
    }
}
impl WeaveUnitData {
    pub fn builder() -> builder::WeaveUnitData {
        Default::default()
    }
}
#[doc = "A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/WeaveUnitPayload.v1.schema.json\","]
#[doc = "  \"title\": \"Weave Unit Payload\","]
#[doc = "  \"description\": \"A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"WeaveUnitData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"parent_weave_id\","]
#[doc = "        \"text\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"parent_weave_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"suggested_entity_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "          \"title\": \"Entity Type\","]
#[doc = "          \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Focus\","]
#[doc = "            \"Filament\","]
#[doc = "            \"Motif\","]
#[doc = "            \"Intent\","]
#[doc = "            \"Moment\","]
#[doc = "            \"Bond\","]
#[doc = "            \"Thread\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"user_context\": {"]
#[doc = "      \"title\": \"BaseUserContext\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"user_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"session_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"schema_version\": \"1.1.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeaveUnitPayload {
    pub data: WeaveUnitData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&WeaveUnitPayload> for WeaveUnitPayload {
    fn from(value: &WeaveUnitPayload) -> Self {
        value.clone()
    }
}
impl WeaveUnitPayload {
    pub fn builder() -> builder::WeaveUnitPayload {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseUserContext {
        session_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseUserContext {
        fn default() -> Self {
            Self {
                session_id: Ok(Default::default()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl BaseUserContext {
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseUserContext> for super::BaseUserContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseUserContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::BaseUserContext> for BaseUserContext {
        fn from(value: super::BaseUserContext) -> Self {
            Self {
                session_id: Ok(value.session_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IngestionShuttleData {
        active_weave_unit: ::std::result::Result<super::WeaveUnitPayload, ::std::string::String>,
        is_ready_for_creation: ::std::result::Result<bool, ::std::string::String>,
        parent_course_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        reconciliation_results: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::ReconciliationResultPayload>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for IngestionShuttleData {
        fn default() -> Self {
            Self {
                active_weave_unit: Err("no value supplied for active_weave_unit".to_string()),
                is_ready_for_creation: Ok(Default::default()),
                parent_course_id: Err("no value supplied for parent_course_id".to_string()),
                reconciliation_results: Ok(Default::default()),
            }
        }
    }
    impl IngestionShuttleData {
        pub fn active_weave_unit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeaveUnitPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.active_weave_unit = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for active_weave_unit: {}",
                    e
                )
            });
            self
        }
        pub fn is_ready_for_creation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_ready_for_creation = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for is_ready_for_creation: {}",
                    e
                )
            });
            self
        }
        pub fn parent_course_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_course_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for parent_course_id: {}",
                    e
                )
            });
            self
        }
        pub fn reconciliation_results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    super::ReconciliationResultPayload,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.reconciliation_results = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for reconciliation_results: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<IngestionShuttleData> for super::IngestionShuttleData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IngestionShuttleData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                active_weave_unit: value.active_weave_unit?,
                is_ready_for_creation: value.is_ready_for_creation?,
                parent_course_id: value.parent_course_id?,
                reconciliation_results: value.reconciliation_results?,
            })
        }
    }
    impl ::std::convert::From<super::IngestionShuttleData> for IngestionShuttleData {
        fn from(value: super::IngestionShuttleData) -> Self {
            Self {
                active_weave_unit: Ok(value.active_weave_unit),
                is_ready_for_creation: Ok(value.is_ready_for_creation),
                parent_course_id: Ok(value.parent_course_id),
                reconciliation_results: Ok(value.reconciliation_results),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IngestionShuttlePayload {
        data: ::std::result::Result<super::IngestionShuttleData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for IngestionShuttlePayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl IngestionShuttlePayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IngestionShuttleData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn user_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseUserContext>,
            T::Error: ::std::fmt::Display,
        {
            self.user_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_context: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<IngestionShuttlePayload> for super::IngestionShuttlePayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IngestionShuttlePayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::IngestionShuttlePayload> for IngestionShuttlePayload {
        fn from(value: super::IngestionShuttlePayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReconciliationResultData {
        agent_reasoning: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        confidence_score: ::std::result::Result<f64, ::std::string::String>,
        selected_candidate_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        task_type: ::std::result::Result<super::ReconciliationTaskType, ::std::string::String>,
    }
    impl ::std::default::Default for ReconciliationResultData {
        fn default() -> Self {
            Self {
                agent_reasoning: Ok(Default::default()),
                confidence_score: Err("no value supplied for confidence_score".to_string()),
                selected_candidate_id: Err(
                    "no value supplied for selected_candidate_id".to_string()
                ),
                task_type: Err("no value supplied for task_type".to_string()),
            }
        }
    }
    impl ReconciliationResultData {
        pub fn agent_reasoning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_reasoning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_reasoning: {}", e));
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
        pub fn selected_candidate_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.selected_candidate_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for selected_candidate_id: {}",
                    e
                )
            });
            self
        }
        pub fn task_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReconciliationTaskType>,
            T::Error: ::std::fmt::Display,
        {
            self.task_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for task_type: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ReconciliationResultData> for super::ReconciliationResultData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReconciliationResultData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_reasoning: value.agent_reasoning?,
                confidence_score: value.confidence_score?,
                selected_candidate_id: value.selected_candidate_id?,
                task_type: value.task_type?,
            })
        }
    }
    impl ::std::convert::From<super::ReconciliationResultData> for ReconciliationResultData {
        fn from(value: super::ReconciliationResultData) -> Self {
            Self {
                agent_reasoning: Ok(value.agent_reasoning),
                confidence_score: Ok(value.confidence_score),
                selected_candidate_id: Ok(value.selected_candidate_id),
                task_type: Ok(value.task_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReconciliationResultPayload {
        data: ::std::result::Result<super::ReconciliationResultData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for ReconciliationResultPayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl ReconciliationResultPayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReconciliationResultData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn user_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseUserContext>,
            T::Error: ::std::fmt::Display,
        {
            self.user_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_context: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ReconciliationResultPayload> for super::ReconciliationResultPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReconciliationResultPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::ReconciliationResultPayload> for ReconciliationResultPayload {
        fn from(value: super::ReconciliationResultPayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeaveUnitData {
        parent_weave_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        suggested_entity_type:
            ::std::result::Result<::std::option::Option<super::EntityType>, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WeaveUnitData {
        fn default() -> Self {
            Self {
                parent_weave_id: Err("no value supplied for parent_weave_id".to_string()),
                suggested_entity_type: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl WeaveUnitData {
        pub fn parent_weave_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_weave_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_weave_id: {}", e));
            self
        }
        pub fn suggested_entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EntityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.suggested_entity_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for suggested_entity_type: {}",
                    e
                )
            });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeaveUnitData> for super::WeaveUnitData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeaveUnitData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                parent_weave_id: value.parent_weave_id?,
                suggested_entity_type: value.suggested_entity_type?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::WeaveUnitData> for WeaveUnitData {
        fn from(value: super::WeaveUnitData) -> Self {
            Self {
                parent_weave_id: Ok(value.parent_weave_id),
                suggested_entity_type: Ok(value.suggested_entity_type),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeaveUnitPayload {
        data: ::std::result::Result<super::WeaveUnitData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for WeaveUnitPayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl WeaveUnitPayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeaveUnitData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn user_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseUserContext>,
            T::Error: ::std::fmt::Display,
        {
            self.user_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_context: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeaveUnitPayload> for super::WeaveUnitPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeaveUnitPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::WeaveUnitPayload> for WeaveUnitPayload {
        fn from(value: super::WeaveUnitPayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
}
