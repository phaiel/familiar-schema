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
#[doc = "Schema for defining a database table column."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/database/ColumnDefinition.v1.json\","]
#[doc = "  \"title\": \"Column Definition\","]
#[doc = "  \"description\": \"Schema for defining a database table column.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name Field\","]
#[doc = "      \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"primary_key\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"boolean\","]
#[doc = "        \"timestamp\","]
#[doc = "        \"uuid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"unique\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ColumnDefinition {
    #[doc = "The primary, human-readable name of an entity."]
    pub name: ::std::string::String,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(rename = "type")]
    pub type_: ColumnDefinitionType,
    #[serde(default)]
    pub unique: bool,
}
impl ::std::convert::From<&ColumnDefinition> for ColumnDefinition {
    fn from(value: &ColumnDefinition) -> Self {
        value.clone()
    }
}
impl ColumnDefinition {
    pub fn builder() -> builder::ColumnDefinition {
        Default::default()
    }
}
#[doc = "`ColumnDefinitionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"boolean\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"uuid\""]
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
pub enum ColumnDefinitionType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "uuid")]
    Uuid,
}
impl ::std::convert::From<&Self> for ColumnDefinitionType {
    fn from(value: &ColumnDefinitionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ColumnDefinitionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => write!(f, "string"),
            Self::Integer => write!(f, "integer"),
            Self::Boolean => write!(f, "boolean"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Uuid => write!(f, "uuid"),
        }
    }
}
impl ::std::str::FromStr for ColumnDefinitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            "integer" => Ok(Self::Integer),
            "boolean" => Ok(Self::Boolean),
            "timestamp" => Ok(Self::Timestamp),
            "uuid" => Ok(Self::Uuid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A generic nested key-value map structure where outer keys map to objects, and inner keys map to string, number, or boolean values."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Key Value\","]
#[doc = "  \"description\": \"A generic nested key-value map structure where outer keys map to objects, and inner keys map to string, number, or boolean values.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"value\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"value\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"additionalProperties\": {"]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"number\","]
#[doc = "            \"boolean\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct KeyValue {
    pub value: ::std::collections::HashMap<
        ::std::string::String,
        ::std::collections::HashMap<::std::string::String, KeyValueValueValueValue>,
    >,
}
impl ::std::convert::From<&KeyValue> for KeyValue {
    fn from(value: &KeyValue) -> Self {
        value.clone()
    }
}
impl KeyValue {
    pub fn builder() -> builder::KeyValue {
        Default::default()
    }
}
#[doc = "`KeyValueValueValueValue`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"number\","]
#[doc = "    \"boolean\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum KeyValueValueValueValue {
    Boolean(bool),
    Number(f64),
    String(::std::string::String),
}
impl ::std::convert::From<&Self> for KeyValueValueValueValue {
    fn from(value: &KeyValueValueValueValue) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for KeyValueValueValueValue {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Boolean(v))
        } else { let v = value.parse() .unwrap();
            Ok(Self::Number(v))
        } else { let v = value.parse() .unwrap();
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for KeyValueValueValueValue {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for KeyValueValueValueValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for KeyValueValueValueValue {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::fmt::Display for KeyValueValueValueValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Boolean(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<bool> for KeyValueValueValueValue {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<f64> for KeyValueValueValueValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "A canonical enum of all possible lifecycle states for a Thread entity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "  \"title\": \"Thread State\","]
#[doc = "  \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Active\","]
#[doc = "    \"Inactive\","]
#[doc = "    \"Fading\","]
#[doc = "    \"Archived\""]
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
pub enum ThreadState {
    Active,
    Inactive,
    Fading,
    Archived,
}
impl ::std::convert::From<&Self> for ThreadState {
    fn from(value: &ThreadState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "Active"),
            Self::Inactive => write!(f, "Inactive"),
            Self::Fading => write!(f, "Fading"),
            Self::Archived => write!(f, "Archived"),
        }
    }
}
impl ::std::str::FromStr for ThreadState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Active" => Ok(Self::Active),
            "Inactive" => Ok(Self::Inactive),
            "Fading" => Ok(Self::Fading),
            "Archived" => Ok(Self::Archived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A map of column names to their definitions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThreadStateLogColumns\","]
#[doc = "  \"description\": \"A map of column names to their definitions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"effective_at\","]
#[doc = "    \"state\","]
#[doc = "    \"thread_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"effective_at\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"Key Value\","]
#[doc = "      \"description\": \"A generic nested key-value map structure where outer keys map to objects, and inner keys map to string, number, or boolean values.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"value\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"value\": {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"additionalProperties\": {"]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"number\","]
#[doc = "                \"boolean\""]
#[doc = "              ]"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "      \"title\": \"Thread State Reason\","]
#[doc = "      \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"UserMarkedInactive\","]
#[doc = "        \"UserMarkedDeceased\","]
#[doc = "        \"SystemDetectedInactivity\","]
#[doc = "        \"LifecycleCompleted\""]
#[doc = "      ],"]
#[doc = "      \"nullable\": true"]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "      \"title\": \"Thread State\","]
#[doc = "      \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Active\","]
#[doc = "        \"Inactive\","]
#[doc = "        \"Fading\","]
#[doc = "        \"Archived\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"thread_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"$id\": \"https://familiar.dev/schemas/types/database/ColumnDefinition.v1.json\","]
#[doc = "    \"title\": \"Column Definition\","]
#[doc = "    \"description\": \"Schema for defining a database table column.\","]
#[doc = "    \"type\": \"object\","]
#[doc = "    \"required\": ["]
#[doc = "      \"name\","]
#[doc = "      \"type\""]
#[doc = "    ],"]
#[doc = "    \"properties\": {"]
#[doc = "      \"name\": {"]
#[doc = "        \"title\": \"Name Field\","]
#[doc = "        \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"nullable\": {"]
#[doc = "        \"default\": false,"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      },"]
#[doc = "      \"primary_key\": {"]
#[doc = "        \"default\": false,"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      },"]
#[doc = "      \"type\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"enum\": ["]
#[doc = "          \"string\","]
#[doc = "          \"integer\","]
#[doc = "          \"boolean\","]
#[doc = "          \"timestamp\","]
#[doc = "          \"uuid\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"unique\": {"]
#[doc = "        \"default\": false,"]
#[doc = "        \"type\": \"boolean\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStateLogColumns {
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub effective_at: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<KeyValue>,
    #[doc = "A canonical enum of the machine-readable reasons for a Thread state change."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<ThreadStateReason>,
    #[doc = "A canonical enum of all possible lifecycle states for a Thread entity."]
    pub state: ThreadState,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub thread_id: ::std::string::String,
    #[serde(flatten)]
    pub extra: ::std::collections::HashMap<::std::string::String, ColumnDefinition>,
}
impl ::std::convert::From<&ThreadStateLogColumns> for ThreadStateLogColumns {
    fn from(value: &ThreadStateLogColumns) -> Self {
        value.clone()
    }
}
impl ThreadStateLogColumns {
    pub fn builder() -> builder::ThreadStateLogColumns {
        Default::default()
    }
}
#[doc = "Defines the schema for the 'thread_state_log' database table..."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/tables/thread_state_log.v1.schema.json\","]
#[doc = "  \"title\": \"Thread State Log Table\","]
#[doc = "  \"description\": \"Defines the schema for the 'thread_state_log' database table...\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"columns\","]
#[doc = "    \"description\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"tableName\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"columns\": {"]
#[doc = "      \"title\": \"ThreadStateLogColumns\","]
#[doc = "      \"description\": \"A map of column names to their definitions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"effective_at\","]
#[doc = "        \"state\","]
#[doc = "        \"thread_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"effective_at\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "          \"title\": \"Timestamp\","]
#[doc = "          \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Key Value\","]
#[doc = "          \"description\": \"A generic nested key-value map structure where outer keys map to objects, and inner keys map to string, number, or boolean values.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"value\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"value\": {"]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"additionalProperties\": {"]
#[doc = "                  \"type\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"number\","]
#[doc = "                    \"boolean\""]
#[doc = "                  ]"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "          \"title\": \"Thread State Reason\","]
#[doc = "          \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"UserMarkedInactive\","]
#[doc = "            \"UserMarkedDeceased\","]
#[doc = "            \"SystemDetectedInactivity\","]
#[doc = "            \"LifecycleCompleted\""]
#[doc = "          ],"]
#[doc = "          \"nullable\": true"]
#[doc = "        },"]
#[doc = "        \"state\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadState.v1.json\","]
#[doc = "          \"title\": \"Thread State\","]
#[doc = "          \"description\": \"A canonical enum of all possible lifecycle states for a Thread entity.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Active\","]
#[doc = "            \"Inactive\","]
#[doc = "            \"Fading\","]
#[doc = "            \"Archived\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"thread_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/database/ColumnDefinition.v1.json\","]
#[doc = "        \"title\": \"Column Definition\","]
#[doc = "        \"description\": \"Schema for defining a database table column.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"name\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"title\": \"Name Field\","]
#[doc = "            \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nullable\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"primary_key\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"string\","]
#[doc = "              \"integer\","]
#[doc = "              \"boolean\","]
#[doc = "              \"timestamp\","]
#[doc = "              \"uuid\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"unique\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tableName\": {"]
#[doc = "      \"description\": \"The physical name of the table in the database.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"tables\","]
#[doc = "  \"indexes\": ["]
#[doc = "    {"]
#[doc = "      \"columns\": ["]
#[doc = "        \"thread_id\","]
#[doc = "        \"effective_at DESC\""]
#[doc = "      ],"]
#[doc = "      \"name\": \"idx_thread_state_log_latest\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"schema_version\": \"1.2.0\","]
#[doc = "  \"source_file\": \"tables/Thread_state_log.schema.json\","]
#[doc = "  \"tableName\": \"thread_state_log\","]
#[doc = "  \"timescale\": {"]
#[doc = "    \"is_hypertable\": true,"]
#[doc = "    \"time_column_name\": \"effective_at\""]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThreadStateLogTable {
    pub columns: ThreadStateLogColumns,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The physical name of the table in the database."]
    #[serde(rename = "tableName")]
    pub table_name: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&ThreadStateLogTable> for ThreadStateLogTable {
    fn from(value: &ThreadStateLogTable) -> Self {
        value.clone()
    }
}
impl ThreadStateLogTable {
    pub fn builder() -> builder::ThreadStateLogTable {
        Default::default()
    }
}
#[doc = "A canonical enum of the machine-readable reasons for a Thread state change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "  \"title\": \"Thread State Reason\","]
#[doc = "  \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserMarkedInactive\","]
#[doc = "    \"UserMarkedDeceased\","]
#[doc = "    \"SystemDetectedInactivity\","]
#[doc = "    \"LifecycleCompleted\""]
#[doc = "  ],"]
#[doc = "  \"nullable\": true"]
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
pub enum ThreadStateReason {
    UserMarkedInactive,
    UserMarkedDeceased,
    SystemDetectedInactivity,
    LifecycleCompleted,
}
impl ::std::convert::From<&Self> for ThreadStateReason {
    fn from(value: &ThreadStateReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserMarkedInactive => write!(f, "UserMarkedInactive"),
            Self::UserMarkedDeceased => write!(f, "UserMarkedDeceased"),
            Self::SystemDetectedInactivity => write!(f, "SystemDetectedInactivity"),
            Self::LifecycleCompleted => write!(f, "LifecycleCompleted"),
        }
    }
}
impl ::std::str::FromStr for ThreadStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UserMarkedInactive" => Ok(Self::UserMarkedInactive),
            "UserMarkedDeceased" => Ok(Self::UserMarkedDeceased),
            "SystemDetectedInactivity" => Ok(Self::SystemDetectedInactivity),
            "LifecycleCompleted" => Ok(Self::LifecycleCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadStateReason {
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
    pub struct ColumnDefinition {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        primary_key: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::ColumnDefinitionType, ::std::string::String>,
        unique: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ColumnDefinition {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                nullable: Ok(Default::default()),
                primary_key: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                unique: Ok(Default::default()),
            }
        }
    }
    impl ColumnDefinition {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn primary_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_key: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ColumnDefinitionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn unique<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.unique = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unique: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ColumnDefinition> for super::ColumnDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ColumnDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                nullable: value.nullable?,
                primary_key: value.primary_key?,
                type_: value.type_?,
                unique: value.unique?,
            })
        }
    }
    impl ::std::convert::From<super::ColumnDefinition> for ColumnDefinition {
        fn from(value: super::ColumnDefinition) -> Self {
            Self {
                name: Ok(value.name),
                nullable: Ok(value.nullable),
                primary_key: Ok(value.primary_key),
                type_: Ok(value.type_),
                unique: Ok(value.unique),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct KeyValue {
        value: ::std::result::Result<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::collections::HashMap<::std::string::String, super::KeyValueValueValueValue>,
            >,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for KeyValue {
        fn default() -> Self {
            Self {
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl KeyValue {
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    ::std::string::String,
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::KeyValueValueValueValue,
                    >,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for value: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<KeyValue> for super::KeyValue {
        type Error = super::error::ConversionError;
        fn try_from(value: KeyValue) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::KeyValue> for KeyValue {
        fn from(value: super::KeyValue) -> Self {
            Self {
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadStateLogColumns {
        effective_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        metadata:
            ::std::result::Result<::std::option::Option<super::KeyValue>, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::ThreadStateReason>,
            ::std::string::String,
        >,
        state: ::std::result::Result<super::ThreadState, ::std::string::String>,
        thread_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        extra: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::ColumnDefinition>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ThreadStateLogColumns {
        fn default() -> Self {
            Self {
                effective_at: Err("no value supplied for effective_at".to_string()),
                metadata: Ok(Default::default()),
                reason: Ok(Default::default()),
                state: Err("no value supplied for state".to_string()),
                thread_id: Err("no value supplied for thread_id".to_string()),
                extra: Err("no value supplied for extra".to_string()),
            }
        }
    }
    impl ThreadStateLogColumns {
        pub fn effective_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.effective_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for effective_at: {}", e));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::KeyValue>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {}", e));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ThreadStateReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {}", e));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {}", e));
            self
        }
        pub fn thread_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.thread_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for thread_id: {}", e));
            self
        }
        pub fn extra<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::ColumnDefinition>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.extra = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for extra: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadStateLogColumns> for super::ThreadStateLogColumns {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadStateLogColumns,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                effective_at: value.effective_at?,
                metadata: value.metadata?,
                reason: value.reason?,
                state: value.state?,
                thread_id: value.thread_id?,
                extra: value.extra?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadStateLogColumns> for ThreadStateLogColumns {
        fn from(value: super::ThreadStateLogColumns) -> Self {
            Self {
                effective_at: Ok(value.effective_at),
                metadata: Ok(value.metadata),
                reason: Ok(value.reason),
                state: Ok(value.state),
                thread_id: Ok(value.thread_id),
                extra: Ok(value.extra),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThreadStateLogTable {
        columns: ::std::result::Result<super::ThreadStateLogColumns, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        table_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ThreadStateLogTable {
        fn default() -> Self {
            Self {
                columns: Err("no value supplied for columns".to_string()),
                description: Err("no value supplied for description".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                table_name: Err("no value supplied for table_name".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl ThreadStateLogTable {
        pub fn columns<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThreadStateLogColumns>,
            T::Error: ::std::fmt::Display,
        {
            self.columns = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for columns: {}", e));
            self
        }
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
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema_version: {}", e));
            self
        }
        pub fn table_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.table_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for table_name: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ThreadStateLogTable> for super::ThreadStateLogTable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThreadStateLogTable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                columns: value.columns?,
                description: value.description?,
                schema_version: value.schema_version?,
                table_name: value.table_name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::ThreadStateLogTable> for ThreadStateLogTable {
        fn from(value: super::ThreadStateLogTable) -> Self {
            Self {
                columns: Ok(value.columns),
                description: Ok(value.description),
                schema_version: Ok(value.schema_version),
                table_name: Ok(value.table_name),
                title: Ok(value.title),
            }
        }
    }
}
