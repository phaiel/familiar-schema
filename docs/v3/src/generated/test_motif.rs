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
#[doc = "A canonical enum for the temporal consolidation level of a Motif or Filament."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/system/ConsolidationLevel.v1.json\","]
#[doc = "  \"title\": \"Consolidation Level\","]
#[doc = "  \"description\": \"A canonical enum for the temporal consolidation level of a Motif or Filament.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Daily\","]
#[doc = "    \"Weekly\","]
#[doc = "    \"Monthly\","]
#[doc = "    \"Yearly\","]
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
pub enum ConsolidationLevel {
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Archived,
}
impl ::std::convert::From<&Self> for ConsolidationLevel {
    fn from(value: &ConsolidationLevel) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ConsolidationLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Daily => write!(f, "Daily"),
            Self::Weekly => write!(f, "Weekly"),
            Self::Monthly => write!(f, "Monthly"),
            Self::Yearly => write!(f, "Yearly"),
            Self::Archived => write!(f, "Archived"),
        }
    }
}
impl ::std::str::FromStr for ConsolidationLevel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Daily" => Ok(Self::Daily),
            "Weekly" => Ok(Self::Weekly),
            "Monthly" => Ok(Self::Monthly),
            "Yearly" => Ok(Self::Yearly),
            "Archived" => Ok(Self::Archived),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ConsolidationLevel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ConsolidationLevel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ConsolidationLevel {
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
#[doc = "Tracks the temporal consolidation state of a Motif or Filament."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/components/ConsolidationState.v1.schema.json\","]
#[doc = "  \"title\": \"Consolidation State Component\","]
#[doc = "  \"description\": \"Tracks the temporal consolidation state of a Motif or Filament.\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"ConsolidationStateFields\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"consolidation_level\","]
#[doc = "        \"last_consolidated_at\","]
#[doc = "        \"next_consolidation_due\","]
#[doc = "        \"source_entity_count\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"consolidation_level\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/system/ConsolidationLevel.v1.json\","]
#[doc = "          \"title\": \"Consolidation Level\","]
#[doc = "          \"description\": \"A canonical enum for the temporal consolidation level of a Motif or Filament.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Daily\","]
#[doc = "            \"Weekly\","]
#[doc = "            \"Monthly\","]
#[doc = "            \"Yearly\","]
#[doc = "            \"Archived\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"last_consolidated_at\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "          \"title\": \"Nullable Timestamp\","]
#[doc = "          \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"next_consolidation_due\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "          \"title\": \"Nullable Timestamp\","]
#[doc = "          \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"source_entity_count\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
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
#[doc = "  \"schema_version\": \"1.2.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ConsolidationStateComponent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<ConsolidationStateFields>,
}
impl ::std::convert::From<&ConsolidationStateComponent> for ConsolidationStateComponent {
    fn from(value: &ConsolidationStateComponent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConsolidationStateComponent {
    fn default() -> Self {
        Self {
            fields: Default::default(),
        }
    }
}
impl ConsolidationStateComponent {
    pub fn builder() -> builder::ConsolidationStateComponent {
        Default::default()
    }
}
#[doc = "`ConsolidationStateFields`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ConsolidationStateFields\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"consolidation_level\","]
#[doc = "    \"last_consolidated_at\","]
#[doc = "    \"next_consolidation_due\","]
#[doc = "    \"source_entity_count\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"consolidation_level\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/system/ConsolidationLevel.v1.json\","]
#[doc = "      \"title\": \"Consolidation Level\","]
#[doc = "      \"description\": \"A canonical enum for the temporal consolidation level of a Motif or Filament.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Daily\","]
#[doc = "        \"Weekly\","]
#[doc = "        \"Monthly\","]
#[doc = "        \"Yearly\","]
#[doc = "        \"Archived\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"last_consolidated_at\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "      \"title\": \"Nullable Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"next_consolidation_due\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "      \"title\": \"Nullable Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"source_entity_count\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConsolidationStateFields {
    #[doc = "A canonical enum for the temporal consolidation level of a Motif or Filament."]
    pub consolidation_level: ConsolidationLevel,
    #[doc = "A canonical definition for an optional ISO 8601 timestamp with timezone."]
    pub last_consolidated_at: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for an optional ISO 8601 timestamp with timezone."]
    pub next_consolidation_due: ::std::option::Option<::std::string::String>,
    pub source_entity_count: ::std::num::NonZeroU64,
}
impl ::std::convert::From<&ConsolidationStateFields> for ConsolidationStateFields {
    fn from(value: &ConsolidationStateFields) -> Self {
        value.clone()
    }
}
impl ConsolidationStateFields {
    pub fn builder() -> builder::ConsolidationStateFields {
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
#[doc = "Component references temporarily disabled for pipeline testing"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MotifComponents\","]
#[doc = "  \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"consolidation\","]
#[doc = "    \"content\","]
#[doc = "    \"gdpr\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"_comment\": {"]
#[doc = "      \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"consolidation\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/ConsolidationState.v1.schema.json\","]
#[doc = "      \"title\": \"Consolidation State Component\","]
#[doc = "      \"description\": \"Tracks the temporal consolidation state of a Motif or Filament.\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"ConsolidationStateFields\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"consolidation_level\","]
#[doc = "            \"last_consolidated_at\","]
#[doc = "            \"next_consolidation_due\","]
#[doc = "            \"source_entity_count\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"consolidation_level\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/system/ConsolidationLevel.v1.json\","]
#[doc = "              \"title\": \"Consolidation Level\","]
#[doc = "              \"description\": \"A canonical enum for the temporal consolidation level of a Motif or Filament.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Daily\","]
#[doc = "                \"Weekly\","]
#[doc = "                \"Monthly\","]
#[doc = "                \"Yearly\","]
#[doc = "                \"Archived\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"last_consolidated_at\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "              \"title\": \"Nullable Timestamp\","]
#[doc = "              \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"next_consolidation_due\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "              \"title\": \"Nullable Timestamp\","]
#[doc = "              \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "              \"type\": ["]
#[doc = "                \"string\","]
#[doc = "                \"null\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"source_entity_count\": {"]
#[doc = "              \"type\": \"integer\","]
#[doc = "              \"minimum\": 1.0"]
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
#[doc = "      \"schema_version\": \"1.2.0\""]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/MotifContent.v1.schema.json\","]
#[doc = "      \"title\": \"Motif Content Component\","]
#[doc = "      \"description\": \"Defines the emergent pattern content of a Motif entity. This is a quantum component.\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"fields\": {"]
#[doc = "          \"title\": \"MotifContentFields\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"motif_description\","]
#[doc = "            \"motif_type\","]
#[doc = "            \"source_entanglements\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"motif_description\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "              \"title\": \"Description Field\","]
#[doc = "              \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "              \"default\": \"\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"motif_type\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/physics/MotifType.v1.json\","]
#[doc = "              \"title\": \"Motif Type\","]
#[doc = "              \"description\": \"A canonical enum of the types of recurring themes or motifs in memory.\","]
#[doc = "              \"type\": \"string\","]
#[doc = "              \"enum\": ["]
#[doc = "                \"Behavioral\","]
#[doc = "                \"Emotional\","]
#[doc = "                \"Situational\","]
#[doc = "                \"Temporal\""]
#[doc = "              ]"]
#[doc = "            },"]
#[doc = "            \"source_entanglements\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "              \"title\": \"Entity ID List\","]
#[doc = "              \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"uniqueItems\": true"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"quantum\","]
#[doc = "        \"is_quantum\": true"]
#[doc = "      },"]
#[doc = "      \"schema_version\": \"1.4.0\""]
#[doc = "    },"]
#[doc = "    \"gdpr\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/GDPRDependency.v1.schema.json\","]
#[doc = "      \"title\": \"GDPR Dependency Component\","]
#[doc = "      \"description\": \"Tracks data provenance to enable compliant cascading deletions for GDPR's Right to Erasure.\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"contributing_users\": {"]
#[doc = "          \"description\": \"A set of User IDs whose data contributed to this entity's existence or state.\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"uuid\""]
#[doc = "          },"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"upstream_dependencies\": {"]
#[doc = "          \"description\": \"A list of source entity IDs (e.g., Moments) that this entity (e.g., a Motif) depends on.\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"uuid\""]
#[doc = "          },"]
#[doc = "          \"type\": \"array\""]
#[doc = "        },"]
#[doc = "        \"user_evidence_weights\": {"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minimum\": 0.0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
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
pub struct MotifComponents {
    #[doc = "TODO: Re-enable component references when component schemas are available"]
    #[serde(
        rename = "_comment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comment: ::std::option::Option<::std::string::String>,
    pub consolidation: ConsolidationStateComponent,
    pub content: MotifContentComponent,
    #[doc = "Tracks data provenance to enable compliant cascading deletions for GDPR's Right to Erasure."]
    pub gdpr: ::serde_json::Value,
}
impl ::std::convert::From<&MotifComponents> for MotifComponents {
    fn from(value: &MotifComponents) -> Self {
        value.clone()
    }
}
impl MotifComponents {
    pub fn builder() -> builder::MotifComponents {
        Default::default()
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
#[doc = "  \"properties\": {"]
#[doc = "    \"fields\": {"]
#[doc = "      \"title\": \"MotifContentFields\","]
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
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"engine\": \"quantum\","]
#[doc = "    \"is_quantum\": true"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.4.0\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MotifContentComponent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub fields: ::std::option::Option<MotifContentFields>,
}
impl ::std::convert::From<&MotifContentComponent> for MotifContentComponent {
    fn from(value: &MotifContentComponent) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for MotifContentComponent {
    fn default() -> Self {
        Self {
            fields: Default::default(),
        }
    }
}
impl MotifContentComponent {
    pub fn builder() -> builder::MotifContentComponent {
        Default::default()
    }
}
#[doc = "`MotifContentFields`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MotifContentFields\","]
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
#[doc = "A quantum entity representing a recurring pattern of subjective experiences, derived from the consolidation of EntanglementState entities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/Motif.v1.schema.json\","]
#[doc = "  \"title\": \"Motif Entity\","]
#[doc = "  \"description\": \"A quantum entity representing a recurring pattern of subjective experiences, derived from the consolidation of EntanglementState entities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"entity_type\","]
#[doc = "    \"physics_state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"MotifComponents\","]
#[doc = "      \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"consolidation\","]
#[doc = "        \"content\","]
#[doc = "        \"gdpr\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"_comment\": {"]
#[doc = "          \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"consolidation\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/ConsolidationState.v1.schema.json\","]
#[doc = "          \"title\": \"Consolidation State Component\","]
#[doc = "          \"description\": \"Tracks the temporal consolidation state of a Motif or Filament.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"ConsolidationStateFields\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"consolidation_level\","]
#[doc = "                \"last_consolidated_at\","]
#[doc = "                \"next_consolidation_due\","]
#[doc = "                \"source_entity_count\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"consolidation_level\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/system/ConsolidationLevel.v1.json\","]
#[doc = "                  \"title\": \"Consolidation Level\","]
#[doc = "                  \"description\": \"A canonical enum for the temporal consolidation level of a Motif or Filament.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Daily\","]
#[doc = "                    \"Weekly\","]
#[doc = "                    \"Monthly\","]
#[doc = "                    \"Yearly\","]
#[doc = "                    \"Archived\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"last_consolidated_at\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "                  \"title\": \"Nullable Timestamp\","]
#[doc = "                  \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "                  \"type\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"null\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"next_consolidation_due\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "                  \"title\": \"Nullable Timestamp\","]
#[doc = "                  \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "                  \"type\": ["]
#[doc = "                    \"string\","]
#[doc = "                    \"null\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"source_entity_count\": {"]
#[doc = "                  \"type\": \"integer\","]
#[doc = "                  \"minimum\": 1.0"]
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
#[doc = "          \"schema_version\": \"1.2.0\""]
#[doc = "        },"]
#[doc = "        \"content\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/MotifContent.v1.schema.json\","]
#[doc = "          \"title\": \"Motif Content Component\","]
#[doc = "          \"description\": \"Defines the emergent pattern content of a Motif entity. This is a quantum component.\","]
#[doc = "          \"properties\": {"]
#[doc = "            \"fields\": {"]
#[doc = "              \"title\": \"MotifContentFields\","]
#[doc = "              \"type\": \"object\","]
#[doc = "              \"required\": ["]
#[doc = "                \"motif_description\","]
#[doc = "                \"motif_type\","]
#[doc = "                \"source_entanglements\""]
#[doc = "              ],"]
#[doc = "              \"properties\": {"]
#[doc = "                \"motif_description\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                  \"title\": \"Description Field\","]
#[doc = "                  \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                  \"default\": \"\","]
#[doc = "                  \"type\": \"string\""]
#[doc = "                },"]
#[doc = "                \"motif_type\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/physics/MotifType.v1.json\","]
#[doc = "                  \"title\": \"Motif Type\","]
#[doc = "                  \"description\": \"A canonical enum of the types of recurring themes or motifs in memory.\","]
#[doc = "                  \"type\": \"string\","]
#[doc = "                  \"enum\": ["]
#[doc = "                    \"Behavioral\","]
#[doc = "                    \"Emotional\","]
#[doc = "                    \"Situational\","]
#[doc = "                    \"Temporal\""]
#[doc = "                  ]"]
#[doc = "                },"]
#[doc = "                \"source_entanglements\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "                  \"title\": \"Entity ID List\","]
#[doc = "                  \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "                  \"type\": \"array\","]
#[doc = "                  \"items\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"uniqueItems\": true"]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"additionalProperties\": false"]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"quantum\","]
#[doc = "            \"is_quantum\": true"]
#[doc = "          },"]
#[doc = "          \"schema_version\": \"1.4.0\""]
#[doc = "        },"]
#[doc = "        \"gdpr\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/GDPRDependency.v1.schema.json\","]
#[doc = "          \"title\": \"GDPR Dependency Component\","]
#[doc = "          \"description\": \"Tracks data provenance to enable compliant cascading deletions for GDPR's Right to Erasure.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"contributing_users\": {"]
#[doc = "              \"description\": \"A set of User IDs whose data contributed to this entity's existence or state.\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"uuid\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"uniqueItems\": true"]
#[doc = "            },"]
#[doc = "            \"upstream_dependencies\": {"]
#[doc = "              \"description\": \"A list of source entity IDs (e.g., Moments) that this entity (e.g., a Motif) depends on.\","]
#[doc = "              \"items\": {"]
#[doc = "                \"type\": \"uuid\""]
#[doc = "              },"]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"user_evidence_weights\": {"]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minimum\": 0.0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
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
#[doc = "      \"title\": \"MotifPhysicsState\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"quantum\","]
#[doc = "        \"universal\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"quantum\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/components/QuantumState.v1.schema.json\","]
#[doc = "          \"title\": \"Quantum State Component\","]
#[doc = "          \"description\": \"Manages the quantum properties of an entity, including its superposition and entanglement.\","]
#[doc = "          \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "          \"fields\": {"]
#[doc = "            \"coherence_score\": {"]
#[doc = "              \"description\": \"The purity of the quantum state (0.0 = maximally mixed, 1.0 = pure).\","]
#[doc = "              \"maximum\": 1.0,"]
#[doc = "              \"minimum\": 0.0,"]
#[doc = "              \"type\": \"number\""]
#[doc = "            },"]
#[doc = "            \"density_matrix\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/physics/DensityMatrix.v1.json\","]
#[doc = "              \"description\": \"A 2x2 matrix of complex numbers representing a quantum state.\","]
#[doc = "              \"items\": {"]
#[doc = "                \"items\": {"]
#[doc = "                  \"$id\": \"https://familiar.dev/schemas/types/physics/ComplexNumber.v1.json\","]
#[doc = "                  \"description\": \"Represents a complex number with real and imaginary parts.\","]
#[doc = "                  \"properties\": {"]
#[doc = "                    \"imaginary\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    },"]
#[doc = "                    \"real\": {"]
#[doc = "                      \"type\": \"number\""]
#[doc = "                    }"]
#[doc = "                  },"]
#[doc = "                  \"required\": ["]
#[doc = "                    \"real\","]
#[doc = "                    \"imaginary\""]
#[doc = "                  ],"]
#[doc = "                  \"title\": \"Complex Number\","]
#[doc = "                  \"type\": \"object\""]
#[doc = "                },"]
#[doc = "                \"maxItems\": 2,"]
#[doc = "                \"minItems\": 2,"]
#[doc = "                \"type\": \"array\""]
#[doc = "              },"]
#[doc = "              \"maxItems\": 2,"]
#[doc = "              \"minItems\": 2,"]
#[doc = "              \"title\": \"Density Matrix\","]
#[doc = "              \"type\": \"array\""]
#[doc = "            },"]
#[doc = "            \"entanglement_network\": {"]
#[doc = "              \"additionalProperties\": {"]
#[doc = "                \"maximum\": 1.0,"]
#[doc = "                \"minimum\": 0.0,"]
#[doc = "                \"type\": \"number\""]
#[doc = "              },"]
#[doc = "              \"type\": \"object\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"physics_properties\": {"]
#[doc = "            \"engine\": \"quantum\","]
#[doc = "            \"is_quantum\": true"]
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
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"source_file\": \"entities/Motif.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct MotifEntity {
    pub components: MotifComponents,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    pub entity_type: EntityType,
    pub physics_state: MotifPhysicsState,
}
impl ::std::convert::From<&MotifEntity> for MotifEntity {
    fn from(value: &MotifEntity) -> Self {
        value.clone()
    }
}
impl MotifEntity {
    pub fn builder() -> builder::MotifEntity {
        Default::default()
    }
}
#[doc = "`MotifPhysicsState`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"MotifPhysicsState\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"quantum\","]
#[doc = "    \"universal\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"quantum\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/components/QuantumState.v1.schema.json\","]
#[doc = "      \"title\": \"Quantum State Component\","]
#[doc = "      \"description\": \"Manages the quantum properties of an entity, including its superposition and entanglement.\","]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\","]
#[doc = "      \"fields\": {"]
#[doc = "        \"coherence_score\": {"]
#[doc = "          \"description\": \"The purity of the quantum state (0.0 = maximally mixed, 1.0 = pure).\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0,"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"density_matrix\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/physics/DensityMatrix.v1.json\","]
#[doc = "          \"description\": \"A 2x2 matrix of complex numbers representing a quantum state.\","]
#[doc = "          \"items\": {"]
#[doc = "            \"items\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/types/physics/ComplexNumber.v1.json\","]
#[doc = "              \"description\": \"Represents a complex number with real and imaginary parts.\","]
#[doc = "              \"properties\": {"]
#[doc = "                \"imaginary\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                },"]
#[doc = "                \"real\": {"]
#[doc = "                  \"type\": \"number\""]
#[doc = "                }"]
#[doc = "              },"]
#[doc = "              \"required\": ["]
#[doc = "                \"real\","]
#[doc = "                \"imaginary\""]
#[doc = "              ],"]
#[doc = "              \"title\": \"Complex Number\","]
#[doc = "              \"type\": \"object\""]
#[doc = "            },"]
#[doc = "            \"maxItems\": 2,"]
#[doc = "            \"minItems\": 2,"]
#[doc = "            \"type\": \"array\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 2,"]
#[doc = "          \"minItems\": 2,"]
#[doc = "          \"title\": \"Density Matrix\","]
#[doc = "          \"type\": \"array\""]
#[doc = "        },"]
#[doc = "        \"entanglement_network\": {"]
#[doc = "          \"additionalProperties\": {"]
#[doc = "            \"maximum\": 1.0,"]
#[doc = "            \"minimum\": 0.0,"]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"type\": \"object\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"physics_properties\": {"]
#[doc = "        \"engine\": \"quantum\","]
#[doc = "        \"is_quantum\": true"]
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
pub struct MotifPhysicsState {
    #[doc = "Manages the quantum properties of an entity, including its superposition and entanglement."]
    pub quantum: ::serde_json::Value,
    pub universal: UniversalPhysicsStateComponent,
}
impl ::std::convert::From<&MotifPhysicsState> for MotifPhysicsState {
    fn from(value: &MotifPhysicsState) -> Self {
        value.clone()
    }
}
impl MotifPhysicsState {
    pub fn builder() -> builder::MotifPhysicsState {
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
impl ::std::default::Default for UniversalPhysicsStateComponent {
    fn default() -> Self {
        Self {
            fields: Default::default(),
        }
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
    pub struct ConsolidationStateComponent {
        fields: ::std::result::Result<
            ::std::option::Option<super::ConsolidationStateFields>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConsolidationStateComponent {
        fn default() -> Self {
            Self {
                fields: Ok(Default::default()),
            }
        }
    }
    impl ConsolidationStateComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConsolidationStateFields>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConsolidationStateComponent> for super::ConsolidationStateComponent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConsolidationStateComponent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                fields: value.fields?,
            })
        }
    }
    impl ::std::convert::From<super::ConsolidationStateComponent> for ConsolidationStateComponent {
        fn from(value: super::ConsolidationStateComponent) -> Self {
            Self {
                fields: Ok(value.fields),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConsolidationStateFields {
        consolidation_level:
            ::std::result::Result<super::ConsolidationLevel, ::std::string::String>,
        last_consolidated_at: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        next_consolidation_due: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        source_entity_count: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for ConsolidationStateFields {
        fn default() -> Self {
            Self {
                consolidation_level: Err("no value supplied for consolidation_level".to_string()),
                last_consolidated_at: Err("no value supplied for last_consolidated_at".to_string()),
                next_consolidation_due: Err(
                    "no value supplied for next_consolidation_due".to_string()
                ),
                source_entity_count: Err("no value supplied for source_entity_count".to_string()),
            }
        }
    }
    impl ConsolidationStateFields {
        pub fn consolidation_level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConsolidationLevel>,
            T::Error: ::std::fmt::Display,
        {
            self.consolidation_level = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for consolidation_level: {}",
                    e
                )
            });
            self
        }
        pub fn last_consolidated_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_consolidated_at = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for last_consolidated_at: {}",
                    e
                )
            });
            self
        }
        pub fn next_consolidation_due<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.next_consolidation_due = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for next_consolidation_due: {}",
                    e
                )
            });
            self
        }
        pub fn source_entity_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.source_entity_count = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for source_entity_count: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ConsolidationStateFields> for super::ConsolidationStateFields {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConsolidationStateFields,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                consolidation_level: value.consolidation_level?,
                last_consolidated_at: value.last_consolidated_at?,
                next_consolidation_due: value.next_consolidation_due?,
                source_entity_count: value.source_entity_count?,
            })
        }
    }
    impl ::std::convert::From<super::ConsolidationStateFields> for ConsolidationStateFields {
        fn from(value: super::ConsolidationStateFields) -> Self {
            Self {
                consolidation_level: Ok(value.consolidation_level),
                last_consolidated_at: Ok(value.last_consolidated_at),
                next_consolidation_due: Ok(value.next_consolidation_due),
                source_entity_count: Ok(value.source_entity_count),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MotifComponents {
        comment: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        consolidation:
            ::std::result::Result<super::ConsolidationStateComponent, ::std::string::String>,
        content: ::std::result::Result<super::MotifContentComponent, ::std::string::String>,
        gdpr: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MotifComponents {
        fn default() -> Self {
            Self {
                comment: Ok(Default::default()),
                consolidation: Err("no value supplied for consolidation".to_string()),
                content: Err("no value supplied for content".to_string()),
                gdpr: Err("no value supplied for gdpr".to_string()),
            }
        }
    }
    impl MotifComponents {
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
        pub fn consolidation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ConsolidationStateComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.consolidation = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for consolidation: {}", e));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MotifContentComponent>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn gdpr<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.gdpr = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for gdpr: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MotifComponents> for super::MotifComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MotifComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comment: value.comment?,
                consolidation: value.consolidation?,
                content: value.content?,
                gdpr: value.gdpr?,
            })
        }
    }
    impl ::std::convert::From<super::MotifComponents> for MotifComponents {
        fn from(value: super::MotifComponents) -> Self {
            Self {
                comment: Ok(value.comment),
                consolidation: Ok(value.consolidation),
                content: Ok(value.content),
                gdpr: Ok(value.gdpr),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MotifContentComponent {
        fields: ::std::result::Result<
            ::std::option::Option<super::MotifContentFields>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for MotifContentComponent {
        fn default() -> Self {
            Self {
                fields: Ok(Default::default()),
            }
        }
    }
    impl MotifContentComponent {
        pub fn fields<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MotifContentFields>>,
            T::Error: ::std::fmt::Display,
        {
            self.fields = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fields: {}", e));
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
            })
        }
    }
    impl ::std::convert::From<super::MotifContentComponent> for MotifContentComponent {
        fn from(value: super::MotifContentComponent) -> Self {
            Self {
                fields: Ok(value.fields),
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
    #[derive(Clone, Debug)]
    pub struct MotifEntity {
        components: ::std::result::Result<super::MotifComponents, ::std::string::String>,
        entity_type: ::std::result::Result<super::EntityType, ::std::string::String>,
        physics_state: ::std::result::Result<super::MotifPhysicsState, ::std::string::String>,
    }
    impl ::std::default::Default for MotifEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                physics_state: Err("no value supplied for physics_state".to_string()),
            }
        }
    }
    impl MotifEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MotifComponents>,
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
            T: ::std::convert::TryInto<super::MotifPhysicsState>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for physics_state: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<MotifEntity> for super::MotifEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MotifEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                entity_type: value.entity_type?,
                physics_state: value.physics_state?,
            })
        }
    }
    impl ::std::convert::From<super::MotifEntity> for MotifEntity {
        fn from(value: super::MotifEntity) -> Self {
            Self {
                components: Ok(value.components),
                entity_type: Ok(value.entity_type),
                physics_state: Ok(value.physics_state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MotifPhysicsState {
        quantum: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        universal:
            ::std::result::Result<super::UniversalPhysicsStateComponent, ::std::string::String>,
    }
    impl ::std::default::Default for MotifPhysicsState {
        fn default() -> Self {
            Self {
                quantum: Err("no value supplied for quantum".to_string()),
                universal: Err("no value supplied for universal".to_string()),
            }
        }
    }
    impl MotifPhysicsState {
        pub fn quantum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.quantum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for quantum: {}", e));
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
    impl ::std::convert::TryFrom<MotifPhysicsState> for super::MotifPhysicsState {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MotifPhysicsState,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                quantum: value.quantum?,
                universal: value.universal?,
            })
        }
    }
    impl ::std::convert::From<super::MotifPhysicsState> for MotifPhysicsState {
        fn from(value: super::MotifPhysicsState) -> Self {
            Self {
                quantum: Ok(value.quantum),
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
