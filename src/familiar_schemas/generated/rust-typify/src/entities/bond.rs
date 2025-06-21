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
#[doc = "Component references temporarily disabled for pipeline testing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BondComponents\","]
#[doc = "  \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"content\","]
#[doc = "    \"permissions\","]
#[doc = "    \"physics_config\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_comment\": {"]
#[doc = "      \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/BondContent.v1.schema.json\","]
#[doc = "      \"title\": \"Bond Content Component\","]
#[doc = "      \"description\": \"Defines the descriptive content and history of a relationship Bond.\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"BondContentFields\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"relationship_type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"history\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "                \"title\": \"Bond Event Type\","]
#[doc = "                \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"impact\","]
#[doc = "                  \"moment_id\","]
#[doc = "                  \"timestamp\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"impact\": {"]
#[doc = "                    \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "                    \"type\": \"number\","]
#[doc = "                    \"maximum\": 1.0,"]
#[doc = "                    \"minimum\": -1.0"]
#[doc = "                  },"]
#[doc = "                  \"moment_id\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"timestamp\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "                    \"title\": \"Timestamp\","]
#[doc = "                    \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"relationship_type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "              \"title\": \"Relationship Type\","]
#[doc = "              \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Family\","]
#[doc = "                \"Friend\","]
#[doc = "                \"Romantic\","]
#[doc = "                \"Professional\","]
#[doc = "                \"Acquaintance\","]
#[doc = "                \"Adversarial\""]
#[doc = "              ]"]
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
#[doc = "      \"schema_version\": \"1.1.0\""]
#[doc = "    },"]
#[doc = "    \"permissions\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/BondPermissions.v1.schema.json\","]
#[doc = "      \"title\": \"Bond Permissions Component\","]
#[doc = "      \"description\": \"Defines the access control and privacy rules for a Bond and its associated Moments.\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"allow_cross_tenant_view\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"description\": \"If true, allows linked users from other tenants to view data according to their permissions.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"min_age_for_viewing\": {"]
#[doc = "          \"default\": 13,"]
#[doc = "          \"description\": \"The minimum age a child Thread must be for this bond's moments to be visible to them. Supports age-aware privacy.\","]
#[doc = "          \"minimum\": 0,"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"owner_permissions\": {"]
#[doc = "          \"default\": ["]
#[doc = "            \"Read\","]
#[doc = "            \"Write\","]
#[doc = "            \"Delete\","]
#[doc = "            \"Share\""]
#[doc = "          ],"]
#[doc = "          \"description\": \"Permissions for the owner of the bond (the user in the current tenant).\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/security/BondPermission.v1.json\","]
#[doc = "            \"description\": \"A canonical enum of permissions related to a Bond.\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"Read\","]
#[doc = "              \"Write\","]
#[doc = "              \"Delete\","]
#[doc = "              \"Share\""]
#[doc = "            ],"]
#[doc = "            \"title\": \"Bond Permission\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": \"array\""]
#[doc = "        },"]
#[doc = "        \"subject_permissions\": {"]
#[doc = "          \"default\": ["]
#[doc = "            \"Read\","]
#[doc = "            \"Contribute\""]
#[doc = "          ],"]
#[doc = "          \"description\": \"Permissions for the other person in the bond.\","]
#[doc = "          \"items\": {"]
#[doc = "            \"enum\": ["]
#[doc = "              \"Read\","]
#[doc = "              \"Write\","]
#[doc = "              \"Contribute\""]
#[doc = "            ],"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"type\": \"array\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.0.0\""]
#[doc = "    },"]
#[doc = "    \"physics_config\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/BondPhysicsConfig.v1.schema.json\","]
#[doc = "      \"title\": \"Bond Physics Config Component\","]
#[doc = "      \"description\": \"Configuration that defines the physics model for a Bond (Dynamic or Static).\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"bond_model\": {"]
#[doc = "          \"description\": \"The physics model to apply to this bond.\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Dynamic\","]
#[doc = "            \"Static\""]
#[doc = "          ],"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"dynamic_params\": {"]
#[doc = "          \"description\": \"Parameters for the spring-damper model, used only if bond_model is 'Dynamic'.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"base_damping_coefficient\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"base_spring_constant\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"base_spring_constant\","]
#[doc = "            \"base_damping_coefficient\""]
#[doc = "          ],"]
#[doc = "          \"title\": \"BondPhysicsConfigDynamicParams\","]
#[doc = "          \"type\": \"object\""]
#[doc = "        },"]
#[doc = "        \"static_params\": {"]
#[doc = "          \"description\": \"Parameters for the associative model, used only if bond_model is 'Static'.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"consolidation_boost\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"emotional_anchor_strength\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"consolidation_boost\","]
#[doc = "            \"emotional_anchor_strength\""]
#[doc = "          ],"]
#[doc = "          \"title\": \"BondPhysicsConfigStaticParams\","]
#[doc = "          \"type\": \"object\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.0.0\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondComponents {
    #[doc = "TODO: Re-enable component references when component schemas are available"]
    #[serde(
        rename = "_comment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comment: ::std::option::Option<::std::string::String>,
    pub content: BondContentComponent,
    #[doc = "Defines the access control and privacy rules for a Bond and its associated Moments."]
    pub permissions: ::serde_json::Value,
    #[doc = "Configuration that defines the physics model for a Bond (Dynamic or Static)."]
    pub physics_config: ::serde_json::Value,
}
impl ::std::convert::From<&BondComponents> for BondComponents {
    fn from(value: &BondComponents) -> Self {
        value.clone()
    }
}
impl BondComponents {
    pub fn builder() -> builder::BondComponents {
        Default::default()
    }
}
#[doc = "Defines the descriptive content and history of a relationship Bond."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/BondContent.v1.schema.json\","]
#[doc = "  \"title\": \"Bond Content Component\","]
#[doc = "  \"description\": \"Defines the descriptive content and history of a relationship Bond.\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"BondContentFields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"relationship_type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"description\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"history\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "            \"title\": \"Bond Event Type\","]
#[doc = "            \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"impact\","]
#[doc = "              \"moment_id\","]
#[doc = "              \"timestamp\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"impact\": {"]
#[doc = "                \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "                \"type\": \"number\","]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minimum\": -1.0"]
#[doc = "              },"]
#[doc = "              \"moment_id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"timestamp\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "                \"title\": \"Timestamp\","]
#[doc = "                \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"relationship_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "          \"title\": \"Relationship Type\","]
#[doc = "          \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Family\","]
#[doc = "            \"Friend\","]
#[doc = "            \"Romantic\","]
#[doc = "            \"Professional\","]
#[doc = "            \"Acquaintance\","]
#[doc = "            \"Adversarial\""]
#[doc = "          ]"]
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
#[doc = "  \"schema_version\": \"1.1.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondContentComponent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<BondContentFields>,
}
impl ::std::convert::From<&BondContentComponent> for BondContentComponent {
    fn from(value: &BondContentComponent) -> Self {
        value.clone()
    }
}

impl BondContentComponent {
    pub fn builder() -> builder::BondContentComponent {
        Default::default()
    }
}
#[doc = "`BondContentFields`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BondContentFields\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"relationship_type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"history\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "        \"title\": \"Bond Event Type\","]
#[doc = "        \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"impact\","]
#[doc = "          \"moment_id\","]
#[doc = "          \"timestamp\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"impact\": {"]
#[doc = "            \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "            \"type\": \"number\","]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minimum\": -1.0"]
#[doc = "          },"]
#[doc = "          \"moment_id\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"timestamp\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "            \"title\": \"Timestamp\","]
#[doc = "            \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"relationship_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "      \"title\": \"Relationship Type\","]
#[doc = "      \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Family\","]
#[doc = "        \"Friend\","]
#[doc = "        \"Romantic\","]
#[doc = "        \"Professional\","]
#[doc = "        \"Acquaintance\","]
#[doc = "        \"Adversarial\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct BondContentFields {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    #[serde(default)]
    pub description: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub history: ::std::vec::Vec<BondEventType>,
    #[doc = "A canonical enum of all possible relationship types between Threads."]
    pub relationship_type: RelationshipType,
}
impl ::std::convert::From<&BondContentFields> for BondContentFields {
    fn from(value: &BondContentFields) -> Self {
        value.clone()
    }
}
impl BondContentFields {
    pub fn builder() -> builder::BondContentFields {
        Default::default()
    }
}
#[doc = "A classical entity representing a persistent relationship between two Thread entities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/Bond.v1.schema.json\","]
#[doc = "  \"title\": \"Bond Entity\","]
#[doc = "  \"description\": \"A classical entity representing a persistent relationship between two Thread entities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"entity_type\","]
#[doc = "    \"physics_state\","]
#[doc = "    \"thread_a_id\","]
#[doc = "    \"thread_b_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"BondComponents\","]
#[doc = "      \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"content\","]
#[doc = "        \"permissions\","]
#[doc = "        \"physics_config\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"_comment\": {"]
#[doc = "          \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"content\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/BondContent.v1.schema.json\","]
#[doc = "          \"title\": \"Bond Content Component\","]
#[doc = "          \"description\": \"Defines the descriptive content and history of a relationship Bond.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"BondContentFields\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"relationship_type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"description\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                  \"title\": \"Description Field\","]
#[doc = "                  \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                  \"default\": \"\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"history\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "                    \"title\": \"Bond Event Type\","]
#[doc = "                    \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "                    \"type\": \"object\","]
#[doc = "                    \"required\": ["]
#[doc = "                      \"impact\","]
#[doc = "                      \"moment_id\","]
#[doc = "                      \"timestamp\""]
#[doc = "                    ],"]
#[doc = "                    \"properties\": {"]
#[doc = "                      \"impact\": {"]
#[doc = "                        \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "                        \"type\": \"number\","]
#[doc = "                        \"maximum\": 1.0,"]
#[doc = "                        \"minimum\": -1.0"]
#[doc = "                      },"]
#[doc = "                      \"moment_id\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      },"]
#[doc = "                      \"timestamp\": {"]
#[doc = "                        \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "                        \"title\": \"Timestamp\","]
#[doc = "                        \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    }"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"relationship_type\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "                  \"title\": \"Relationship Type\","]
#[doc = "                  \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Family\","]
#[doc = "                    \"Friend\","]
#[doc = "                    \"Romantic\","]
#[doc = "                    \"Professional\","]
#[doc = "                    \"Acquaintance\","]
#[doc = "                    \"Adversarial\""]
#[doc = "                  ]"]
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
#[doc = "          \"schema_version\": \"1.1.0\""]
#[doc = "        },"]
#[doc = "        \"permissions\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/BondPermissions.v1.schema.json\","]
#[doc = "          \"title\": \"Bond Permissions Component\","]
#[doc = "          \"description\": \"Defines the access control and privacy rules for a Bond and its associated Moments.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"allow_cross_tenant_view\": {"]
#[doc = "              \"default\": false,"]
#[doc = "              \"description\": \"If true, allows linked users from other tenants to view data according to their permissions.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"min_age_for_viewing\": {"]
#[doc = "              \"default\": 13,"]
#[doc = "              \"description\": \"The minimum age a child Thread must be for this bond's moments to be visible to them. Supports age-aware privacy.\","]
#[doc = "              \"minimum\": 0,"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"owner_permissions\": {"]
#[doc = "              \"default\": ["]
#[doc = "                \"Read\","]
#[doc = "                \"Write\","]
#[doc = "                \"Delete\","]
#[doc = "                \"Share\""]
#[doc = "              ],"]
#[doc = "              \"description\": \"Permissions for the owner of the bond (the user in the current tenant).\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/security/BondPermission.v1.json\","]
#[doc = "                \"description\": \"A canonical enum of permissions related to a Bond.\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"Read\","]
#[doc = "                  \"Write\","]
#[doc = "                  \"Delete\","]
#[doc = "                  \"Share\""]
#[doc = "                ],"]
#[doc = "                \"title\": \"Bond Permission\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"subject_permissions\": {"]
#[doc = "              \"default\": ["]
#[doc = "                \"Read\","]
#[doc = "                \"Contribute\""]
#[doc = "              ],"]
#[doc = "              \"description\": \"Permissions for the other person in the bond.\","]
#[doc = "              \"items\": {"]
#[doc = "                \"enum\": ["]
#[doc = "                  \"Read\","]
#[doc = "                  \"Write\","]
#[doc = "                  \"Contribute\""]
#[doc = "                ],"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.0.0\""]
#[doc = "        },"]
#[doc = "        \"physics_config\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/BondPhysicsConfig.v1.schema.json\","]
#[doc = "          \"title\": \"Bond Physics Config Component\","]
#[doc = "          \"description\": \"Configuration that defines the physics model for a Bond (Dynamic or Static).\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"bond_model\": {"]
#[doc = "              \"description\": \"The physics model to apply to this bond.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Dynamic\","]
#[doc = "                \"Static\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"dynamic_params\": {"]
#[doc = "              \"description\": \"Parameters for the spring-damper model, used only if bond_model is 'Dynamic'.\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"base_damping_coefficient\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"base_spring_constant\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"base_spring_constant\","]
#[doc = "                \"base_damping_coefficient\""]
#[doc = "              ],"]
#[doc = "              \"title\": \"BondPhysicsConfigDynamicParams\","]
#[doc = "              \"type\": \"object\""]
#[doc = "            },"]
#[doc = "            \"static_params\": {"]
#[doc = "              \"description\": \"Parameters for the associative model, used only if bond_model is 'Static'.\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"consolidation_boost\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"emotional_anchor_strength\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"consolidation_boost\","]
#[doc = "                \"emotional_anchor_strength\""]
#[doc = "              ],"]
#[doc = "              \"title\": \"BondPhysicsConfigStaticParams\","]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.0.0\""]
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
#[doc = "      \"title\": \"BondPhysicsState\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"tension\","]
#[doc = "        \"universal\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"tension\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/BondTension.v1.schema.json\","]
#[doc = "          \"title\": \"Bond Tension Component\","]
#[doc = "          \"description\": \"Models the classical physics of a relationship Bond, including its strength, tension, and resonance properties. This is managed by the Particular engine.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"bond_strength\": {"]
#[doc = "              \"constraints\": {"]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minimum\": 0.0"]
#[doc = "              },"]
#[doc = "              \"default\": 0.5,"]
#[doc = "              \"description\": \"The overall strength or health of the bond.\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            },"]
#[doc = "            \"damping_coefficient\": {"]
#[doc = "              \"default\": 0.5,"]
#[doc = "              \"description\": \"The 'inertia' of the bond, representing how quickly it returns to equilibrium after a perturbation.\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            },"]
#[doc = "            \"resonance_phase\": {"]
#[doc = "              \"default\": 0.0,"]
#[doc = "              \"description\": \"The phase angle for resonance calculations with other bonds.\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            },"]
#[doc = "            \"spring_constant\": {"]
#[doc = "              \"default\": 10.0,"]
#[doc = "              \"description\": \"The 'stiffness' of the bond, representing its resistance to change.\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            },"]
#[doc = "            \"torsional_stress\": {"]
#[doc = "              \"default\": 0.0,"]
#[doc = "              \"description\": \"The cognitive dissonance or strain on the bond. Can be positive (creative tension) or negative (draining conflict).\","]
#[doc = "              \"format\": \"double\","]
#[doc = "              \"type\": \"number\","]
#[doc = "              \"x-python-type\": \"float\","]
#[doc = "              \"x-rust-type\": \"f64\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"collapse_sensitive\": false,"]
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
#[doc = "    },"]
#[doc = "    \"thread_a_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"thread_b_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"source_file\": \"entities/Bond.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondEntity {
    pub components: BondComponents,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    pub entity_type: EntityType,
    pub physics_state: BondPhysicsState,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub thread_a_id: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub thread_b_id: ::std::string::String,
}
impl ::std::convert::From<&BondEntity> for BondEntity {
    fn from(value: &BondEntity) -> Self {
        value.clone()
    }
}
impl BondEntity {
    pub fn builder() -> builder::BondEntity {
        Default::default()
    }
}
#[doc = "Represents a significant event that impacted a bond's strength or state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "  \"title\": \"Bond Event Type\","]
#[doc = "  \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"impact\","]
#[doc = "    \"moment_id\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"impact\": {"]
#[doc = "      \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": -1.0"]
#[doc = "    },"]
#[doc = "    \"moment_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondEventType {
    pub impact: f64,
    pub moment_id: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&BondEventType> for BondEventType {
    fn from(value: &BondEventType) -> Self {
        value.clone()
    }
}
impl BondEventType {
    pub fn builder() -> builder::BondEventType {
        Default::default()
    }
}
#[doc = "`BondPhysicsState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BondPhysicsState\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"tension\","]
#[doc = "    \"universal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"tension\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/BondTension.v1.schema.json\","]
#[doc = "      \"title\": \"Bond Tension Component\","]
#[doc = "      \"description\": \"Models the classical physics of a relationship Bond, including its strength, tension, and resonance properties. This is managed by the Particular engine.\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"bond_strength\": {"]
#[doc = "          \"constraints\": {"]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minimum\": 0.0"]
#[doc = "          },"]
#[doc = "          \"default\": 0.5,"]
#[doc = "          \"description\": \"The overall strength or health of the bond.\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        },"]
#[doc = "        \"damping_coefficient\": {"]
#[doc = "          \"default\": 0.5,"]
#[doc = "          \"description\": \"The 'inertia' of the bond, representing how quickly it returns to equilibrium after a perturbation.\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        },"]
#[doc = "        \"resonance_phase\": {"]
#[doc = "          \"default\": 0.0,"]
#[doc = "          \"description\": \"The phase angle for resonance calculations with other bonds.\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        },"]
#[doc = "        \"spring_constant\": {"]
#[doc = "          \"default\": 10.0,"]
#[doc = "          \"description\": \"The 'stiffness' of the bond, representing its resistance to change.\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        },"]
#[doc = "        \"torsional_stress\": {"]
#[doc = "          \"default\": 0.0,"]
#[doc = "          \"description\": \"The cognitive dissonance or strain on the bond. Can be positive (creative tension) or negative (draining conflict).\","]
#[doc = "          \"format\": \"double\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"x-python-type\": \"float\","]
#[doc = "          \"x-rust-type\": \"f64\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"collapse_sensitive\": false,"]
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
pub struct BondPhysicsState {
    #[doc = "Models the classical physics of a relationship Bond, including its strength, tension, and resonance properties. This is managed by the Particular engine."]
    pub tension: ::serde_json::Value,
    pub universal: UniversalPhysicsStateComponent,
}
impl ::std::convert::From<&BondPhysicsState> for BondPhysicsState {
    fn from(value: &BondPhysicsState) -> Self {
        value.clone()
    }
}
impl BondPhysicsState {
    pub fn builder() -> builder::BondPhysicsState {
        Default::default()
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
#[doc = "A canonical enum of all possible relationship types between Threads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "  \"title\": \"Relationship Type\","]
#[doc = "  \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Family\","]
#[doc = "    \"Friend\","]
#[doc = "    \"Romantic\","]
#[doc = "    \"Professional\","]
#[doc = "    \"Acquaintance\","]
#[doc = "    \"Adversarial\""]
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
pub enum RelationshipType {
    Family,
    Friend,
    Romantic,
    Professional,
    Acquaintance,
    Adversarial,
}
impl ::std::convert::From<&Self> for RelationshipType {
    fn from(value: &RelationshipType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Family => write!(f, "Family"),
            Self::Friend => write!(f, "Friend"),
            Self::Romantic => write!(f, "Romantic"),
            Self::Professional => write!(f, "Professional"),
            Self::Acquaintance => write!(f, "Acquaintance"),
            Self::Adversarial => write!(f, "Adversarial"),
        }
    }
}
impl ::std::str::FromStr for RelationshipType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Family" => Ok(Self::Family),
            "Friend" => Ok(Self::Friend),
            "Romantic" => Ok(Self::Romantic),
            "Professional" => Ok(Self::Professional),
            "Acquaintance" => Ok(Self::Acquaintance),
            "Adversarial" => Ok(Self::Adversarial),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RelationshipType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RelationshipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RelationshipType {
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
    pub struct BondComponents {
        comment: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        content: ::std::result::Result<super::BondContentComponent, ::std::string::String>,
        permissions: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        physics_config: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for BondComponents {
        fn default() -> Self {
            Self {
                comment: Ok(Default::default()),
                content: Err("no value supplied for content".to_string()),
                permissions: Err("no value supplied for permissions".to_string()),
                physics_config: Err("no value supplied for physics_config".to_string()),
            }
        }
    }
    impl BondComponents {
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
            T: ::std::convert::TryInto<super::BondContentComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn permissions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for permissions: {}", e));
            self
        }
        pub fn physics_config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for physics_config: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondComponents> for super::BondComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comment: value.comment?,
                content: value.content?,
                permissions: value.permissions?,
                physics_config: value.physics_config?,
            })
        }
    }
    impl ::std::convert::From<super::BondComponents> for BondComponents {
        fn from(value: super::BondComponents) -> Self {
            Self {
                comment: Ok(value.comment),
                content: Ok(value.content),
                permissions: Ok(value.permissions),
                physics_config: Ok(value.physics_config),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondContentComponent {
        fields: ::std::result::Result<
            ::std::option::Option<super::BondContentFields>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BondContentComponent {
        fn default() -> Self {
            Self {
                fields: Ok(Default::default()),
            }
        }
    }
    impl BondContentComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BondContentFields>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondContentComponent> for super::BondContentComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondContentComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
            })
        }
    }
    impl ::std::convert::From<super::BondContentComponent> for BondContentComponent {
        fn from(value: super::BondContentComponent) -> Self {
            Self {
                fields: Ok(value.fields),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondContentFields {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        history:
            ::std::result::Result<::std::vec::Vec<super::BondEventType>, ::std::string::String>,
        relationship_type: ::std::result::Result<super::RelationshipType, ::std::string::String>,
    }
    impl ::std::default::Default for BondContentFields {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                history: Ok(Default::default()),
                relationship_type: Err("no value supplied for relationship_type".to_string()),
            }
        }
    }
    impl BondContentFields {
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
        pub fn history<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BondEventType>>,
            T::Error: ::std::fmt::Display,
        {
            self.history = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for history: {}", e));
            self
        }
        pub fn relationship_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RelationshipType>,
            T::Error: ::std::fmt::Display,
        {
            self.relationship_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for relationship_type: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<BondContentFields> for super::BondContentFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondContentFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                history: value.history?,
                relationship_type: value.relationship_type?,
            })
        }
    }
    impl ::std::convert::From<super::BondContentFields> for BondContentFields {
        fn from(value: super::BondContentFields) -> Self {
            Self {
                description: Ok(value.description),
                history: Ok(value.history),
                relationship_type: Ok(value.relationship_type),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondEntity {
        components: ::std::result::Result<super::BondComponents, ::std::string::String>,
        entity_type: ::std::result::Result<super::EntityType, ::std::string::String>,
        physics_state: ::std::result::Result<super::BondPhysicsState, ::std::string::String>,
        thread_a_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        thread_b_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BondEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                physics_state: Err("no value supplied for physics_state".to_string()),
                thread_a_id: Err("no value supplied for thread_a_id".to_string()),
                thread_b_id: Err("no value supplied for thread_b_id".to_string()),
            }
        }
    }
    impl BondEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BondComponents>,
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
            T: ::std::convert::TryInto<super::BondPhysicsState>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for physics_state: {}", e));
            self
        }
        pub fn thread_a_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_a_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_a_id: {}", e));
            self
        }
        pub fn thread_b_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_b_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_b_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondEntity> for super::BondEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                entity_type: value.entity_type?,
                physics_state: value.physics_state?,
                thread_a_id: value.thread_a_id?,
                thread_b_id: value.thread_b_id?,
            })
        }
    }
    impl ::std::convert::From<super::BondEntity> for BondEntity {
        fn from(value: super::BondEntity) -> Self {
            Self {
                components: Ok(value.components),
                entity_type: Ok(value.entity_type),
                physics_state: Ok(value.physics_state),
                thread_a_id: Ok(value.thread_a_id),
                thread_b_id: Ok(value.thread_b_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondEventType {
        impact: ::std::result::Result<f64, ::std::string::String>,
        moment_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BondEventType {
        fn default() -> Self {
            Self {
                impact: Err("no value supplied for impact".to_string()),
                moment_id: Err("no value supplied for moment_id".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl BondEventType {
        pub fn impact<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.impact = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for impact: {}", e));
            self
        }
        pub fn moment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.moment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moment_id: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondEventType> for super::BondEventType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondEventType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                impact: value.impact?,
                moment_id: value.moment_id?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::BondEventType> for BondEventType {
        fn from(value: super::BondEventType) -> Self {
            Self {
                impact: Ok(value.impact),
                moment_id: Ok(value.moment_id),
                timestamp: Ok(value.timestamp),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondPhysicsState {
        tension: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        universal:
            ::std::result::Result<super::UniversalPhysicsStateComponent, ::std::string::String>,
    }
    impl ::std::default::Default for BondPhysicsState {
        fn default() -> Self {
            Self {
                tension: Err("no value supplied for tension".to_string()),
                universal: Err("no value supplied for universal".to_string()),
            }
        }
    }
    impl BondPhysicsState {
        pub fn tension<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.tension = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tension: {}", e));
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
    impl ::std::convert::TryFrom<BondPhysicsState> for super::BondPhysicsState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondPhysicsState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                tension: value.tension?,
                universal: value.universal?,
            })
        }
    }
    impl ::std::convert::From<super::BondPhysicsState> for BondPhysicsState {
        fn from(value: super::BondPhysicsState) -> Self {
            Self {
                tension: Ok(value.tension),
                universal: Ok(value.universal),
            }
        }
    }
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
