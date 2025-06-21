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
#[doc = "Defines the emergent pattern content of a Motif entity. This is a quantum component."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/MotifContent.v1.schema.json\","]
#[doc = "  \"title\": \"Motif Content Component\","]
#[doc = "  \"description\": \"Defines the emergent pattern content of a Motif entity. This is a quantum component.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"MotifContentFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"motif_description\","]
#[doc = "        \"motif_type\","]
#[doc = "        \"source_entanglements\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"motif_description\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"motif_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/physics/MotifType.v1.json\","]
#[doc = "          \"title\": \"Motif Type\","]
#[doc = "          \"description\": \"A canonical enum of the types of recurring themes or motifs in memory.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Behavioral\","]
#[doc = "            \"Emotional\","]
#[doc = "            \"Situational\","]
#[doc = "            \"Temporal\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source_entanglements\": {"]
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
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"quantum\","]
#[doc = "    \"is_quantum\": true"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.4.0\","]
#[doc = "  \"source_file\": \"components/MotifContent.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MotifContentComponent {
    pub fields: MotifContentFields,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&MotifContentComponent> for MotifContentComponent {
    fn from(value: &MotifContentComponent) -> Self {
        value.clone()
    }
}
impl MotifContentComponent {
    pub fn builder() -> builder::MotifContentComponent {
        Default::default()
    }
}
#[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MotifContentFields\","]
#[doc = "  \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"motif_description\","]
#[doc = "    \"motif_type\","]
#[doc = "    \"source_entanglements\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"motif_description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"motif_type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/physics/MotifType.v1.json\","]
#[doc = "      \"title\": \"Motif Type\","]
#[doc = "      \"description\": \"A canonical enum of the types of recurring themes or motifs in memory.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Behavioral\","]
#[doc = "        \"Emotional\","]
#[doc = "        \"Situational\","]
#[doc = "        \"Temporal\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_entanglements\": {"]
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
pub struct MotifContentFields {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub motif_description: ::std::string::String,
    #[doc = "A canonical enum of the types of recurring themes or motifs in memory."]
    pub motif_type: MotifType,
    #[doc = "A canonical definition for a list of unique entity identifiers (UUIDs)."]
    pub source_entanglements: Vec<::std::string::String>,
}
impl ::std::convert::From<&MotifContentFields> for MotifContentFields {
    fn from(value: &MotifContentFields) -> Self {
        value.clone()
    }
}
impl MotifContentFields {
    pub fn builder() -> builder::MotifContentFields {
        Default::default()
    }
}
#[doc = "A canonical enum of the types of recurring themes or motifs in memory."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/physics/MotifType.v1.json\","]
#[doc = "  \"title\": \"Motif Type\","]
#[doc = "  \"description\": \"A canonical enum of the types of recurring themes or motifs in memory.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Behavioral\","]
#[doc = "    \"Emotional\","]
#[doc = "    \"Situational\","]
#[doc = "    \"Temporal\""]
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
pub enum MotifType {
    Behavioral,
    Emotional,
    Situational,
    Temporal,
}
impl ::std::convert::From<&Self> for MotifType {
    fn from(value: &MotifType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for MotifType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Behavioral => write!(f, "Behavioral"),
            Self::Emotional => write!(f, "Emotional"),
            Self::Situational => write!(f, "Situational"),
            Self::Temporal => write!(f, "Temporal"),
        }
    }
}
impl ::std::str::FromStr for MotifType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Behavioral" => Ok(Self::Behavioral),
            "Emotional" => Ok(Self::Emotional),
            "Situational" => Ok(Self::Situational),
            "Temporal" => Ok(Self::Temporal),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MotifType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MotifType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MotifType {
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
    pub struct MotifContentComponent {
        fields: ::std::result::Result<super::MotifContentFields, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for MotifContentComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl MotifContentComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MotifContentFields>,
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
    impl ::std::convert::TryFrom<MotifContentComponent> for super::MotifContentComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MotifContentComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::MotifContentComponent> for MotifContentComponent {
        fn from(value: super::MotifContentComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MotifContentFields {
        motif_description: ::std::result::Result<::std::string::String, ::std::string::String>,
        motif_type: ::std::result::Result<super::MotifType, ::std::string::String>,
        source_entanglements:
            ::std::result::Result<Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for MotifContentFields {
        fn default() -> Self {
            Self {
                motif_description: Err("no value supplied for motif_description".to_string()),
                motif_type: Err("no value supplied for motif_type".to_string()),
                source_entanglements: Err("no value supplied for source_entanglements".to_string()),
            }
        }
    }
    impl MotifContentFields {
        pub fn motif_description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.motif_description = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for motif_description: {}",
                    e
                )
            });
            self
        }
        pub fn motif_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MotifType>,
            T::Error: ::std::fmt::Display,
        {
            self.motif_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for motif_type: {}", e));
            self
        }
        pub fn source_entanglements<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.source_entanglements = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_entanglements: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<MotifContentFields> for super::MotifContentFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MotifContentFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                motif_description: value.motif_description?,
                motif_type: value.motif_type?,
                source_entanglements: value.source_entanglements?,
            })
        }
    }
    impl ::std::convert::From<super::MotifContentFields> for MotifContentFields {
        fn from(value: super::MotifContentFields) -> Self {
            Self {
                motif_description: Ok(value.motif_description),
                motif_type: Ok(value.motif_type),
                source_entanglements: Ok(value.source_entanglements),
            }
        }
    }
}
