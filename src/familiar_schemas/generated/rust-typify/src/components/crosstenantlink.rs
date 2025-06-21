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
#[doc = "A canonical enum of permissions related to a Bond."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/security/BondPermission.v1.json\","]
#[doc = "  \"title\": \"Bond Permission\","]
#[doc = "  \"description\": \"A canonical enum of permissions related to a Bond.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Read\","]
#[doc = "    \"Write\","]
#[doc = "    \"Delete\","]
#[doc = "    \"Share\""]
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
pub enum BondPermission {
    Read,
    Write,
    Delete,
    Share,
}
impl ::std::convert::From<&Self> for BondPermission {
    fn from(value: &BondPermission) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BondPermission {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Read => write!(f, "Read"),
            Self::Write => write!(f, "Write"),
            Self::Delete => write!(f, "Delete"),
            Self::Share => write!(f, "Share"),
        }
    }
}
impl ::std::str::FromStr for BondPermission {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Read" => Ok(Self::Read),
            "Write" => Ok(Self::Write),
            "Delete" => Ok(Self::Delete),
            "Share" => Ok(Self::Share),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BondPermission {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BondPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BondPermission {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Manages a link to a Thread in another tenant, enabling federated relationships."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/CrossTenantLink.v1.schema.json\","]
#[doc = "  \"title\": \"Cross-Tenant Link Component\","]
#[doc = "  \"description\": \"Manages a link to a Thread in another tenant, enabling federated relationships.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"Cross-tenantLinkFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"link_status\","]
#[doc = "        \"remote_tenant_id\","]
#[doc = "        \"remote_thread_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"link_status\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/system/LinkStatus.v1.json\","]
#[doc = "          \"title\": \"Link Status\","]
#[doc = "          \"description\": \"A canonical enum for the status of a cross-tenant link.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"PendingApproval\","]
#[doc = "            \"Approved\","]
#[doc = "            \"Revoked\","]
#[doc = "            \"Declined\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"permissions_granted\": {"]
#[doc = "          \"description\": \"A list of permissions granted by the remote user for this link.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/security/BondPermission.v1.json\","]
#[doc = "            \"title\": \"Bond Permission\","]
#[doc = "            \"description\": \"A canonical enum of permissions related to a Bond.\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"Read\","]
#[doc = "              \"Write\","]
#[doc = "              \"Delete\","]
#[doc = "              \"Share\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"remote_tenant_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"remote_thread_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
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
#[doc = "  \"source_file\": \"components/CrossTenantLink.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CrossTenantLinkComponent {
    pub fields: CrossTenantLinkFields,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&CrossTenantLinkComponent> for CrossTenantLinkComponent {
    fn from(value: &CrossTenantLinkComponent) -> Self {
        value.clone()
    }
}
impl CrossTenantLinkComponent {
    pub fn builder() -> builder::CrossTenantLinkComponent {
        Default::default()
    }
}
#[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Cross-tenantLinkFields\","]
#[doc = "  \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"link_status\","]
#[doc = "    \"remote_tenant_id\","]
#[doc = "    \"remote_thread_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"link_status\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/system/LinkStatus.v1.json\","]
#[doc = "      \"title\": \"Link Status\","]
#[doc = "      \"description\": \"A canonical enum for the status of a cross-tenant link.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"PendingApproval\","]
#[doc = "        \"Approved\","]
#[doc = "        \"Revoked\","]
#[doc = "        \"Declined\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"permissions_granted\": {"]
#[doc = "      \"description\": \"A list of permissions granted by the remote user for this link.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/security/BondPermission.v1.json\","]
#[doc = "        \"title\": \"Bond Permission\","]
#[doc = "        \"description\": \"A canonical enum of permissions related to a Bond.\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"Read\","]
#[doc = "          \"Write\","]
#[doc = "          \"Delete\","]
#[doc = "          \"Share\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"remote_tenant_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"remote_thread_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
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
pub struct CrossTenantLinkFields {
    #[doc = "A canonical enum for the status of a cross-tenant link."]
    pub link_status: LinkStatus,
    #[doc = "A list of permissions granted by the remote user for this link."]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub permissions_granted: ::std::vec::Vec<BondPermission>,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub remote_tenant_id: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub remote_thread_id: ::std::string::String,
}
impl ::std::convert::From<&CrossTenantLinkFields> for CrossTenantLinkFields {
    fn from(value: &CrossTenantLinkFields) -> Self {
        value.clone()
    }
}
impl CrossTenantLinkFields {
    pub fn builder() -> builder::CrossTenantLinkFields {
        Default::default()
    }
}
#[doc = "A canonical enum for the status of a cross-tenant link."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/system/LinkStatus.v1.json\","]
#[doc = "  \"title\": \"Link Status\","]
#[doc = "  \"description\": \"A canonical enum for the status of a cross-tenant link.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"PendingApproval\","]
#[doc = "    \"Approved\","]
#[doc = "    \"Revoked\","]
#[doc = "    \"Declined\""]
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
pub enum LinkStatus {
    PendingApproval,
    Approved,
    Revoked,
    Declined,
}
impl ::std::convert::From<&Self> for LinkStatus {
    fn from(value: &LinkStatus) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for LinkStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::PendingApproval => write!(f, "PendingApproval"),
            Self::Approved => write!(f, "Approved"),
            Self::Revoked => write!(f, "Revoked"),
            Self::Declined => write!(f, "Declined"),
        }
    }
}
impl ::std::str::FromStr for LinkStatus {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "PendingApproval" => Ok(Self::PendingApproval),
            "Approved" => Ok(Self::Approved),
            "Revoked" => Ok(Self::Revoked),
            "Declined" => Ok(Self::Declined),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for LinkStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for LinkStatus {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for LinkStatus {
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
    pub struct CrossTenantLinkComponent {
        fields: ::std::result::Result<super::CrossTenantLinkFields, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for CrossTenantLinkComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl CrossTenantLinkComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CrossTenantLinkFields>,
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
    impl ::std::convert::TryFrom<CrossTenantLinkComponent> for super::CrossTenantLinkComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CrossTenantLinkComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::CrossTenantLinkComponent> for CrossTenantLinkComponent {
        fn from(value: super::CrossTenantLinkComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CrossTenantLinkFields {
        link_status: ::std::result::Result<super::LinkStatus, ::std::string::String>,
        permissions_granted:
            ::std::result::Result<::std::vec::Vec<super::BondPermission>, ::std::string::String>,
        remote_tenant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        remote_thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CrossTenantLinkFields {
        fn default() -> Self {
            Self {
                link_status: Err("no value supplied for link_status".to_string()),
                permissions_granted: Ok(Default::default()),
                remote_tenant_id: Err("no value supplied for remote_tenant_id".to_string()),
                remote_thread_id: Err("no value supplied for remote_thread_id".to_string()),
            }
        }
    }
    impl CrossTenantLinkFields {
        pub fn link_status<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::LinkStatus>,
            T::Error: ::std::fmt::Display,
        {
            self.link_status = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_status: {}", e));
            self
        }
        pub fn permissions_granted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BondPermission>>,
            T::Error: ::std::fmt::Display,
        {
            self.permissions_granted = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for permissions_granted: {}",
                    e
                )
            });
            self
        }
        pub fn remote_tenant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_tenant_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for remote_tenant_id: {}",
                    e
                )
            });
            self
        }
        pub fn remote_thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_thread_id = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for remote_thread_id: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CrossTenantLinkFields> for super::CrossTenantLinkFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CrossTenantLinkFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                link_status: value.link_status?,
                permissions_granted: value.permissions_granted?,
                remote_tenant_id: value.remote_tenant_id?,
                remote_thread_id: value.remote_thread_id?,
            })
        }
    }
    impl ::std::convert::From<super::CrossTenantLinkFields> for CrossTenantLinkFields {
        fn from(value: super::CrossTenantLinkFields) -> Self {
            Self {
                link_status: Ok(value.link_status),
                permissions_granted: Ok(value.permissions_granted),
                remote_tenant_id: Ok(value.remote_tenant_id),
                remote_thread_id: Ok(value.remote_thread_id),
            }
        }
    }
}
