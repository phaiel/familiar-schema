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
#[doc = "Defines the common physics-related properties for components and laws."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "  \"title\": \"Base Physics Properties\","]
#[doc = "  \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"engine\","]
#[doc = "    \"is_quantum\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"collapse_sensitive\": {"]
#[doc = "      \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"engine\": {"]
#[doc = "      \"description\": \"The physics engine responsible for this object.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"quantum\","]
#[doc = "        \"classical\","]
#[doc = "        \"hybrid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"is_quantum\": {"]
#[doc = "      \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BasePhysicsProperties {
    #[doc = "Indicates if the object is affected by or triggers quantum collapse."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub collapse_sensitive: ::std::option::Option<bool>,
    #[doc = "The physics engine responsible for this object."]
    pub engine: BasePhysicsPropertiesEngine,
    #[doc = "Indicates if the object has quantum properties."]
    pub is_quantum: bool,
}
impl ::std::convert::From<&BasePhysicsProperties> for BasePhysicsProperties {
    fn from(value: &BasePhysicsProperties) -> Self {
        value.clone()
    }
}
impl BasePhysicsProperties {
    pub fn builder() -> builder::BasePhysicsProperties {
        Default::default()
    }
}
#[doc = "The physics engine responsible for this object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The physics engine responsible for this object.\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"quantum\","]
#[doc = "    \"classical\","]
#[doc = "    \"hybrid\""]
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
pub enum BasePhysicsPropertiesEngine {
    #[serde(rename = "quantum")]
    Quantum,
    #[serde(rename = "classical")]
    Classical,
    #[serde(rename = "hybrid")]
    Hybrid,
}
impl ::std::convert::From<&Self> for BasePhysicsPropertiesEngine {
    fn from(value: &BasePhysicsPropertiesEngine) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BasePhysicsPropertiesEngine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Quantum => write!(f, "quantum"),
            Self::Classical => write!(f, "classical"),
            Self::Hybrid => write!(f, "hybrid"),
        }
    }
}
impl ::std::str::FromStr for BasePhysicsPropertiesEngine {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "quantum" => Ok(Self::Quantum),
            "classical" => Ok(Self::Classical),
            "hybrid" => Ok(Self::Hybrid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
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
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
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
#[doc = "      \"patternProperties\": {"]
#[doc = "        \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"$ref\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"$ref\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "              \"title\": \"Base Field\","]
#[doc = "              \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"description\","]
#[doc = "                \"type\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"constraints\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                  \"title\": \"Constraint Definition\","]
#[doc = "                  \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"enum\": {"]
#[doc = "                      \"type\": \"array\","]
#[doc = "                      \"items\": {"]
#[doc = "                        \"type\": \"string\""]
#[doc = "                      }"]
#[doc = "                    },"]
#[doc = "                    \"maxLength\": {"]
#[doc = "                      \"type\": \"integer\""]
#[doc = "                    },"]
#[doc = "                    \"maximum\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"minLength\": {"]
#[doc = "                      \"type\": \"integer\""]
#[doc = "                    },"]
#[doc = "                    \"minimum\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"pattern\": {"]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                },"]
#[doc = "                \"default_value\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                  \"title\": \"Any Value\","]
#[doc = "                  \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                  \"type\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"number\","]
#[doc = "                    \"integer\","]
#[doc = "                    \"boolean\","]
#[doc = "                    \"object\","]
#[doc = "                    \"array\","]
#[doc = "                    \"null\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"description\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                  \"title\": \"Description Field\","]
#[doc = "                  \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                  \"default\": \"\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"type\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                  \"title\": \"Base Type System\","]
#[doc = "                  \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                  \"oneOf\": ["]
#[doc = "                    {"]
#[doc = "                      \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                      \"type\": \"string\","]
#[doc = "                      \"enum\": ["]
#[doc = "                        \"string\","]
#[doc = "                        \"boolean\","]
#[doc = "                        \"integer\","]
#[doc = "                        \"number\","]
#[doc = "                        \"f32\","]
#[doc = "                        \"f64\","]
#[doc = "                        \"i32\","]
#[doc = "                        \"i64\","]
#[doc = "                        \"u32\","]
#[doc = "                        \"u64\","]
#[doc = "                        \"uuid\","]
#[doc = "                        \"date-time\","]
#[doc = "                        \"duration\","]
#[doc = "                        \"object\","]
#[doc = "                        \"array\""]
#[doc = "                      ]"]
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"$ref\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"$ref\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    }"]
#[doc = "                  ],"]
#[doc = "                  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                },"]
#[doc = "                \"ui_label\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"physics_properties\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "      \"title\": \"Base Physics Properties\","]
#[doc = "      \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"engine\","]
#[doc = "        \"is_quantum\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"collapse_sensitive\": {"]
#[doc = "          \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"engine\": {"]
#[doc = "          \"description\": \"The physics engine responsible for this object.\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"quantum\","]
#[doc = "            \"classical\","]
#[doc = "            \"hybrid\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"is_quantum\": {"]
#[doc = "          \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"components\","]
#[doc = "  \"schema_version\": \"3.0.0\","]
#[doc = "  \"source_file\": \"components/UniversalPhysicsState.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct UniversalPhysicsStateComponent {
    pub fields: UniversalPhysicsStateFields,
    pub physics_properties: BasePhysicsProperties,
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
#[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"UniversalPhysicsStateFields\","]
#[doc = "  \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
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
#[doc = "  \"patternProperties\": {"]
#[doc = "    \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"$ref\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"$ref\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "          \"title\": \"Base Field\","]
#[doc = "          \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"description\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"constraints\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "              \"title\": \"Constraint Definition\","]
#[doc = "              \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"enum\": {"]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"maxLength\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"maximum\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"minLength\": {"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                },"]
#[doc = "                \"minimum\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"pattern\": {"]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"default_value\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "              \"title\": \"Any Value\","]
#[doc = "              \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"number\","]
#[doc = "                \"integer\","]
#[doc = "                \"boolean\","]
#[doc = "                \"object\","]
#[doc = "                \"array\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "              \"title\": \"Base Type System\","]
#[doc = "              \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"boolean\","]
#[doc = "                    \"integer\","]
#[doc = "                    \"number\","]
#[doc = "                    \"f32\","]
#[doc = "                    \"f64\","]
#[doc = "                    \"i32\","]
#[doc = "                    \"i64\","]
#[doc = "                    \"u32\","]
#[doc = "                    \"u64\","]
#[doc = "                    \"uuid\","]
#[doc = "                    \"date-time\","]
#[doc = "                    \"duration\","]
#[doc = "                    \"object\","]
#[doc = "                    \"array\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"$ref\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"$ref\": {"]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"additionalProperties\": false"]
#[doc = "                }"]
#[doc = "              ],"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "            },"]
#[doc = "            \"ui_label\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        }"]
#[doc = "      ]"]
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
    pub struct BasePhysicsProperties {
        collapse_sensitive:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        engine: ::std::result::Result<super::BasePhysicsPropertiesEngine, ::std::string::String>,
        is_quantum: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for BasePhysicsProperties {
        fn default() -> Self {
            Self {
                collapse_sensitive: Ok(Default::default()),
                engine: Err("no value supplied for engine".to_string()),
                is_quantum: Err("no value supplied for is_quantum".to_string()),
            }
        }
    }
    impl BasePhysicsProperties {
        pub fn collapse_sensitive<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.collapse_sensitive = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for collapse_sensitive: {}",
                    e
                )
            });
            self
        }
        pub fn engine<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsPropertiesEngine>,
            T::Error: ::std::fmt::Display,
        {
            self.engine = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for engine: {}", e));
            self
        }
        pub fn is_quantum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_quantum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_quantum: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BasePhysicsProperties> for super::BasePhysicsProperties {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BasePhysicsProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                collapse_sensitive: value.collapse_sensitive?,
                engine: value.engine?,
                is_quantum: value.is_quantum?,
            })
        }
    }
    impl ::std::convert::From<super::BasePhysicsProperties> for BasePhysicsProperties {
        fn from(value: super::BasePhysicsProperties) -> Self {
            Self {
                collapse_sensitive: Ok(value.collapse_sensitive),
                engine: Ok(value.engine),
                is_quantum: Ok(value.is_quantum),
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
        fields: ::std::result::Result<super::UniversalPhysicsStateFields, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for UniversalPhysicsStateComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl UniversalPhysicsStateComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::UniversalPhysicsStateFields>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
        pub fn physics_properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsProperties>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_properties = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for physics_properties: {}",
                    e
                )
            });
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
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::UniversalPhysicsStateComponent>
        for UniversalPhysicsStateComponent
    {
        fn from(value: super::UniversalPhysicsStateComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
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
