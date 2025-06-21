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
impl ::std::default::Default for ConstraintDefinition {
    fn default() -> Self {
        Self {
            enum_: Default::default(),
            max_length: Default::default(),
            maximum: Default::default(),
            min_length: Default::default(),
            minimum: Default::default(),
            pattern: Default::default(),
        }
    }
}
impl ConstraintDefinition {
    pub fn builder() -> builder::ConstraintDefinition {
        Default::default()
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
impl ::std::default::Default for StatusField {
    fn default() -> Self {
        StatusField::Pending
    }
}
#[doc = "A canonical enum of all available subscription plans for a Tenant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/system/SubscriptionPlan.v1.json\","]
#[doc = "  \"title\": \"Subscription Plan\","]
#[doc = "  \"description\": \"A canonical enum of all available subscription plans for a Tenant.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Free\","]
#[doc = "    \"Personal\","]
#[doc = "    \"FamilyPlus\","]
#[doc = "    \"Enterprise\""]
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
pub enum SubscriptionPlan {
    Free,
    Personal,
    FamilyPlus,
    Enterprise,
}
impl ::std::convert::From<&Self> for SubscriptionPlan {
    fn from(value: &SubscriptionPlan) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubscriptionPlan {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Free => write!(f, "Free"),
            Self::Personal => write!(f, "Personal"),
            Self::FamilyPlus => write!(f, "FamilyPlus"),
            Self::Enterprise => write!(f, "Enterprise"),
        }
    }
}
impl ::std::str::FromStr for SubscriptionPlan {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Free" => Ok(Self::Free),
            "Personal" => Ok(Self::Personal),
            "FamilyPlus" => Ok(Self::FamilyPlus),
            "Enterprise" => Ok(Self::Enterprise),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubscriptionPlan {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SubscriptionPlan {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SubscriptionPlan {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Stores tenant-specific settings and feature flag overrides."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/TenantConfig.v1.schema.json\","]
#[doc = "  \"title\": \"Tenant Configuration Component\","]
#[doc = "  \"description\": \"Stores tenant-specific settings and feature flag overrides.\","]
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
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"fields\": {"]
#[doc = "    \"feature_flags\": {"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      },"]
#[doc = "      \"type\": \"object\""]
#[doc = "    },"]
#[doc = "    \"privacy_settings\": {"]
#[doc = "      \"description\": \"Tenant-level privacy controls.\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"allow_social_weather\": {"]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"data_retention_days\": {"]
#[doc = "          \"default\": 3650,"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"title\": \"TenantConfigurationPrivacySettings\","]
#[doc = "      \"type\": \"object\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.0.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TenantConfigurationComponent {
    #[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
    pub fields: ::std::collections::HashMap<
        TenantConfigurationComponentFieldsKey,
        TenantConfigurationComponentFieldsValue,
    >,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&TenantConfigurationComponent> for TenantConfigurationComponent {
    fn from(value: &TenantConfigurationComponent) -> Self {
        value.clone()
    }
}
impl TenantConfigurationComponent {
    pub fn builder() -> builder::TenantConfigurationComponent {
        Default::default()
    }
}
#[doc = "`TenantConfigurationComponentFieldsKey`"]
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
pub struct TenantConfigurationComponentFieldsKey(::std::string::String);
impl ::std::ops::Deref for TenantConfigurationComponentFieldsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TenantConfigurationComponentFieldsKey> for ::std::string::String {
    fn from(value: TenantConfigurationComponentFieldsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TenantConfigurationComponentFieldsKey>
    for TenantConfigurationComponentFieldsKey
{
    fn from(value: &TenantConfigurationComponentFieldsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TenantConfigurationComponentFieldsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-z_][a-z0-9_]*$").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z_][a-z0-9_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TenantConfigurationComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TenantConfigurationComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TenantConfigurationComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TenantConfigurationComponentFieldsKey {
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
#[doc = "`TenantConfigurationComponentFieldsValue`"]
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
pub enum TenantConfigurationComponentFieldsValue {
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
impl ::std::convert::From<&Self> for TenantConfigurationComponentFieldsValue {
    fn from(value: &TenantConfigurationComponentFieldsValue) -> Self {
        value.clone()
    }
}
#[doc = "Canonical schema for a Tenant, the root container for all user data and configuration. This is a System Entity and does not participate in physics simulations."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/tenant.v1.schema.json\","]
#[doc = "  \"title\": \"Tenant Entity\","]
#[doc = "  \"description\": \"Canonical schema for a Tenant, the root container for all user data and configuration. This is a System Entity and does not participate in physics simulations.\","]
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
#[doc = "    \"created_at\","]
#[doc = "    \"entity_id\","]
#[doc = "    \"entity_type\","]
#[doc = "    \"tenant_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"TenantComponents\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"config\","]
#[doc = "        \"identity\","]
#[doc = "        \"members\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"config\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/TenantConfig.v1.schema.json\","]
#[doc = "          \"title\": \"Tenant Configuration Component\","]
#[doc = "          \"description\": \"Stores tenant-specific settings and feature flag overrides.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"fields\","]
#[doc = "            \"physics_properties\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"BaseFields\","]
#[doc = "              \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "                  \"oneOf\": ["]
#[doc = "                    {"]
#[doc = "                      \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Field\","]
#[doc = "                      \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"description\","]
#[doc = "                        \"type\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"constraints\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                          \"title\": \"Constraint Definition\","]
#[doc = "                          \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"enum\": {"]
#[doc = "                              \"type\": \"array\","]
#[doc = "                              \"items\": {"]
#[doc = "                                \"type\": \"string\""]
#[doc = "                              }"]
#[doc = "                            },"]
#[doc = "                            \"maxLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"maximum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"minLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"minimum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"pattern\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        },"]
#[doc = "                        \"default_value\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                          \"title\": \"Any Value\","]
#[doc = "                          \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                          \"type\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"number\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\","]
#[doc = "                            \"null\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        \"description\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                          \"title\": \"Description Field\","]
#[doc = "                          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                          \"default\": \"\","]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        },"]
#[doc = "                        \"type\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                          \"title\": \"Base Type System\","]
#[doc = "                          \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                          \"oneOf\": ["]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                              \"type\": \"string\","]
#[doc = "                              \"enum\": ["]
#[doc = "                                \"string\","]
#[doc = "                                \"boolean\","]
#[doc = "                                \"integer\","]
#[doc = "                                \"number\","]
#[doc = "                                \"f32\","]
#[doc = "                                \"f64\","]
#[doc = "                                \"i32\","]
#[doc = "                                \"i64\","]
#[doc = "                                \"u32\","]
#[doc = "                                \"u64\","]
#[doc = "                                \"uuid\","]
#[doc = "                                \"date-time\","]
#[doc = "                                \"duration\","]
#[doc = "                                \"object\","]
#[doc = "                                \"array\""]
#[doc = "                              ]"]
#[doc = "                            },"]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                              \"type\": \"object\","]
#[doc = "                              \"required\": ["]
#[doc = "                                \"$ref\""]
#[doc = "                              ],"]
#[doc = "                              \"properties\": {"]
#[doc = "                                \"$ref\": {"]
#[doc = "                                  \"type\": \"string\""]
#[doc = "                                }"]
#[doc = "                              },"]
#[doc = "                              \"additionalProperties\": false"]
#[doc = "                            }"]
#[doc = "                          ],"]
#[doc = "                          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                        },"]
#[doc = "                        \"ui_label\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    }"]
#[doc = "                  ]"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"physics_properties\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "              \"title\": \"Base Physics Properties\","]
#[doc = "              \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"engine\","]
#[doc = "                \"is_quantum\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"collapse_sensitive\": {"]
#[doc = "                  \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"engine\": {"]
#[doc = "                  \"description\": \"The physics engine responsible for this object.\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"quantum\","]
#[doc = "                    \"classical\","]
#[doc = "                    \"hybrid\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"is_quantum\": {"]
#[doc = "                  \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"feature_flags\": {"]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"type\": \"boolean\""]
#[doc = "              },"]
#[doc = "              \"type\": \"object\""]
#[doc = "            },"]
#[doc = "            \"privacy_settings\": {"]
#[doc = "              \"description\": \"Tenant-level privacy controls.\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"allow_social_weather\": {"]
#[doc = "                  \"default\": false,"]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"data_retention_days\": {"]
#[doc = "                  \"default\": 3650,"]
#[doc = "                  \"type\": \"integer\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"title\": \"TenantConfigurationPrivacySettings\","]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.0.0\""]
#[doc = "        },"]
#[doc = "        \"identity\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/TenantIdentity.v1.schema.json\","]
#[doc = "          \"title\": \"Tenant Identity Component\","]
#[doc = "          \"description\": \"Defines the core identity and metadata for a Tenant.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"fields\","]
#[doc = "            \"physics_properties\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"TenantIdentityFields\","]
#[doc = "              \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"subscription_plan\","]
#[doc = "                \"tenant_name\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"subscription_plan\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/system/SubscriptionPlan.v1.json\","]
#[doc = "                  \"title\": \"Subscription Plan\","]
#[doc = "                  \"description\": \"A canonical enum of all available subscription plans for a Tenant.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Free\","]
#[doc = "                    \"Personal\","]
#[doc = "                    \"FamilyPlus\","]
#[doc = "                    \"Enterprise\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"tenant_name\": {"]
#[doc = "                  \"title\": \"Name Field\","]
#[doc = "                  \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "                  \"oneOf\": ["]
#[doc = "                    {"]
#[doc = "                      \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Field\","]
#[doc = "                      \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"description\","]
#[doc = "                        \"type\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"constraints\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                          \"title\": \"Constraint Definition\","]
#[doc = "                          \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"enum\": {"]
#[doc = "                              \"type\": \"array\","]
#[doc = "                              \"items\": {"]
#[doc = "                                \"type\": \"string\""]
#[doc = "                              }"]
#[doc = "                            },"]
#[doc = "                            \"maxLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"maximum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"minLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"minimum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"pattern\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        },"]
#[doc = "                        \"default_value\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                          \"title\": \"Any Value\","]
#[doc = "                          \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                          \"type\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"number\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\","]
#[doc = "                            \"null\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        \"description\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                          \"title\": \"Description Field\","]
#[doc = "                          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                          \"default\": \"\","]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        },"]
#[doc = "                        \"type\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                          \"title\": \"Base Type System\","]
#[doc = "                          \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                          \"oneOf\": ["]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                              \"type\": \"string\","]
#[doc = "                              \"enum\": ["]
#[doc = "                                \"string\","]
#[doc = "                                \"boolean\","]
#[doc = "                                \"integer\","]
#[doc = "                                \"number\","]
#[doc = "                                \"f32\","]
#[doc = "                                \"f64\","]
#[doc = "                                \"i32\","]
#[doc = "                                \"i64\","]
#[doc = "                                \"u32\","]
#[doc = "                                \"u64\","]
#[doc = "                                \"uuid\","]
#[doc = "                                \"date-time\","]
#[doc = "                                \"duration\","]
#[doc = "                                \"object\","]
#[doc = "                                \"array\""]
#[doc = "                              ]"]
#[doc = "                            },"]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                              \"type\": \"object\","]
#[doc = "                              \"required\": ["]
#[doc = "                                \"$ref\""]
#[doc = "                              ],"]
#[doc = "                              \"properties\": {"]
#[doc = "                                \"$ref\": {"]
#[doc = "                                  \"type\": \"string\""]
#[doc = "                                }"]
#[doc = "                              },"]
#[doc = "                              \"additionalProperties\": false"]
#[doc = "                            }"]
#[doc = "                          ],"]
#[doc = "                          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                        },"]
#[doc = "                        \"ui_label\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    }"]
#[doc = "                  ]"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"physics_properties\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "              \"title\": \"Base Physics Properties\","]
#[doc = "              \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"engine\","]
#[doc = "                \"is_quantum\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"collapse_sensitive\": {"]
#[doc = "                  \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"engine\": {"]
#[doc = "                  \"description\": \"The physics engine responsible for this object.\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"quantum\","]
#[doc = "                    \"classical\","]
#[doc = "                    \"hybrid\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"is_quantum\": {"]
#[doc = "                  \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"classical\","]
#[doc = "            \"is_quantum\": false"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.1.0\""]
#[doc = "        },"]
#[doc = "        \"members\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/TenantMembers.v1.schema.json\","]
#[doc = "          \"title\": \"Tenant Members Component\","]
#[doc = "          \"description\": \"Manages the users and their roles within a Tenant.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"fields\","]
#[doc = "            \"physics_properties\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"BaseFields\","]
#[doc = "              \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"patternProperties\": {"]
#[doc = "                \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "                  \"oneOf\": ["]
#[doc = "                    {"]
#[doc = "                      \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                    },"]
#[doc = "                    {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Field\","]
#[doc = "                      \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"required\": ["]
#[doc = "                        \"description\","]
#[doc = "                        \"type\""]
#[doc = "                      ],"]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"constraints\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                          \"title\": \"Constraint Definition\","]
#[doc = "                          \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"enum\": {"]
#[doc = "                              \"type\": \"array\","]
#[doc = "                              \"items\": {"]
#[doc = "                                \"type\": \"string\""]
#[doc = "                              }"]
#[doc = "                            },"]
#[doc = "                            \"maxLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"maximum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"minLength\": {"]
#[doc = "                              \"type\": \"integer\""]
#[doc = "                            },"]
#[doc = "                            \"minimum\": {"]
#[doc = "                              \"type\": \"number\""]
#[doc = "                            },"]
#[doc = "                            \"pattern\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        },"]
#[doc = "                        \"default_value\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                          \"title\": \"Any Value\","]
#[doc = "                          \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                          \"type\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"number\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\","]
#[doc = "                            \"null\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        \"description\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                          \"title\": \"Description Field\","]
#[doc = "                          \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                          \"default\": \"\","]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        },"]
#[doc = "                        \"type\": {"]
#[doc = "                          \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                          \"title\": \"Base Type System\","]
#[doc = "                          \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                          \"oneOf\": ["]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                              \"type\": \"string\","]
#[doc = "                              \"enum\": ["]
#[doc = "                                \"string\","]
#[doc = "                                \"boolean\","]
#[doc = "                                \"integer\","]
#[doc = "                                \"number\","]
#[doc = "                                \"f32\","]
#[doc = "                                \"f64\","]
#[doc = "                                \"i32\","]
#[doc = "                                \"i64\","]
#[doc = "                                \"u32\","]
#[doc = "                                \"u64\","]
#[doc = "                                \"uuid\","]
#[doc = "                                \"date-time\","]
#[doc = "                                \"duration\","]
#[doc = "                                \"object\","]
#[doc = "                                \"array\""]
#[doc = "                              ]"]
#[doc = "                            },"]
#[doc = "                            {"]
#[doc = "                              \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                              \"type\": \"object\","]
#[doc = "                              \"required\": ["]
#[doc = "                                \"$ref\""]
#[doc = "                              ],"]
#[doc = "                              \"properties\": {"]
#[doc = "                                \"$ref\": {"]
#[doc = "                                  \"type\": \"string\""]
#[doc = "                                }"]
#[doc = "                              },"]
#[doc = "                              \"additionalProperties\": false"]
#[doc = "                            }"]
#[doc = "                          ],"]
#[doc = "                          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                        },"]
#[doc = "                        \"ui_label\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    }"]
#[doc = "                  ]"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            },"]
#[doc = "            \"physics_properties\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "              \"title\": \"Base Physics Properties\","]
#[doc = "              \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"engine\","]
#[doc = "                \"is_quantum\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"collapse_sensitive\": {"]
#[doc = "                  \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                },"]
#[doc = "                \"engine\": {"]
#[doc = "                  \"description\": \"The physics engine responsible for this object.\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"quantum\","]
#[doc = "                    \"classical\","]
#[doc = "                    \"hybrid\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"is_quantum\": {"]
#[doc = "                  \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "                  \"type\": \"boolean\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"members\": {"]
#[doc = "              \"description\": \"A list of users who are members of this tenant.\","]
#[doc = "              \"items\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/security/Member.v1.json\","]
#[doc = "                \"description\": \"Represents a single user within a tenant, including their role and join date.\","]
#[doc = "                \"properties\": {"]
#[doc = "                  \"joined_at\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "                    \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "                    \"title\": \"Timestamp\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"role\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/security/Role.v1.json\","]
#[doc = "                    \"description\": \"Defines the roles a user can have within a tenant.\","]
#[doc = "                    \"enum\": ["]
#[doc = "                      \"Admin\","]
#[doc = "                      \"Member\","]
#[doc = "                      \"Child\","]
#[doc = "                      \"Guardian\","]
#[doc = "                      \"ReadOnly\""]
#[doc = "                    ],"]
#[doc = "                    \"title\": \"Tenant Member Role\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"user_id\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "                    \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "                    \"title\": \"UUID\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"required\": ["]
#[doc = "                  \"user_id\","]
#[doc = "                  \"role\","]
#[doc = "                  \"joined_at\""]
#[doc = "                ],"]
#[doc = "                \"title\": \"Tenant Member\","]
#[doc = "                \"type\": \"object\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
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
#[doc = "    \"created_at\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"entity_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"entity_type\": {"]
#[doc = "      \"description\": \"The canonical type of the system entity.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Stitch\","]
#[doc = "        \"WorkflowTask\","]
#[doc = "        \"Tenant\""]
#[doc = "      ],"]
#[doc = "      \"const\": \"Tenant\""]
#[doc = "    },"]
#[doc = "    \"tenant_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
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
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"entities/Tenant.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TenantEntity {
    pub components: TenantEntityComponents,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub created_at: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub entity_id: ::std::string::String,
    #[doc = "The canonical type of the system entity."]
    pub entity_type: TenantEntityEntityType,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub tenant_id: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub workflow_state: ::std::option::Option<WorkflowState>,
}
impl ::std::convert::From<&TenantEntity> for TenantEntity {
    fn from(value: &TenantEntity) -> Self {
        value.clone()
    }
}
impl TenantEntity {
    pub fn builder() -> builder::TenantEntity {
        Default::default()
    }
}
#[doc = "`TenantEntityComponents`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"config\","]
#[doc = "    \"identity\","]
#[doc = "    \"members\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"config\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/TenantConfig.v1.schema.json\","]
#[doc = "      \"title\": \"Tenant Configuration Component\","]
#[doc = "      \"description\": \"Stores tenant-specific settings and feature flag overrides.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"fields\","]
#[doc = "        \"physics_properties\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"BaseFields\","]
#[doc = "          \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                  \"title\": \"Base Field\","]
#[doc = "                  \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"description\","]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"constraints\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                      \"title\": \"Constraint Definition\","]
#[doc = "                      \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"enum\": {"]
#[doc = "                          \"type\": \"array\","]
#[doc = "                          \"items\": {"]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          }"]
#[doc = "                        },"]
#[doc = "                        \"maxLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"maximum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"minLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"minimum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"pattern\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"default_value\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                      \"title\": \"Any Value\","]
#[doc = "                      \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                      \"type\": ["]
#[doc = "                        \"string\","]
#[doc = "                        \"number\","]
#[doc = "                        \"integer\","]
#[doc = "                        \"boolean\","]
#[doc = "                        \"object\","]
#[doc = "                        \"array\","]
#[doc = "                        \"null\""]
#[doc = "                      ]"]
#[doc = "                    },"]
#[doc = "                    \"description\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                      \"title\": \"Description Field\","]
#[doc = "                      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                      \"default\": \"\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Type System\","]
#[doc = "                      \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                      \"oneOf\": ["]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                          \"type\": \"string\","]
#[doc = "                          \"enum\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"number\","]
#[doc = "                            \"f32\","]
#[doc = "                            \"f64\","]
#[doc = "                            \"i32\","]
#[doc = "                            \"i64\","]
#[doc = "                            \"u32\","]
#[doc = "                            \"u64\","]
#[doc = "                            \"uuid\","]
#[doc = "                            \"date-time\","]
#[doc = "                            \"duration\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"required\": ["]
#[doc = "                            \"$ref\""]
#[doc = "                          ],"]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"$ref\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        }"]
#[doc = "                      ],"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    },"]
#[doc = "                    \"ui_label\": {"]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"physics_properties\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "          \"title\": \"Base Physics Properties\","]
#[doc = "          \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"engine\","]
#[doc = "            \"is_quantum\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"collapse_sensitive\": {"]
#[doc = "              \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"engine\": {"]
#[doc = "              \"description\": \"The physics engine responsible for this object.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"quantum\","]
#[doc = "                \"classical\","]
#[doc = "                \"hybrid\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"is_quantum\": {"]
#[doc = "              \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"feature_flags\": {"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"type\": \"object\""]
#[doc = "        },"]
#[doc = "        \"privacy_settings\": {"]
#[doc = "          \"description\": \"Tenant-level privacy controls.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"allow_social_weather\": {"]
#[doc = "              \"default\": false,"]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"data_retention_days\": {"]
#[doc = "              \"default\": 3650,"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"title\": \"TenantConfigurationPrivacySettings\","]
#[doc = "          \"type\": \"object\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.0.0\""]
#[doc = "    },"]
#[doc = "    \"identity\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/TenantIdentity.v1.schema.json\","]
#[doc = "      \"title\": \"Tenant Identity Component\","]
#[doc = "      \"description\": \"Defines the core identity and metadata for a Tenant.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"fields\","]
#[doc = "        \"physics_properties\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"TenantIdentityFields\","]
#[doc = "          \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"subscription_plan\","]
#[doc = "            \"tenant_name\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"subscription_plan\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/system/SubscriptionPlan.v1.json\","]
#[doc = "              \"title\": \"Subscription Plan\","]
#[doc = "              \"description\": \"A canonical enum of all available subscription plans for a Tenant.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Free\","]
#[doc = "                \"Personal\","]
#[doc = "                \"FamilyPlus\","]
#[doc = "                \"Enterprise\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"tenant_name\": {"]
#[doc = "              \"title\": \"Name Field\","]
#[doc = "              \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                  \"title\": \"Base Field\","]
#[doc = "                  \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"description\","]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"constraints\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                      \"title\": \"Constraint Definition\","]
#[doc = "                      \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"enum\": {"]
#[doc = "                          \"type\": \"array\","]
#[doc = "                          \"items\": {"]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          }"]
#[doc = "                        },"]
#[doc = "                        \"maxLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"maximum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"minLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"minimum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"pattern\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"default_value\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                      \"title\": \"Any Value\","]
#[doc = "                      \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                      \"type\": ["]
#[doc = "                        \"string\","]
#[doc = "                        \"number\","]
#[doc = "                        \"integer\","]
#[doc = "                        \"boolean\","]
#[doc = "                        \"object\","]
#[doc = "                        \"array\","]
#[doc = "                        \"null\""]
#[doc = "                      ]"]
#[doc = "                    },"]
#[doc = "                    \"description\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                      \"title\": \"Description Field\","]
#[doc = "                      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                      \"default\": \"\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Type System\","]
#[doc = "                      \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                      \"oneOf\": ["]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                          \"type\": \"string\","]
#[doc = "                          \"enum\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"number\","]
#[doc = "                            \"f32\","]
#[doc = "                            \"f64\","]
#[doc = "                            \"i32\","]
#[doc = "                            \"i64\","]
#[doc = "                            \"u32\","]
#[doc = "                            \"u64\","]
#[doc = "                            \"uuid\","]
#[doc = "                            \"date-time\","]
#[doc = "                            \"duration\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"required\": ["]
#[doc = "                            \"$ref\""]
#[doc = "                          ],"]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"$ref\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        }"]
#[doc = "                      ],"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    },"]
#[doc = "                    \"ui_label\": {"]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"physics_properties\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "          \"title\": \"Base Physics Properties\","]
#[doc = "          \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"engine\","]
#[doc = "            \"is_quantum\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"collapse_sensitive\": {"]
#[doc = "              \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"engine\": {"]
#[doc = "              \"description\": \"The physics engine responsible for this object.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"quantum\","]
#[doc = "                \"classical\","]
#[doc = "                \"hybrid\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"is_quantum\": {"]
#[doc = "              \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"classical\","]
#[doc = "        \"is_quantum\": false"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.1.0\""]
#[doc = "    },"]
#[doc = "    \"manifold_position\": false,"]
#[doc = "    \"members\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/TenantMembers.v1.schema.json\","]
#[doc = "      \"title\": \"Tenant Members Component\","]
#[doc = "      \"description\": \"Manages the users and their roles within a Tenant.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"fields\","]
#[doc = "        \"physics_properties\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"BaseFields\","]
#[doc = "          \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"patternProperties\": {"]
#[doc = "            \"^[a-z_][a-z0-9_]*$\": {"]
#[doc = "              \"oneOf\": ["]
#[doc = "                {"]
#[doc = "                  \"description\": \"A direct reference to a shared definition in SharedDefinitions.schema.json.\","]
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
#[doc = "                },"]
#[doc = "                {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "                  \"title\": \"Base Field\","]
#[doc = "                  \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "                  \"type\": \"object\","]
#[doc = "                  \"required\": ["]
#[doc = "                    \"description\","]
#[doc = "                    \"type\""]
#[doc = "                  ],"]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"constraints\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "                      \"title\": \"Constraint Definition\","]
#[doc = "                      \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "                      \"type\": \"object\","]
#[doc = "                      \"properties\": {"]
#[doc = "                        \"enum\": {"]
#[doc = "                          \"type\": \"array\","]
#[doc = "                          \"items\": {"]
#[doc = "                            \"type\": \"string\""]
#[doc = "                          }"]
#[doc = "                        },"]
#[doc = "                        \"maxLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"maximum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"minLength\": {"]
#[doc = "                          \"type\": \"integer\""]
#[doc = "                        },"]
#[doc = "                        \"minimum\": {"]
#[doc = "                          \"type\": \"number\""]
#[doc = "                        },"]
#[doc = "                        \"pattern\": {"]
#[doc = "                          \"type\": \"string\""]
#[doc = "                        }"]
#[doc = "                      },"]
#[doc = "                      \"additionalProperties\": false"]
#[doc = "                    },"]
#[doc = "                    \"default_value\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "                      \"title\": \"Any Value\","]
#[doc = "                      \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "                      \"type\": ["]
#[doc = "                        \"string\","]
#[doc = "                        \"number\","]
#[doc = "                        \"integer\","]
#[doc = "                        \"boolean\","]
#[doc = "                        \"object\","]
#[doc = "                        \"array\","]
#[doc = "                        \"null\""]
#[doc = "                      ]"]
#[doc = "                    },"]
#[doc = "                    \"description\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                      \"title\": \"Description Field\","]
#[doc = "                      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                      \"default\": \"\","]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    },"]
#[doc = "                    \"type\": {"]
#[doc = "                      \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "                      \"title\": \"Base Type System\","]
#[doc = "                      \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "                      \"oneOf\": ["]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "                          \"type\": \"string\","]
#[doc = "                          \"enum\": ["]
#[doc = "                            \"string\","]
#[doc = "                            \"boolean\","]
#[doc = "                            \"integer\","]
#[doc = "                            \"number\","]
#[doc = "                            \"f32\","]
#[doc = "                            \"f64\","]
#[doc = "                            \"i32\","]
#[doc = "                            \"i64\","]
#[doc = "                            \"u32\","]
#[doc = "                            \"u64\","]
#[doc = "                            \"uuid\","]
#[doc = "                            \"date-time\","]
#[doc = "                            \"duration\","]
#[doc = "                            \"object\","]
#[doc = "                            \"array\""]
#[doc = "                          ]"]
#[doc = "                        },"]
#[doc = "                        {"]
#[doc = "                          \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "                          \"type\": \"object\","]
#[doc = "                          \"required\": ["]
#[doc = "                            \"$ref\""]
#[doc = "                          ],"]
#[doc = "                          \"properties\": {"]
#[doc = "                            \"$ref\": {"]
#[doc = "                              \"type\": \"string\""]
#[doc = "                            }"]
#[doc = "                          },"]
#[doc = "                          \"additionalProperties\": false"]
#[doc = "                        }"]
#[doc = "                      ],"]
#[doc = "                      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                    },"]
#[doc = "                    \"ui_label\": {"]
#[doc = "                      \"type\": \"string\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "                }"]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"physics_properties\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "          \"title\": \"Base Physics Properties\","]
#[doc = "          \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"engine\","]
#[doc = "            \"is_quantum\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"collapse_sensitive\": {"]
#[doc = "              \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            },"]
#[doc = "            \"engine\": {"]
#[doc = "              \"description\": \"The physics engine responsible for this object.\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"quantum\","]
#[doc = "                \"classical\","]
#[doc = "                \"hybrid\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"is_quantum\": {"]
#[doc = "              \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"members\": {"]
#[doc = "          \"description\": \"A list of users who are members of this tenant.\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/security/Member.v1.json\","]
#[doc = "            \"description\": \"Represents a single user within a tenant, including their role and join date.\","]
#[doc = "            \"properties\": {"]
#[doc = "              \"joined_at\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "                \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "                \"title\": \"Timestamp\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"role\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/security/Role.v1.json\","]
#[doc = "                \"description\": \"Defines the roles a user can have within a tenant.\","]
#[doc = "                \"enum\": ["]
#[doc = "                  \"Admin\","]
#[doc = "                  \"Member\","]
#[doc = "                  \"Child\","]
#[doc = "                  \"Guardian\","]
#[doc = "                  \"ReadOnly\""]
#[doc = "                ],"]
#[doc = "                \"title\": \"Tenant Member Role\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"user_id\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "                \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "                \"title\": \"UUID\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"required\": ["]
#[doc = "              \"user_id\","]
#[doc = "              \"role\","]
#[doc = "              \"joined_at\""]
#[doc = "            ],"]
#[doc = "            \"title\": \"Tenant Member\","]
#[doc = "            \"type\": \"object\""]
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
#[doc = "    \"universal_physics_state\": false"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TenantEntityComponents {
    pub config: TenantConfigurationComponent,
    pub identity: TenantIdentityComponent,
    pub members: TenantMembersComponent,
}
impl ::std::convert::From<&TenantEntityComponents> for TenantEntityComponents {
    fn from(value: &TenantEntityComponents) -> Self {
        value.clone()
    }
}
impl TenantEntityComponents {
    pub fn builder() -> builder::TenantEntityComponents {
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
#[doc = "  \"const\": \"Tenant\""]
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
pub enum TenantEntityEntityType {
    Stitch,
    WorkflowTask,
    Tenant,
}
impl ::std::convert::From<&Self> for TenantEntityEntityType {
    fn from(value: &TenantEntityEntityType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TenantEntityEntityType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Stitch => write!(f, "Stitch"),
            Self::WorkflowTask => write!(f, "WorkflowTask"),
            Self::Tenant => write!(f, "Tenant"),
        }
    }
}
impl ::std::str::FromStr for TenantEntityEntityType {
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
impl ::std::convert::TryFrom<&str> for TenantEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TenantEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TenantEntityEntityType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Defines the core identity and metadata for a Tenant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/TenantIdentity.v1.schema.json\","]
#[doc = "  \"title\": \"Tenant Identity Component\","]
#[doc = "  \"description\": \"Defines the core identity and metadata for a Tenant.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"fields\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"TenantIdentityFields\","]
#[doc = "      \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"subscription_plan\","]
#[doc = "        \"tenant_name\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"subscription_plan\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/system/SubscriptionPlan.v1.json\","]
#[doc = "          \"title\": \"Subscription Plan\","]
#[doc = "          \"description\": \"A canonical enum of all available subscription plans for a Tenant.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Free\","]
#[doc = "            \"Personal\","]
#[doc = "            \"FamilyPlus\","]
#[doc = "            \"Enterprise\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"tenant_name\": {"]
#[doc = "          \"title\": \"Name Field\","]
#[doc = "          \"description\": \"The primary, human-readable name of an entity.\","]
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
pub struct TenantIdentityComponent {
    pub fields: TenantIdentityFields,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&TenantIdentityComponent> for TenantIdentityComponent {
    fn from(value: &TenantIdentityComponent) -> Self {
        value.clone()
    }
}
impl TenantIdentityComponent {
    pub fn builder() -> builder::TenantIdentityComponent {
        Default::default()
    }
}
#[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"TenantIdentityFields\","]
#[doc = "  \"description\": \"A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"subscription_plan\","]
#[doc = "    \"tenant_name\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"subscription_plan\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/system/SubscriptionPlan.v1.json\","]
#[doc = "      \"title\": \"Subscription Plan\","]
#[doc = "      \"description\": \"A canonical enum of all available subscription plans for a Tenant.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Free\","]
#[doc = "        \"Personal\","]
#[doc = "        \"FamilyPlus\","]
#[doc = "        \"Enterprise\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tenant_name\": {"]
#[doc = "      \"title\": \"Name Field\","]
#[doc = "      \"description\": \"The primary, human-readable name of an entity.\","]
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
pub struct TenantIdentityFields {
    #[doc = "A canonical enum of all available subscription plans for a Tenant."]
    pub subscription_plan: SubscriptionPlan,
    #[doc = "The primary, human-readable name of an entity."]
    pub tenant_name: ::std::string::String,
}
impl ::std::convert::From<&TenantIdentityFields> for TenantIdentityFields {
    fn from(value: &TenantIdentityFields) -> Self {
        value.clone()
    }
}
impl TenantIdentityFields {
    pub fn builder() -> builder::TenantIdentityFields {
        Default::default()
    }
}
#[doc = "Manages the users and their roles within a Tenant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/TenantMembers.v1.schema.json\","]
#[doc = "  \"title\": \"Tenant Members Component\","]
#[doc = "  \"description\": \"Manages the users and their roles within a Tenant.\","]
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
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"fields\": {"]
#[doc = "    \"members\": {"]
#[doc = "      \"description\": \"A list of users who are members of this tenant.\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/security/Member.v1.json\","]
#[doc = "        \"description\": \"Represents a single user within a tenant, including their role and join date.\","]
#[doc = "        \"properties\": {"]
#[doc = "          \"joined_at\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "            \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "            \"title\": \"Timestamp\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"role\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/security/Role.v1.json\","]
#[doc = "            \"description\": \"Defines the roles a user can have within a tenant.\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"Admin\","]
#[doc = "              \"Member\","]
#[doc = "              \"Child\","]
#[doc = "              \"Guardian\","]
#[doc = "              \"ReadOnly\""]
#[doc = "            ],"]
#[doc = "            \"title\": \"Tenant Member Role\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"user_id\": {"]
#[doc = "            \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "            \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "            \"title\": \"UUID\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"required\": ["]
#[doc = "          \"user_id\","]
#[doc = "          \"role\","]
#[doc = "          \"joined_at\""]
#[doc = "        ],"]
#[doc = "        \"title\": \"Tenant Member\","]
#[doc = "        \"type\": \"object\""]
#[doc = "      },"]
#[doc = "      \"type\": \"array\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.0.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TenantMembersComponent {
    #[doc = "A map of field names to their definitions. Each field can be defined inline or by referencing a shared definition."]
    pub fields: ::std::collections::HashMap<
        TenantMembersComponentFieldsKey,
        TenantMembersComponentFieldsValue,
    >,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&TenantMembersComponent> for TenantMembersComponent {
    fn from(value: &TenantMembersComponent) -> Self {
        value.clone()
    }
}
impl TenantMembersComponent {
    pub fn builder() -> builder::TenantMembersComponent {
        Default::default()
    }
}
#[doc = "`TenantMembersComponentFieldsKey`"]
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
pub struct TenantMembersComponentFieldsKey(::std::string::String);
impl ::std::ops::Deref for TenantMembersComponentFieldsKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TenantMembersComponentFieldsKey> for ::std::string::String {
    fn from(value: TenantMembersComponentFieldsKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TenantMembersComponentFieldsKey> for TenantMembersComponentFieldsKey {
    fn from(value: &TenantMembersComponentFieldsKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for TenantMembersComponentFieldsKey {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-z_][a-z0-9_]*$").unwrap());
        if (&*PATTERN).find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z_][a-z0-9_]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TenantMembersComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TenantMembersComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TenantMembersComponentFieldsKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TenantMembersComponentFieldsKey {
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
#[doc = "`TenantMembersComponentFieldsValue`"]
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
pub enum TenantMembersComponentFieldsValue {
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
impl ::std::convert::From<&Self> for TenantMembersComponentFieldsValue {
    fn from(value: &TenantMembersComponentFieldsValue) -> Self {
        value.clone()
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
    pub struct TenantConfigurationComponent {
        fields: ::std::result::Result<
            ::std::collections::HashMap<
                super::TenantConfigurationComponentFieldsKey,
                super::TenantConfigurationComponentFieldsValue,
            >,
            ::std::string::String,
        >,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for TenantConfigurationComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl TenantConfigurationComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::TenantConfigurationComponentFieldsKey,
                    super::TenantConfigurationComponentFieldsValue,
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
    impl ::std::convert::TryFrom<TenantConfigurationComponent> for super::TenantConfigurationComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantConfigurationComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::TenantConfigurationComponent> for TenantConfigurationComponent {
        fn from(value: super::TenantConfigurationComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TenantEntity {
        components: ::std::result::Result<super::TenantEntityComponents, ::std::string::String>,
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        entity_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        entity_type: ::std::result::Result<super::TenantEntityEntityType, ::std::string::String>,
        tenant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        workflow_state: ::std::result::Result<
            ::std::option::Option<super::WorkflowState>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TenantEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                entity_id: Err("no value supplied for entity_id".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                tenant_id: Err("no value supplied for tenant_id".to_string()),
                workflow_state: Ok(Default::default()),
            }
        }
    }
    impl TenantEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantEntityComponents>,
            T::Error: ::std::fmt::Display,
        {
            self.components = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for components: {}", e));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {}", e));
            self
        }
        pub fn entity_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity_id: {}", e));
            self
        }
        pub fn entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantEntityEntityType>,
            T::Error: ::std::fmt::Display,
        {
            self.entity_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for entity_type: {}", e));
            self
        }
        pub fn tenant_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant_id: {}", e));
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
    impl ::std::convert::TryFrom<TenantEntity> for super::TenantEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                created_at: value.created_at?,
                entity_id: value.entity_id?,
                entity_type: value.entity_type?,
                tenant_id: value.tenant_id?,
                workflow_state: value.workflow_state?,
            })
        }
    }
    impl ::std::convert::From<super::TenantEntity> for TenantEntity {
        fn from(value: super::TenantEntity) -> Self {
            Self {
                components: Ok(value.components),
                created_at: Ok(value.created_at),
                entity_id: Ok(value.entity_id),
                entity_type: Ok(value.entity_type),
                tenant_id: Ok(value.tenant_id),
                workflow_state: Ok(value.workflow_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TenantEntityComponents {
        config: ::std::result::Result<super::TenantConfigurationComponent, ::std::string::String>,
        identity: ::std::result::Result<super::TenantIdentityComponent, ::std::string::String>,
        members: ::std::result::Result<super::TenantMembersComponent, ::std::string::String>,
    }
    impl ::std::default::Default for TenantEntityComponents {
        fn default() -> Self {
            Self {
                config: Err("no value supplied for config".to_string()),
                identity: Err("no value supplied for identity".to_string()),
                members: Err("no value supplied for members".to_string()),
            }
        }
    }
    impl TenantEntityComponents {
        pub fn config<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantConfigurationComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.config = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for config: {}", e));
            self
        }
        pub fn identity<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantIdentityComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.identity = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for identity: {}", e));
            self
        }
        pub fn members<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantMembersComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.members = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for members: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TenantEntityComponents> for super::TenantEntityComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantEntityComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                config: value.config?,
                identity: value.identity?,
                members: value.members?,
            })
        }
    }
    impl ::std::convert::From<super::TenantEntityComponents> for TenantEntityComponents {
        fn from(value: super::TenantEntityComponents) -> Self {
            Self {
                config: Ok(value.config),
                identity: Ok(value.identity),
                members: Ok(value.members),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TenantIdentityComponent {
        fields: ::std::result::Result<super::TenantIdentityFields, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for TenantIdentityComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl TenantIdentityComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantIdentityFields>,
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
    impl ::std::convert::TryFrom<TenantIdentityComponent> for super::TenantIdentityComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantIdentityComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::TenantIdentityComponent> for TenantIdentityComponent {
        fn from(value: super::TenantIdentityComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TenantIdentityFields {
        subscription_plan: ::std::result::Result<super::SubscriptionPlan, ::std::string::String>,
        tenant_name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TenantIdentityFields {
        fn default() -> Self {
            Self {
                subscription_plan: Err("no value supplied for subscription_plan".to_string()),
                tenant_name: Err("no value supplied for tenant_name".to_string()),
            }
        }
    }
    impl TenantIdentityFields {
        pub fn subscription_plan<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SubscriptionPlan>,
            T::Error: ::std::fmt::Display,
        {
            self.subscription_plan = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for subscription_plan: {}",
                    e
                )
            });
            self
        }
        pub fn tenant_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.tenant_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tenant_name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<TenantIdentityFields> for super::TenantIdentityFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantIdentityFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                subscription_plan: value.subscription_plan?,
                tenant_name: value.tenant_name?,
            })
        }
    }
    impl ::std::convert::From<super::TenantIdentityFields> for TenantIdentityFields {
        fn from(value: super::TenantIdentityFields) -> Self {
            Self {
                subscription_plan: Ok(value.subscription_plan),
                tenant_name: Ok(value.tenant_name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TenantMembersComponent {
        fields: ::std::result::Result<
            ::std::collections::HashMap<
                super::TenantMembersComponentFieldsKey,
                super::TenantMembersComponentFieldsValue,
            >,
            ::std::string::String,
        >,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for TenantMembersComponent {
        fn default() -> Self {
            Self {
                fields: Err("no value supplied for fields".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl TenantMembersComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::TenantMembersComponentFieldsKey,
                    super::TenantMembersComponentFieldsValue,
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
    impl ::std::convert::TryFrom<TenantMembersComponent> for super::TenantMembersComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantMembersComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::TenantMembersComponent> for TenantMembersComponent {
        fn from(value: super::TenantMembersComponent) -> Self {
            Self {
                fields: Ok(value.fields),
                physics_properties: Ok(value.physics_properties),
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
