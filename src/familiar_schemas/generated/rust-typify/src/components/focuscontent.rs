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
#[doc = "The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "  \"title\": \"Base Type System\","]
#[doc = "  \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\","]
#[doc = "        \"boolean\","]
#[doc = "        \"integer\","]
#[doc = "        \"number\","]
#[doc = "        \"f32\","]
#[doc = "        \"f64\","]
#[doc = "        \"i32\","]
#[doc = "        \"i64\","]
#[doc = "        \"u32\","]
#[doc = "        \"u64\","]
#[doc = "        \"uuid\","]
#[doc = "        \"date-time\","]
#[doc = "        \"duration\","]
#[doc = "        \"object\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"$ref\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$ref\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum BaseTypeSystem {
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "string")]
    String,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "boolean")]
    Boolean,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "integer")]
    Integer,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "number")]
    Number,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "f32")]
    F32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "f64")]
    F64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "i32")]
    I32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "i64")]
    I64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "u32")]
    U32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "u64")]
    U64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "uuid")]
    Uuid,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "date-time")]
    DateTime,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "duration")]
    Duration,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "object")]
    Object,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "array")]
    Array,
    #[doc = "A complex, contrived meta-type, represented by a reference to its canonical schema file."]
    #[serde(rename = "$ref")]
    Ref(::std::string::String),
}
impl ::std::convert::From<&Self> for BaseTypeSystem {
    fn from(value: &BaseTypeSystem) -> Self {
        value.clone()
    }
}
#[doc = "A canonical definition for field validation constraints."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "  \"title\": \"Constraint Definition\","]
#[doc = "  \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"maxLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"maximum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"minLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minimum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstraintDefinition {
    #[serde(
        rename = "enum",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "maxLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<f64>,
    #[serde(
        rename = "minLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pattern: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ConstraintDefinition> for ConstraintDefinition {
    fn from(value: &ConstraintDefinition) -> Self {
        value.clone()
    }
}

impl ConstraintDefinition {
    pub fn builder() -> builder::ConstraintDefinition {
        Default::default()
    }
}
#[doc = "Defines the content and scope of a thematic goal or focus."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/FocusContent.v1.schema.json\","]
#[doc = "  \"title\": \"Focus Content Component\","]
#[doc = "  \"description\": \"Defines the content and scope of a thematic goal or focus.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"BaseFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "      \"type\": \"object\","]
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
#[doc = "  \"fields\": {"]
#[doc = "    \"end_date\": {"]
#[doc = "      \"description\": \"When this focus is scheduled to end or be reviewed.\","]
#[doc = "      \"nullable\": true,"]
#[doc = "      \"type\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"scope\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/TemporalScope.v1.json\","]
#[doc = "      \"description\": \"The temporal scope or duration of an entity or process.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Daily\","]
#[doc = "        \"Weekly\","]
#[doc = "        \"Monthly\","]
#[doc = "        \"Quarterly\","]
#[doc = "        \"Yearly\","]
#[doc = "        \"Ongoing\""]
#[doc = "      ],"]
#[doc = "      \"title\": \"Temporal Scope Field\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"start_date\": {"]
#[doc = "      \"description\": \"When this focus becomes active.\","]
#[doc = "      \"type\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"theme\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Theme.v1.json\","]
#[doc = "      \"description\": \"A concise statement of a focus, goal, or pattern.\","]
#[doc = "      \"title\": \"Theme Field\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"quantum\","]
#[doc = "    \"is_quantum\": true"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"components/FocusContent.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct FocusContentComponent {
    #[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
    pub fields: ::std::collections::HashMap<
        FocusContentComponentFieldsKey,
        FocusContentComponentFieldsValue,
    >,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&FocusContentComponent> for FocusContentComponent {
    fn from(value: &FocusContentComponent) -> Self {
        value.clone()
    }
}
impl FocusContentComponent {
    pub fn builder() -> builder::FocusContentComponent {
        Default::default()
    }
}
#[doc = "`FocusContentComponentFieldsKey`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-z_][a-z0-9_]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FocusContentComponentFieldsKey(::std::string::String);
impl ::std::ops::Deref for FocusContentComponentFieldsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FocusContentComponentFieldsKey> for ::std::string::String {
    fn from(value: FocusContentComponentFieldsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FocusContentComponentFieldsKey> for FocusContentComponentFieldsKey {
    fn from(value: &FocusContentComponentFieldsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for FocusContentComponentFieldsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-z_][a-z0-9_]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z_][a-z0-9_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FocusContentComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FocusContentComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FocusContentComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FocusContentComponentFieldsKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`FocusContentComponentFieldsValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"$ref\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$ref\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "      \"title\": \"Base Field\","]
#[doc = "      \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"description\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"constraints\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "          \"title\": \"Constraint Definition\","]
#[doc = "          \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"enum\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"maxLength\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"maximum\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"minLength\": {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            \"minimum\": {"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"pattern\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"default_value\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "          \"title\": \"Any Value\","]
#[doc = "          \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"number\","]
#[doc = "            \"integer\","]
#[doc = "            \"boolean\","]
#[doc = "            \"object\","]
#[doc = "            \"array\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"description\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "          \"title\": \"Description Field\","]
#[doc = "          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "          \"default\": \"\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "          \"title\": \"Base Type System\","]
#[doc = "          \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"string\","]
#[doc = "                \"boolean\","]
#[doc = "                \"integer\","]
#[doc = "                \"number\","]
#[doc = "                \"f32\","]
#[doc = "                \"f64\","]
#[doc = "                \"i32\","]
#[doc = "                \"i64\","]
#[doc = "                \"u32\","]
#[doc = "                \"u64\","]
#[doc = "                \"uuid\","]
#[doc = "                \"date-time\","]
#[doc = "                \"duration\","]
#[doc = "                \"object\","]
#[doc = "                \"array\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
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
#[doc = "            }"]
#[doc = "          ],"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        },"]
#[doc = "        \"ui_label\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum FocusContentComponentFieldsValue {
    Variant0 {
        #[serde(rename = "$ref")]
        ref_: ::std::string::String,
    },
    Variant1 {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        constraints: ::std::option::Option<ConstraintDefinition>,
        #[doc = "Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        default_value: ::std::option::Option<::serde_json::Value>,
        #[doc = "A canonical, reusable definition for a human-readable description field."]
        description: ::std::string::String,
        #[doc = "The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema."]
        #[serde(rename = "type")]
        type_: BaseTypeSystem,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        ui_label: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for FocusContentComponentFieldsValue {
    fn from(value: &FocusContentComponentFieldsValue) -> Self {
        value.clone()
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
    pub struct ConstraintDefinition {
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        max_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        maximum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        minimum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        pattern: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstraintDefinition {
        fn default() -> Self {
            Self {
                enum_: Ok(Default::default()),
                max_length: Ok(Default::default()),
                maximum: Ok(Default::default()),
                min_length: Ok(Default::default()),
                minimum: Ok(Default::default()),
                pattern: Ok(Default::default()),
            }
        }
    }
    impl ConstraintDefinition {
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {}", e));
            self
        }
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum: {}", e));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {}", e));
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minimum: {}", e));
            self
        }
        pub fn pattern<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pattern: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstraintDefinition> for super::ConstraintDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstraintDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enum_: value.enum_?,
                max_length: value.max_length?,
                maximum: value.maximum?,
                min_length: value.min_length?,
                minimum: value.minimum?,
                pattern: value.pattern?,
            })
        }
    }
    impl ::std::convert::From<super::ConstraintDefinition> for ConstraintDefinition {
        fn from(value: super::ConstraintDefinition) -> Self {
            Self {
                enum_: Ok(value.enum_),
                max_length: Ok(value.max_length),
                maximum: Ok(value.maximum),
                min_length: Ok(value.min_length),
                minimum: Ok(value.minimum),
                pattern: Ok(value.pattern),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FocusContentComponent {
        fields: ::std::result::Result<
            ::std::collections::HashMap<
                super::FocusContentComponentFieldsKey,
                super::FocusContentComponentFieldsValue,
            >,
            ::std::string::String,
        >,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for FocusContentComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl FocusContentComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::FocusContentComponentFieldsKey,
                    super::FocusContentComponentFieldsValue,
                >,
            >,
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
    impl ::std::convert::TryFrom<FocusContentComponent> for super::FocusContentComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FocusContentComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::FocusContentComponent> for FocusContentComponent {
        fn from(value: super::FocusContentComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
}
