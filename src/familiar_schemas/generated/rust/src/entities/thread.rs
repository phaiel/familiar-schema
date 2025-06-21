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
#[doc = "`GenericThreadPhysicsState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GenericThreadPhysicsState\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"universal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
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
pub struct GenericThreadPhysicsState {
    pub universal: UniversalPhysicsStateComponent,
}
impl ::std::convert::From<&GenericThreadPhysicsState> for GenericThreadPhysicsState {
    fn from(value: &GenericThreadPhysicsState) -> Self {
        value.clone()
    }
}
impl GenericThreadPhysicsState {
    pub fn builder() -> builder::GenericThreadPhysicsState {
        Default::default()
    }
}
#[doc = "`PersonThreadComponents`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PersonThreadComponents\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"cognitive_baseline\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"cognitive_baseline\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/CognitiveBaseline.v1.schema.json\","]
#[doc = "      \"title\": \"Cognitive Baseline Component\","]
#[doc = "      \"description\": \"Defines the innate 'personality' or temperament of a Thread.\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"baseline\": {"]
#[doc = "          \"description\": \"Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"bond_damping_factor\": {"]
#[doc = "              \"description\": \"How resistant their bonds are to change. (UI Label: Relationship Stability)\","]
#[doc = "              \"maximum\": 2.0,"]
#[doc = "              \"minimum\": 0.5,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"consolidation_rate_modifier\": {"]
#[doc = "              \"description\": \"How quickly their memories become stable. (UI Label: Memory Consolidation)\","]
#[doc = "              \"maximum\": 1.5,"]
#[doc = "              \"minimum\": 0.8,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"emotional_volatility\": {"]
#[doc = "              \"description\": \"How strongly emotional events affect this person. (UI Label: Emotional Reactivity)\","]
#[doc = "              \"maximum\": 2.0,"]
#[doc = "              \"minimum\": 0.5,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"exploration_bias\": {"]
#[doc = "              \"description\": \"Tendency for creative, associative thinking. (UI Label: Openness to Experience)\","]
#[doc = "              \"maximum\": 1.3,"]
#[doc = "              \"minimum\": 0.7,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"social_energy_factor\": {"]
#[doc = "              \"description\": \"How energized they are by social interactions. (UI Label: Social Energy)\","]
#[doc = "              \"maximum\": 1.2,"]
#[doc = "              \"minimum\": 0.8,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"required\": ["]
#[doc = "            \"emotional_volatility\","]
#[doc = "            \"social_energy_factor\","]
#[doc = "            \"consolidation_rate_modifier\","]
#[doc = "            \"bond_damping_factor\","]
#[doc = "            \"exploration_bias\""]
#[doc = "          ],"]
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
pub struct PersonThreadComponents {
    #[doc = "Defines the innate 'personality' or temperament of a Thread."]
    pub cognitive_baseline: ::serde_json::Value,
}
impl ::std::convert::From<&PersonThreadComponents> for PersonThreadComponents {
    fn from(value: &PersonThreadComponents) -> Self {
        value.clone()
    }
}
impl PersonThreadComponents {
    pub fn builder() -> builder::PersonThreadComponents {
        Default::default()
    }
}
#[doc = "`PersonThreadPhysicsState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"PersonThreadPhysicsState\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"universal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
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
pub struct PersonThreadPhysicsState {
    pub universal: UniversalPhysicsStateComponent,
}
impl ::std::convert::From<&PersonThreadPhysicsState> for PersonThreadPhysicsState {
    fn from(value: &PersonThreadPhysicsState) -> Self {
        value.clone()
    }
}
impl PersonThreadPhysicsState {
    pub fn builder() -> builder::PersonThreadPhysicsState {
        Default::default()
    }
}
#[doc = "A polymorphic Thread entity that can be either a PersonThread or a GenericThread."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/thread.v3.schema.json\","]
#[doc = "  \"title\": \"Thread\","]
#[doc = "  \"description\": \"A polymorphic Thread entity that can be either a PersonThread or a GenericThread.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/entities/PersonThread.v1.schema.json\","]
#[doc = "      \"title\": \"Person Thread\","]
#[doc = "      \"description\": \"A Thread representing a person, which MUST have a CognitiveBaseline.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"components\","]
#[doc = "        \"identity\","]
#[doc = "        \"physics_state\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"components\": {"]
#[doc = "          \"title\": \"PersonThreadComponents\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"cognitive_baseline\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"cognitive_baseline\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/components/CognitiveBaseline.v1.schema.json\","]
#[doc = "              \"title\": \"Cognitive Baseline Component\","]
#[doc = "              \"description\": \"Defines the innate 'personality' or temperament of a Thread.\","]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "              \"fields\": {"]
#[doc = "                \"baseline\": {"]
#[doc = "                  \"description\": \"Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.\","]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"bond_damping_factor\": {"]
#[doc = "                      \"description\": \"How resistant their bonds are to change. (UI Label: Relationship Stability)\","]
#[doc = "                      \"maximum\": 2.0,"]
#[doc = "                      \"minimum\": 0.5,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"consolidation_rate_modifier\": {"]
#[doc = "                      \"description\": \"How quickly their memories become stable. (UI Label: Memory Consolidation)\","]
#[doc = "                      \"maximum\": 1.5,"]
#[doc = "                      \"minimum\": 0.8,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"emotional_volatility\": {"]
#[doc = "                      \"description\": \"How strongly emotional events affect this person. (UI Label: Emotional Reactivity)\","]
#[doc = "                      \"maximum\": 2.0,"]
#[doc = "                      \"minimum\": 0.5,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"exploration_bias\": {"]
#[doc = "                      \"description\": \"Tendency for creative, associative thinking. (UI Label: Openness to Experience)\","]
#[doc = "                      \"maximum\": 1.3,"]
#[doc = "                      \"minimum\": 0.7,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"social_energy_factor\": {"]
#[doc = "                      \"description\": \"How energized they are by social interactions. (UI Label: Social Energy)\","]
#[doc = "                      \"maximum\": 1.2,"]
#[doc = "                      \"minimum\": 0.8,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"required\": ["]
#[doc = "                    \"emotional_volatility\","]
#[doc = "                    \"social_energy_factor\","]
#[doc = "                    \"consolidation_rate_modifier\","]
#[doc = "                    \"bond_damping_factor\","]
#[doc = "                    \"exploration_bias\""]
#[doc = "                  ],"]
#[doc = "                  \"type\": \"object\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"physics_properties\": {"]
#[doc = "                \"engine\": \"classical\","]
#[doc = "                \"is_quantum\": false"]
#[doc = "              },"]
#[doc = "              \"schema_version\": \"1.0.0\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"identity\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/ThreadContent.v1.schema.json\","]
#[doc = "          \"title\": \"Thread Content Component\","]
#[doc = "          \"description\": \"Defines the core, immutable content of a Thread entity, such as its name and type.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"aliases\": {"]
#[doc = "              \"description\": \"A list of alternative names for this Thread (e.g., nicknames).\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"name\": {"]
#[doc = "              \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "              \"title\": \"Name Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"thread_type\": {"]
#[doc = "              \"description\": \"The Platonic Form of the Thread, enforcing abstract relationships.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Person\","]
#[doc = "                \"Place\","]
#[doc = "                \"Concept\","]
#[doc = "                \"GenericObject\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.2.0\""]
#[doc = "        },"]
#[doc = "        \"physics_state\": {"]
#[doc = "          \"title\": \"PersonThreadPhysicsState\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"universal\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"universal\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\","]
#[doc = "              \"title\": \"Universal Physics State Component\","]
#[doc = "              \"description\": \"The transient, mutable physics state of an entity.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"fields\": {"]
#[doc = "                  \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"cognitive_perspective\","]
#[doc = "                    \"consolidation_rate\","]
#[doc = "                    \"decay_rate\","]
#[doc = "                    \"energy\","]
#[doc = "                    \"entanglement_strength\","]
#[doc = "                    \"momentum\","]
#[doc = "                    \"quantum_coherence\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"cognitive_perspective\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "                      \"title\": \"Cognitive Perspective Field\","]
#[doc = "                      \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "                      \"ui_label\": \"Cognitive Perspective\""]
#[doc = "                    },"]
#[doc = "                    \"consolidation_rate\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "                      \"title\": \"Consolidation Rate Field\","]
#[doc = "                      \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"value\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"value\": {"]
#[doc = "                          \"default\": 0.0,"]
#[doc = "                          \"type\": \"number\","]
#[doc = "                          \"format\": \"double\","]
#[doc = "                          \"maximum\": 1.0,"]
#[doc = "                          \"minimum\": 0.0,"]
#[doc = "                          \"x-python-type\": \"float\","]
#[doc = "                          \"x-rust-type\": \"f64\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"decay_rate\": {"]
#[doc = "                      \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"value\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"value\": {"]
#[doc = "                          \"default\": 0.01,"]
#[doc = "                          \"type\": \"number\","]
#[doc = "                          \"format\": \"double\","]
#[doc = "                          \"x-python-type\": \"float\","]
#[doc = "                          \"x-rust-type\": \"f64\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"energy\": {"]
#[doc = "                      \"description\": \"The current energy level of an entity.\","]
#[doc = "                      \"default\": 0.1,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"entanglement_strength\": {"]
#[doc = "                      \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "                      \"default\": null,"]
#[doc = "                      \"type\": ["]
#[doc = "                        \"number\","]
#[doc = "                        \"null\""]
#[doc = "                      ],"]
#[doc = "                      \"maximum\": 1.0,"]
#[doc = "                      \"minimum\": 0.0"]
#[doc = "                    },"]
#[doc = "                    \"momentum\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "                      \"title\": \"6D Physics Vector\","]
#[doc = "                      \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"number\""]
#[doc = "                      },"]
#[doc = "                      \"maxItems\": 6,"]
#[doc = "                      \"minItems\": 6"]
#[doc = "                    },"]
#[doc = "                    \"quantum_coherence\": {"]
#[doc = "                      \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "                      \"default\": null,"]
#[doc = "                      \"type\": ["]
#[doc = "                        \"number\","]
#[doc = "                        \"null\""]
#[doc = "                      ],"]
#[doc = "                      \"maximum\": 1.0,"]
#[doc = "                      \"minimum\": 0.0"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "              \"schema_version\": \"3.0.0\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/entities/GenericThread.v1.schema.json\","]
#[doc = "      \"title\": \"Generic Thread\","]
#[doc = "      \"description\": \"A Thread representing a Place, Concept, or GenericObject, which MUST NOT have a CognitiveBaseline.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"identity\","]
#[doc = "        \"physics_state\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"identity\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/ThreadContent.v1.schema.json\","]
#[doc = "          \"title\": \"Thread Content Component\","]
#[doc = "          \"description\": \"Defines the core, immutable content of a Thread entity, such as its name and type.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"aliases\": {"]
#[doc = "              \"description\": \"A list of alternative names for this Thread (e.g., nicknames).\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"name\": {"]
#[doc = "              \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "              \"title\": \"Name Field\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"thread_type\": {"]
#[doc = "              \"description\": \"The Platonic Form of the Thread, enforcing abstract relationships.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Person\","]
#[doc = "                \"Place\","]
#[doc = "                \"Concept\","]
#[doc = "                \"GenericObject\""]
#[doc = "              ],"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.2.0\""]
#[doc = "        },"]
#[doc = "        \"physics_state\": {"]
#[doc = "          \"title\": \"GenericThreadPhysicsState\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"universal\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"universal\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\","]
#[doc = "              \"title\": \"Universal Physics State Component\","]
#[doc = "              \"description\": \"The transient, mutable physics state of an entity.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"fields\": {"]
#[doc = "                  \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"cognitive_perspective\","]
#[doc = "                    \"consolidation_rate\","]
#[doc = "                    \"decay_rate\","]
#[doc = "                    \"energy\","]
#[doc = "                    \"entanglement_strength\","]
#[doc = "                    \"momentum\","]
#[doc = "                    \"quantum_coherence\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"cognitive_perspective\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/CognitivePerspective.v1.schema.json\","]
#[doc = "                      \"title\": \"Cognitive Perspective Field\","]
#[doc = "                      \"description\": \"The intrinsic 'spin' or 'flavor' of the entity, which generates cognitive dissonance (torsion).\","]
#[doc = "                      \"ui_label\": \"Cognitive Perspective\""]
#[doc = "                    },"]
#[doc = "                    \"consolidation_rate\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/snippets/fields/ConsolidationRate.v1.json\","]
#[doc = "                      \"title\": \"Consolidation Rate Field\","]
#[doc = "                      \"description\": \"The base rate of memory consolidation for an entity, before multipliers are applied. Represents how quickly an unstable memory becomes stable.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"value\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"value\": {"]
#[doc = "                          \"default\": 0.0,"]
#[doc = "                          \"type\": \"number\","]
#[doc = "                          \"format\": \"double\","]
#[doc = "                          \"maximum\": 1.0,"]
#[doc = "                          \"minimum\": 0.0,"]
#[doc = "                          \"x-python-type\": \"float\","]
#[doc = "                          \"x-rust-type\": \"f64\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"decay_rate\": {"]
#[doc = "                      \"description\": \"The base rate of energy or coherence decay for an entity, before multipliers are applied.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"value\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"value\": {"]
#[doc = "                          \"default\": 0.01,"]
#[doc = "                          \"type\": \"number\","]
#[doc = "                          \"format\": \"double\","]
#[doc = "                          \"x-python-type\": \"float\","]
#[doc = "                          \"x-rust-type\": \"f64\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"energy\": {"]
#[doc = "                      \"description\": \"The current energy level of an entity.\","]
#[doc = "                      \"default\": 0.1,"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"entanglement_strength\": {"]
#[doc = "                      \"description\": \"The overall entanglement strength of this entity with others. Null for classical entities.\","]
#[doc = "                      \"default\": null,"]
#[doc = "                      \"type\": ["]
#[doc = "                        \"number\","]
#[doc = "                        \"null\""]
#[doc = "                      ],"]
#[doc = "                      \"maximum\": 1.0,"]
#[doc = "                      \"minimum\": 0.0"]
#[doc = "                    },"]
#[doc = "                    \"momentum\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "                      \"title\": \"6D Physics Vector\","]
#[doc = "                      \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"number\""]
#[doc = "                      },"]
#[doc = "                      \"maxItems\": 6,"]
#[doc = "                      \"minItems\": 6"]
#[doc = "                    },"]
#[doc = "                    \"quantum_coherence\": {"]
#[doc = "                      \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "                      \"default\": null,"]
#[doc = "                      \"type\": ["]
#[doc = "                        \"number\","]
#[doc = "                        \"null\""]
#[doc = "                      ],"]
#[doc = "                      \"maximum\": 1.0,"]
#[doc = "                      \"minimum\": 0.0"]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "              \"schema_version\": \"3.0.0\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"source_file\": \"entities/Thread.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Thread {
    PersonThread {
        components: PersonThreadComponents,
        #[doc = "Defines the core, immutable content of a Thread entity, such as its name and type."]
        identity: ::serde_json::Value,
        physics_state: PersonThreadPhysicsState,
    },
    GenericThread {
        #[doc = "Defines the core, immutable content of a Thread entity, such as its name and type."]
        identity: ::serde_json::Value,
        physics_state: GenericThreadPhysicsState,
    },
}
impl ::std::convert::From<&Self> for Thread {
    fn from(value: &Thread) -> Self {
        value.clone()
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
    pub struct GenericThreadPhysicsState {
        universal:
            ::std::result::Result<super::UniversalPhysicsStateComponent, ::std::string::String>,
    }
    impl ::std::default::Default for GenericThreadPhysicsState {
        fn default() -> Self {
            Self {
                universal: Err("no value supplied for universal".to_string()),
            }
        }
    }
    impl GenericThreadPhysicsState {
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
    impl ::std::convert::TryFrom<GenericThreadPhysicsState> for super::GenericThreadPhysicsState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: GenericThreadPhysicsState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                universal: value.universal?,
            })
        }
    }
    impl ::std::convert::From<super::GenericThreadPhysicsState> for GenericThreadPhysicsState {
        fn from(value: super::GenericThreadPhysicsState) -> Self {
            Self {
                universal: Ok(value.universal),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PersonThreadComponents {
        cognitive_baseline: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for PersonThreadComponents {
        fn default() -> Self {
            Self {
                cognitive_baseline: Err("no value supplied for cognitive_baseline".to_string()),
            }
        }
    }
    impl PersonThreadComponents {
        pub fn cognitive_baseline<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.cognitive_baseline = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for cognitive_baseline: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<PersonThreadComponents> for super::PersonThreadComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PersonThreadComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                cognitive_baseline: value.cognitive_baseline?,
            })
        }
    }
    impl ::std::convert::From<super::PersonThreadComponents> for PersonThreadComponents {
        fn from(value: super::PersonThreadComponents) -> Self {
            Self {
                cognitive_baseline: Ok(value.cognitive_baseline),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PersonThreadPhysicsState {
        universal:
            ::std::result::Result<super::UniversalPhysicsStateComponent, ::std::string::String>,
    }
    impl ::std::default::Default for PersonThreadPhysicsState {
        fn default() -> Self {
            Self {
                universal: Err("no value supplied for universal".to_string()),
            }
        }
    }
    impl PersonThreadPhysicsState {
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
    impl ::std::convert::TryFrom<PersonThreadPhysicsState> for super::PersonThreadPhysicsState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PersonThreadPhysicsState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                universal: value.universal?,
            })
        }
    }
    impl ::std::convert::From<super::PersonThreadPhysicsState> for PersonThreadPhysicsState {
        fn from(value: super::PersonThreadPhysicsState) -> Self {
            Self {
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
