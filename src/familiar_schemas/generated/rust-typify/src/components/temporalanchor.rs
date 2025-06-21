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
#[doc = "Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Block Universe."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/TemporalAnchor.v1.schema.json\","]
#[doc = "  \"title\": \"Temporal Anchor Component\","]
#[doc = "  \"description\": \"Provides an immutable, fixed position in time for a Moment entity, anchoring it to the Growing Block Universe.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"TemporalAnchorFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event_timestamp\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"duration_seconds\": {"]
#[doc = "          \"description\": \"The duration of the event in seconds, if applicable.\","]
#[doc = "          \"default\": 0,"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"event_timestamp\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "          \"title\": \"Timestamp\","]
#[doc = "          \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "          \"type\": \"string\""]
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
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"components/TemporalAnchor.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TemporalAnchorComponent {
    pub fields: TemporalAnchorFields,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&TemporalAnchorComponent> for TemporalAnchorComponent {
    fn from(value: &TemporalAnchorComponent) -> Self {
        value.clone()
    }
}
impl TemporalAnchorComponent {
    pub fn builder() -> builder::TemporalAnchorComponent {
        Default::default()
    }
}
#[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TemporalAnchorFields\","]
#[doc = "  \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event_timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"duration_seconds\": {"]
#[doc = "      \"description\": \"The duration of the event in seconds, if applicable.\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"event_timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
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
pub struct TemporalAnchorFields {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub duration_seconds: ::std::option::Option<f64>,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub event_timestamp: ::std::string::String,
}
impl ::std::convert::From<&TemporalAnchorFields> for TemporalAnchorFields {
    fn from(value: &TemporalAnchorFields) -> Self {
        value.clone()
    }
}
impl TemporalAnchorFields {
    pub fn builder() -> builder::TemporalAnchorFields {
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
    pub struct TemporalAnchorComponent {
        fields: ::std::result::Result<super::TemporalAnchorFields, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for TemporalAnchorComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl TemporalAnchorComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TemporalAnchorFields>,
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
    impl ::std::convert::TryFrom<TemporalAnchorComponent> for super::TemporalAnchorComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TemporalAnchorComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::TemporalAnchorComponent> for TemporalAnchorComponent {
        fn from(value: super::TemporalAnchorComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TemporalAnchorFields {
        duration_seconds: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        event_timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TemporalAnchorFields {
        fn default() -> Self {
            Self {
                duration_seconds: Ok(Default::default()),
                event_timestamp: Err("no value supplied for event_timestamp".to_string()),
            }
        }
    }
    impl TemporalAnchorFields {
        pub fn duration_seconds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.duration_seconds = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for duration_seconds: {}",
                    e
                )
            });
            self
        }
        pub fn event_timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TemporalAnchorFields> for super::TemporalAnchorFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TemporalAnchorFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                duration_seconds: value.duration_seconds?,
                event_timestamp: value.event_timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::TemporalAnchorFields> for TemporalAnchorFields {
        fn from(value: super::TemporalAnchorFields) -> Self {
            Self {
                duration_seconds: Ok(value.duration_seconds),
                event_timestamp: Ok(value.event_timestamp),
            }
        }
    }
}
