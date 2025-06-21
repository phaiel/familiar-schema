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
#[doc = "A canonical enum for the types of cognitive dissonance the system can detect."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/system/DissonanceType.v1.json\","]
#[doc = "  \"title\": \"Dissonance Type\","]
#[doc = "  \"description\": \"A canonical enum for the types of cognitive dissonance the system can detect.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"TemporalOverlap\","]
#[doc = "    \"LogicalContradiction\","]
#[doc = "    \"LowConfidencePattern\","]
#[doc = "    \"UnresolvedEntity\""]
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
pub enum DissonanceType {
    TemporalOverlap,
    LogicalContradiction,
    LowConfidencePattern,
    UnresolvedEntity,
}
impl ::std::convert::From<&Self> for DissonanceType {
    fn from(value: &DissonanceType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DissonanceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TemporalOverlap => write!(f, "TemporalOverlap"),
            Self::LogicalContradiction => write!(f, "LogicalContradiction"),
            Self::LowConfidencePattern => write!(f, "LowConfidencePattern"),
            Self::UnresolvedEntity => write!(f, "UnresolvedEntity"),
        }
    }
}
impl ::std::str::FromStr for DissonanceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TemporalOverlap" => Ok(Self::TemporalOverlap),
            "LogicalContradiction" => Ok(Self::LogicalContradiction),
            "LowConfidencePattern" => Ok(Self::LowConfidencePattern),
            "UnresolvedEntity" => Ok(Self::UnresolvedEntity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The current status of a task or process."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "  \"title\": \"Status Field\","]
#[doc = "  \"description\": \"The current status of a task or process.\","]
#[doc = "  \"default\": \"Pending\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Pending\","]
#[doc = "    \"InProgress\","]
#[doc = "    \"Completed\","]
#[doc = "    \"Cancelled\""]
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
pub enum StatusField {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}
impl ::std::convert::From<&Self> for StatusField {
    fn from(value: &StatusField) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StatusField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Pending => write!(f, "Pending"),
            Self::InProgress => write!(f, "InProgress"),
            Self::Completed => write!(f, "Completed"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}
impl ::std::str::FromStr for StatusField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Pending" => Ok(Self::Pending),
            "InProgress" => Ok(Self::InProgress),
            "Completed" => Ok(Self::Completed),
            "Cancelled" => Ok(Self::Cancelled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StatusField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}

#[doc = "A system entity that represents a cognitive dissonance or ambiguity that requires resolution, often through a Human-in-the-Loop workflow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/Stitch.v1.schema.json\","]
#[doc = "  \"title\": \"Stitch Entity\","]
#[doc = "  \"description\": \"A system entity that represents a cognitive dissonance or ambiguity that requires resolution, often through a Human-in-the-Loop workflow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"not\": {"]
#[doc = "    \"description\": \"Rule 17.2: System entities MUST NOT have physics components.\","]
#[doc = "    \"required\": ["]
#[doc = "      \"components\""]
#[doc = "    ],"]
#[doc = "    \"properties\": {"]
#[doc = "      \"components\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"manifold_position\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"universal_physics_state\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"manifold_position\": {"]
#[doc = "            \"title\": \"BaseSystemManifoldPosition\","]
#[doc = "            \"type\": \"object\""]
#[doc = "          },"]
#[doc = "          \"universal_physics_state\": {"]
#[doc = "            \"title\": \"BaseSystemUniversalPhysicsState\","]
#[doc = "            \"type\": \"object\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"entity_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"StitchComponents\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"dissonance_details\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"dissonance_details\": {"]
#[doc = "          \"title\": \"StitchcomponentsDissonanceDetails\","]
#[doc = "          \"description\": \"Details about the cognitive conflict.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"dissonance_type\","]
#[doc = "            \"severity\","]
#[doc = "            \"source_entities\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"dissonance_type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/system/DissonanceType.v1.json\","]
#[doc = "              \"title\": \"Dissonance Type\","]
#[doc = "              \"description\": \"A canonical enum for the types of cognitive dissonance the system can detect.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"TemporalOverlap\","]
#[doc = "                \"LogicalContradiction\","]
#[doc = "                \"LowConfidencePattern\","]
#[doc = "                \"UnresolvedEntity\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"severity\": {"]
#[doc = "              \"description\": \"The calculated severity of the conflict, from 0.0 (minor) to 1.0 (critical).\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"maximum\": 1.0,"]
#[doc = "              \"minimum\": 0.0"]
#[doc = "            },"]
#[doc = "            \"source_entities\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "              \"title\": \"Entity ID List\","]
#[doc = "              \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"uniqueItems\": true"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"resolution_request\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/payloads/StitchInteractionRequest.schema.json\","]
#[doc = "          \"title\": \"StitchInteractionRequest\","]
#[doc = "          \"description\": \"The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"interaction_id\","]
#[doc = "            \"interaction_type\","]
#[doc = "            \"prompt_details\","]
#[doc = "            \"prompt_title\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"agent_context\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"interaction_id\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "              \"title\": \"UUID\","]
#[doc = "              \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"interaction_type\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"options\","]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"options\": {"]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                        \"type\": \"object\","]
#[doc = "                        \"required\": ["]
#[doc = "                          \"id\","]
#[doc = "                          \"label\""]
#[doc = "                        ],"]
#[doc = "                        \"properties\": {"]
#[doc = "                          \"description\": {"]
#[doc = "                            \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                            \"title\": \"Description Field\","]
#[doc = "                            \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                            \"default\": \"\","]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          },"]
#[doc = "                          \"id\": {"]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          },"]
#[doc = "                          \"label\": {"]
#[doc = "                            \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                            \"title\": \"Label Field\","]
#[doc = "                            \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          }"]
#[doc = "                        }"]
#[doc = "                      }"]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"MultipleChoice\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"confirm_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"deny_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"Confirmation\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"placeholder\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"submit_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"FreeText\""]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"prompt_details\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"prompt_title\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$defs\": {"]
#[doc = "            \"StitchInteractionType\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"options\": {"]
#[doc = "                      \"items\": {"]
#[doc = "                        \"properties\": {"]
#[doc = "                          \"description\": {"]
#[doc = "                            \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                            \"default\": \"\","]
#[doc = "                            \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                            \"title\": \"Description Field\","]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          },"]
#[doc = "                          \"id\": {"]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          },"]
#[doc = "                          \"label\": {"]
#[doc = "                            \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                            \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                            \"title\": \"Label Field\","]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          }"]
#[doc = "                        },"]
#[doc = "                        \"required\": ["]
#[doc = "                          \"id\","]
#[doc = "                          \"label\""]
#[doc = "                        ],"]
#[doc = "                        \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                        \"type\": \"object\""]
#[doc = "                      },"]
#[doc = "                      \"type\": \"array\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"MultipleChoice\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"required\": ["]
#[doc = "                    \"type\","]
#[doc = "                    \"options\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"confirm_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"deny_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"Confirmation\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"required\": ["]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"placeholder\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"submit_text\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                      \"title\": \"Label Field\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"const\": \"FreeText\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"required\": ["]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"entity_type\": {"]
#[doc = "      \"description\": \"The canonical type of the system entity.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Stitch\","]
#[doc = "        \"WorkflowTask\","]
#[doc = "        \"Tenant\""]
#[doc = "      ],"]
#[doc = "      \"const\": \"Stitch\""]
#[doc = "    },"]
#[doc = "    \"workflow_state\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/workflow/WorkflowState.v1.json\","]
#[doc = "      \"title\": \"Workflow State\","]
#[doc = "      \"description\": \"Represents the state of a long-running, orchestrated workflow.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"current_step\","]
#[doc = "        \"last_updated\","]
#[doc = "        \"status\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"current_step\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"error_message\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"last_updated\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "          \"title\": \"Nullable Timestamp\","]
#[doc = "          \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"status\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "          \"title\": \"Status Field\","]
#[doc = "          \"description\": \"The current status of a task or process.\","]
#[doc = "          \"default\": \"Pending\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Pending\","]
#[doc = "            \"InProgress\","]
#[doc = "            \"Completed\","]
#[doc = "            \"Cancelled\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"source_file\": \"entities/Stitch.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchEntity {
    pub components: StitchEntityComponents,
    #[doc = "The canonical type of the system entity."]
    pub entity_type: StitchEntityEntityType,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workflow_state: ::std::option::Option<WorkflowState>,
}
impl ::std::convert::From<&StitchEntity> for StitchEntity {
    fn from(value: &StitchEntity) -> Self {
        value.clone()
    }
}
impl StitchEntity {
    pub fn builder() -> builder::StitchEntity {
        Default::default()
    }
}
#[doc = "`StitchEntityComponents`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dissonance_details\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dissonance_details\": {"]
#[doc = "      \"title\": \"StitchcomponentsDissonanceDetails\","]
#[doc = "      \"description\": \"Details about the cognitive conflict.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"dissonance_type\","]
#[doc = "        \"severity\","]
#[doc = "        \"source_entities\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"dissonance_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/system/DissonanceType.v1.json\","]
#[doc = "          \"title\": \"Dissonance Type\","]
#[doc = "          \"description\": \"A canonical enum for the types of cognitive dissonance the system can detect.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"TemporalOverlap\","]
#[doc = "            \"LogicalContradiction\","]
#[doc = "            \"LowConfidencePattern\","]
#[doc = "            \"UnresolvedEntity\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"severity\": {"]
#[doc = "          \"description\": \"The calculated severity of the conflict, from 0.0 (minor) to 1.0 (critical).\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"source_entities\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "          \"title\": \"Entity ID List\","]
#[doc = "          \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"manifold_position\": false,"]
#[doc = "    \"resolution_request\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/payloads/StitchInteractionRequest.schema.json\","]
#[doc = "      \"title\": \"StitchInteractionRequest\","]
#[doc = "      \"description\": \"The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"interaction_id\","]
#[doc = "        \"interaction_type\","]
#[doc = "        \"prompt_details\","]
#[doc = "        \"prompt_title\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"agent_context\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"interaction_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"interaction_type\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"options\","]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"options\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                    \"type\": \"object\","]
#[doc = "                    \"required\": ["]
#[doc = "                      \"id\","]
#[doc = "                      \"label\""]
#[doc = "                    ],"]
#[doc = "                    \"properties\": {"]
#[doc = "                      \"description\": {"]
#[doc = "                        \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                        \"title\": \"Description Field\","]
#[doc = "                        \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                        \"default\": \"\","]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"id\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"label\": {"]
#[doc = "                        \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                        \"title\": \"Label Field\","]
#[doc = "                        \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"MultipleChoice\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"confirm_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"deny_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"Confirmation\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"placeholder\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"submit_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"FreeText\""]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"prompt_details\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"prompt_title\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$defs\": {"]
#[doc = "        \"StitchInteractionType\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"options\": {"]
#[doc = "                  \"items\": {"]
#[doc = "                    \"properties\": {"]
#[doc = "                      \"description\": {"]
#[doc = "                        \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                        \"default\": \"\","]
#[doc = "                        \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                        \"title\": \"Description Field\","]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"id\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"label\": {"]
#[doc = "                        \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                        \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                        \"title\": \"Label Field\","]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    },"]
#[doc = "                    \"required\": ["]
#[doc = "                      \"id\","]
#[doc = "                      \"label\""]
#[doc = "                    ],"]
#[doc = "                    \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                    \"type\": \"object\""]
#[doc = "                  },"]
#[doc = "                  \"type\": \"array\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"MultipleChoice\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"type\","]
#[doc = "                \"options\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"object\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"confirm_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"deny_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"Confirmation\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"object\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"properties\": {"]
#[doc = "                \"placeholder\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"submit_text\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                  \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                  \"title\": \"Label Field\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"const\": \"FreeText\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    },"]
#[doc = "    \"universal_physics_state\": false"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchEntityComponents {
    pub dissonance_details: StitchcomponentsDissonanceDetails,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub resolution_request: ::std::option::Option<StitchInteractionRequest>,
}
impl ::std::convert::From<&StitchEntityComponents> for StitchEntityComponents {
    fn from(value: &StitchEntityComponents) -> Self {
        value.clone()
    }
}
impl StitchEntityComponents {
    pub fn builder() -> builder::StitchEntityComponents {
        Default::default()
    }
}
#[doc = "The canonical type of the system entity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The canonical type of the system entity.\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Stitch\","]
#[doc = "    \"WorkflowTask\","]
#[doc = "    \"Tenant\""]
#[doc = "  ],"]
#[doc = "  \"const\": \"Stitch\""]
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
pub enum StitchEntityEntityType {
    Stitch,
    WorkflowTask,
    Tenant,
}
impl ::std::convert::From<&Self> for StitchEntityEntityType {
    fn from(value: &StitchEntityEntityType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for StitchEntityEntityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stitch => write!(f, "Stitch"),
            Self::WorkflowTask => write!(f, "WorkflowTask"),
            Self::Tenant => write!(f, "Tenant"),
        }
    }
}
impl ::std::str::FromStr for StitchEntityEntityType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Stitch" => Ok(Self::Stitch),
            "WorkflowTask" => Ok(Self::WorkflowTask),
            "Tenant" => Ok(Self::Tenant),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for StitchEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for StitchEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for StitchEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/StitchInteractionRequest.schema.json\","]
#[doc = "  \"title\": \"StitchInteractionRequest\","]
#[doc = "  \"description\": \"The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"interaction_id\","]
#[doc = "    \"interaction_type\","]
#[doc = "    \"prompt_details\","]
#[doc = "    \"prompt_title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent_context\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"interaction_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"interaction_type\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"options\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"options\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"id\","]
#[doc = "                  \"label\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"description\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                    \"title\": \"Description Field\","]
#[doc = "                    \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                    \"default\": \"\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"label\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                    \"title\": \"Label Field\","]
#[doc = "                    \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"MultipleChoice\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"confirm_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"deny_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"Confirmation\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"placeholder\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"submit_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"FreeText\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"prompt_details\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"prompt_title\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "      \"title\": \"Label Field\","]
#[doc = "      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$defs\": {"]
#[doc = "    \"StitchInteractionType\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"options\": {"]
#[doc = "              \"items\": {"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"description\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                    \"default\": \"\","]
#[doc = "                    \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                    \"title\": \"Description Field\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"label\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                    \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                    \"title\": \"Label Field\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"required\": ["]
#[doc = "                  \"id\","]
#[doc = "                  \"label\""]
#[doc = "                ],"]
#[doc = "                \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                \"type\": \"object\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"MultipleChoice\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"type\","]
#[doc = "            \"options\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"object\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"confirm_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"deny_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"Confirmation\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"object\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"properties\": {"]
#[doc = "            \"placeholder\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"submit_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"FreeText\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"object\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchInteractionRequest {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    #[serde(default)]
    pub agent_context: ::std::string::String,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub interaction_id: ::std::string::String,
    pub interaction_type: StitchInteractionRequestInteractionType,
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub prompt_details: ::std::string::String,
    #[doc = "A canonical definition for a short, human-readable UI label or title."]
    pub prompt_title: ::std::string::String,
}
impl ::std::convert::From<&StitchInteractionRequest> for StitchInteractionRequest {
    fn from(value: &StitchInteractionRequest) -> Self {
        value.clone()
    }
}
impl StitchInteractionRequest {
    pub fn builder() -> builder::StitchInteractionRequest {
        Default::default()
    }
}
#[doc = "`StitchInteractionRequestInteractionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"options\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"options\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\","]
#[doc = "              \"label\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"description\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                \"title\": \"Description Field\","]
#[doc = "                \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                \"default\": \"\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"label\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                \"title\": \"Label Field\","]
#[doc = "                \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"MultipleChoice\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"confirm_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"deny_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"Confirmation\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"placeholder\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"submit_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"FreeText\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StitchInteractionRequestInteractionType {
    MultipleChoice {
        options: ::std::vec::Vec<StitchInteractionRequestOneofItemItems>,
    },
    Confirmation {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        confirm_text: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        deny_text: ::std::option::Option<::std::string::String>,
    },
    FreeText {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        placeholder: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        submit_text: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for StitchInteractionRequestInteractionType {
    fn from(value: &StitchInteractionRequestInteractionType) -> Self {
        value.clone()
    }
}
#[doc = "`StitchInteractionRequestOneofItemItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"label\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "      \"title\": \"Label Field\","]
#[doc = "      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchInteractionRequestOneofItemItems {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: ::std::string::String,
    #[doc = "A canonical definition for a short, human-readable UI label or title."]
    pub label: ::std::string::String,
}
impl ::std::convert::From<&StitchInteractionRequestOneofItemItems>
    for StitchInteractionRequestOneofItemItems
{
    fn from(value: &StitchInteractionRequestOneofItemItems) -> Self {
        value.clone()
    }
}
impl StitchInteractionRequestOneofItemItems {
    pub fn builder() -> builder::StitchInteractionRequestOneofItemItems {
        Default::default()
    }
}
#[doc = "Details about the cognitive conflict."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"StitchcomponentsDissonanceDetails\","]
#[doc = "  \"description\": \"Details about the cognitive conflict.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"dissonance_type\","]
#[doc = "    \"severity\","]
#[doc = "    \"source_entities\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"dissonance_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/system/DissonanceType.v1.json\","]
#[doc = "      \"title\": \"Dissonance Type\","]
#[doc = "      \"description\": \"A canonical enum for the types of cognitive dissonance the system can detect.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"TemporalOverlap\","]
#[doc = "        \"LogicalContradiction\","]
#[doc = "        \"LowConfidencePattern\","]
#[doc = "        \"UnresolvedEntity\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"severity\": {"]
#[doc = "      \"description\": \"The calculated severity of the conflict, from 0.0 (minor) to 1.0 (critical).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"source_entities\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "      \"title\": \"Entity ID List\","]
#[doc = "      \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchcomponentsDissonanceDetails {
    #[doc = "A canonical enum for the types of cognitive dissonance the system can detect."]
    pub dissonance_type: DissonanceType,
    pub severity: f64,
    #[doc = "A canonical definition for a list of unique entity identifiers (UUIDs)."]
    pub source_entities: Vec<::std::string::String>,
}
impl ::std::convert::From<&StitchcomponentsDissonanceDetails>
    for StitchcomponentsDissonanceDetails
{
    fn from(value: &StitchcomponentsDissonanceDetails) -> Self {
        value.clone()
    }
}
impl StitchcomponentsDissonanceDetails {
    pub fn builder() -> builder::StitchcomponentsDissonanceDetails {
        Default::default()
    }
}
#[doc = "Represents the state of a long-running, orchestrated workflow."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/workflow/WorkflowState.v1.json\","]
#[doc = "  \"title\": \"Workflow State\","]
#[doc = "  \"description\": \"Represents the state of a long-running, orchestrated workflow.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"current_step\","]
#[doc = "    \"last_updated\","]
#[doc = "    \"status\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"current_step\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"error_message\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"last_updated\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "      \"title\": \"Nullable Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Status.v1.json\","]
#[doc = "      \"title\": \"Status Field\","]
#[doc = "      \"description\": \"The current status of a task or process.\","]
#[doc = "      \"default\": \"Pending\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Pending\","]
#[doc = "        \"InProgress\","]
#[doc = "        \"Completed\","]
#[doc = "        \"Cancelled\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WorkflowState {
    pub current_step: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error_message: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for an optional ISO 8601 timestamp with timezone."]
    pub last_updated: ::std::option::Option<::std::string::String>,
    #[doc = "The current status of a task or process."]
    pub status: StatusField,
}
impl ::std::convert::From<&WorkflowState> for WorkflowState {
    fn from(value: &WorkflowState) -> Self {
        value.clone()
    }
}
impl WorkflowState {
    pub fn builder() -> builder::WorkflowState {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct StitchEntity {
        components: ::std::result::Result<super::StitchEntityComponents, ::std::string::String>,
        entity_type: ::std::result::Result<super::StitchEntityEntityType, ::std::string::String>,
        workflow_state: ::std::result::Result<
            ::std::option::Option<super::WorkflowState>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StitchEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                workflow_state: Ok(Default::default()),
            }
        }
    }
    impl StitchEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchEntityComponents>,
            T::Error: ::std::fmt::Display,
        {
            self.components = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for components: {}", e));
            self
        }
        pub fn entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchEntityEntityType>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity_type: {}", e));
            self
        }
        pub fn workflow_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WorkflowState>>,
            T::Error: ::std::fmt::Display,
        {
            self.workflow_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for workflow_state: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchEntity> for super::StitchEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                entity_type: value.entity_type?,
                workflow_state: value.workflow_state?,
            })
        }
    }
    impl ::std::convert::From<super::StitchEntity> for StitchEntity {
        fn from(value: super::StitchEntity) -> Self {
            Self {
                components: Ok(value.components),
                entity_type: Ok(value.entity_type),
                workflow_state: Ok(value.workflow_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StitchEntityComponents {
        dissonance_details:
            ::std::result::Result<super::StitchcomponentsDissonanceDetails, ::std::string::String>,
        resolution_request: ::std::result::Result<
            ::std::option::Option<super::StitchInteractionRequest>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for StitchEntityComponents {
        fn default() -> Self {
            Self {
                dissonance_details: Err("no value supplied for dissonance_details".to_string()),
                resolution_request: Ok(Default::default()),
            }
        }
    }
    impl StitchEntityComponents {
        pub fn dissonance_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchcomponentsDissonanceDetails>,
            T::Error: ::std::fmt::Display,
        {
            self.dissonance_details = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for dissonance_details: {}",
                    e
                )
            });
            self
        }
        pub fn resolution_request<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::StitchInteractionRequest>>,
            T::Error: ::std::fmt::Display,
        {
            self.resolution_request = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for resolution_request: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<StitchEntityComponents> for super::StitchEntityComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchEntityComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dissonance_details: value.dissonance_details?,
                resolution_request: value.resolution_request?,
            })
        }
    }
    impl ::std::convert::From<super::StitchEntityComponents> for StitchEntityComponents {
        fn from(value: super::StitchEntityComponents) -> Self {
            Self {
                dissonance_details: Ok(value.dissonance_details),
                resolution_request: Ok(value.resolution_request),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StitchInteractionRequest {
        agent_context: ::std::result::Result<::std::string::String, ::std::string::String>,
        interaction_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        interaction_type: ::std::result::Result<
            super::StitchInteractionRequestInteractionType,
            ::std::string::String,
        >,
        prompt_details: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt_title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StitchInteractionRequest {
        fn default() -> Self {
            Self {
                agent_context: Ok(Default::default()),
                interaction_id: Err("no value supplied for interaction_id".to_string()),
                interaction_type: Err("no value supplied for interaction_type".to_string()),
                prompt_details: Err("no value supplied for prompt_details".to_string()),
                prompt_title: Err("no value supplied for prompt_title".to_string()),
            }
        }
    }
    impl StitchInteractionRequest {
        pub fn agent_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_context: {}", e));
            self
        }
        pub fn interaction_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.interaction_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interaction_id: {}", e));
            self
        }
        pub fn interaction_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchInteractionRequestInteractionType>,
            T::Error: ::std::fmt::Display,
        {
            self.interaction_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for interaction_type: {}",
                    e
                )
            });
            self
        }
        pub fn prompt_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_details: {}", e));
            self
        }
        pub fn prompt_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchInteractionRequest> for super::StitchInteractionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchInteractionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_context: value.agent_context?,
                interaction_id: value.interaction_id?,
                interaction_type: value.interaction_type?,
                prompt_details: value.prompt_details?,
                prompt_title: value.prompt_title?,
            })
        }
    }
    impl ::std::convert::From<super::StitchInteractionRequest> for StitchInteractionRequest {
        fn from(value: super::StitchInteractionRequest) -> Self {
            Self {
                agent_context: Ok(value.agent_context),
                interaction_id: Ok(value.interaction_id),
                interaction_type: Ok(value.interaction_type),
                prompt_details: Ok(value.prompt_details),
                prompt_title: Ok(value.prompt_title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StitchInteractionRequestOneofItemItems {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StitchInteractionRequestOneofItemItems {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                label: Err("no value supplied for label".to_string()),
            }
        }
    }
    impl StitchInteractionRequestOneofItemItems {
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
    }
    impl ::std::convert::TryFrom<StitchInteractionRequestOneofItemItems>
        for super::StitchInteractionRequestOneofItemItems
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchInteractionRequestOneofItemItems,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::StitchInteractionRequestOneofItemItems>
        for StitchInteractionRequestOneofItemItems
    {
        fn from(value: super::StitchInteractionRequestOneofItemItems) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                label: Ok(value.label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StitchcomponentsDissonanceDetails {
        dissonance_type: ::std::result::Result<super::DissonanceType, ::std::string::String>,
        severity: ::std::result::Result<f64, ::std::string::String>,
        source_entities: ::std::result::Result<Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for StitchcomponentsDissonanceDetails {
        fn default() -> Self {
            Self {
                dissonance_type: Err("no value supplied for dissonance_type".to_string()),
                severity: Err("no value supplied for severity".to_string()),
                source_entities: Err("no value supplied for source_entities".to_string()),
            }
        }
    }
    impl StitchcomponentsDissonanceDetails {
        pub fn dissonance_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::DissonanceType>,
            T::Error: ::std::fmt::Display,
        {
            self.dissonance_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for dissonance_type: {}", e));
            self
        }
        pub fn severity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.severity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for severity: {}", e));
            self
        }
        pub fn source_entities<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_entities = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_entities: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchcomponentsDissonanceDetails>
        for super::StitchcomponentsDissonanceDetails
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchcomponentsDissonanceDetails,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dissonance_type: value.dissonance_type?,
                severity: value.severity?,
                source_entities: value.source_entities?,
            })
        }
    }
    impl ::std::convert::From<super::StitchcomponentsDissonanceDetails>
        for StitchcomponentsDissonanceDetails
    {
        fn from(value: super::StitchcomponentsDissonanceDetails) -> Self {
            Self {
                dissonance_type: Ok(value.dissonance_type),
                severity: Ok(value.severity),
                source_entities: Ok(value.source_entities),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WorkflowState {
        current_step: ::std::result::Result<::std::string::String, ::std::string::String>,
        error_message: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        last_updated: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status: ::std::result::Result<super::StatusField, ::std::string::String>,
    }
    impl ::std::default::Default for WorkflowState {
        fn default() -> Self {
            Self {
                current_step: Err("no value supplied for current_step".to_string()),
                error_message: Ok(Default::default()),
                last_updated: Err("no value supplied for last_updated".to_string()),
                status: Err("no value supplied for status".to_string()),
            }
        }
    }
    impl WorkflowState {
        pub fn current_step<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.current_step = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for current_step: {}", e));
            self
        }
        pub fn error_message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_message: {}", e));
            self
        }
        pub fn last_updated<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_updated = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_updated: {}", e));
            self
        }
        pub fn status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StatusField>,
            T::Error: ::std::fmt::Display,
        {
            self.status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WorkflowState> for super::WorkflowState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WorkflowState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                current_step: value.current_step?,
                error_message: value.error_message?,
                last_updated: value.last_updated?,
                status: value.status?,
            })
        }
    }
    impl ::std::convert::From<super::WorkflowState> for WorkflowState {
        fn from(value: super::WorkflowState) -> Self {
            Self {
                current_step: Ok(value.current_step),
                error_message: Ok(value.error_message),
                last_updated: Ok(value.last_updated),
                status: Ok(value.status),
            }
        }
    }
}
