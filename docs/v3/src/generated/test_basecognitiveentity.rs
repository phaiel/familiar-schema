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
#[doc = "  \"title\": \"BaseCognitiveComponents\","]
#[doc = "  \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"_comment\": {"]
#[doc = "      \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseCognitiveComponents {
    #[doc = "TODO: Re-enable component references when component schemas are available"]
    #[serde(
        rename = "_comment",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub comment: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BaseCognitiveComponents> for BaseCognitiveComponents {
    fn from(value: &BaseCognitiveComponents) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for BaseCognitiveComponents {
    fn default() -> Self {
        Self {
            comment: Default::default(),
        }
    }
}
impl BaseCognitiveComponents {
    pub fn builder() -> builder::BaseCognitiveComponents {
        Default::default()
    }
}
#[doc = "The base for all 7 cognitive entities. Inherits from BaseEntity and adds mandatory physics and manifold components (Rules 7 & 8)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/entities/BaseCognitiveEntity.v1.schema.json\","]
#[doc = "  \"title\": \"Base Cognitive Entity\","]
#[doc = "  \"description\": \"The base for all 7 cognitive entities. Inherits from BaseEntity and adds mandatory physics and manifold components (Rules 7 & 8).\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"components\","]
#[doc = "    \"created_at\","]
#[doc = "    \"entity_id\","]
#[doc = "    \"entity_type\","]
#[doc = "    \"tenant_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"components\": {"]
#[doc = "      \"title\": \"BaseCognitiveComponents\","]
#[doc = "      \"description\": \"Component references temporarily disabled for pipeline testing\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"_comment\": {"]
#[doc = "          \"description\": \"TODO: Re-enable component references when component schemas are available\","]
#[doc = "          \"type\": \"string\""]
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
#[doc = "    \"tenant_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"entities\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"entities/_base/BaseCognitiveEntity.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseCognitiveEntity {
    pub components: BaseCognitiveComponents,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub created_at: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub entity_id: ::std::string::String,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    pub entity_type: EntityType,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub tenant_id: ::std::string::String,
}
impl ::std::convert::From<&BaseCognitiveEntity> for BaseCognitiveEntity {
    fn from(value: &BaseCognitiveEntity) -> Self {
        value.clone()
    }
}
impl BaseCognitiveEntity {
    pub fn builder() -> builder::BaseCognitiveEntity {
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
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseCognitiveComponents {
        comment: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseCognitiveComponents {
        fn default() -> Self {
            Self {
                comment: Ok(Default::default()),
            }
        }
    }
    impl BaseCognitiveComponents {
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
    }
    impl ::std::convert::TryFrom<BaseCognitiveComponents> for super::BaseCognitiveComponents {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseCognitiveComponents,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                comment: value.comment?,
            })
        }
    }
    impl ::std::convert::From<super::BaseCognitiveComponents> for BaseCognitiveComponents {
        fn from(value: super::BaseCognitiveComponents) -> Self {
            Self {
                comment: Ok(value.comment),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BaseCognitiveEntity {
        components: ::std::result::Result<super::BaseCognitiveComponents, ::std::string::String>,
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        entity_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        entity_type: ::std::result::Result<super::EntityType, ::std::string::String>,
        tenant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseCognitiveEntity {
        fn default() -> Self {
            Self {
                components: Err("no value supplied for components".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                entity_id: Err("no value supplied for entity_id".to_string()),
                entity_type: Err("no value supplied for entity_type".to_string()),
                tenant_id: Err("no value supplied for tenant_id".to_string()),
            }
        }
    }
    impl BaseCognitiveEntity {
        pub fn components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseCognitiveComponents>,
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
            T: ::std::convert::TryInto<super::EntityType>,
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
    }
    impl ::std::convert::TryFrom<BaseCognitiveEntity> for super::BaseCognitiveEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseCognitiveEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                components: value.components?,
                created_at: value.created_at?,
                entity_id: value.entity_id?,
                entity_type: value.entity_type?,
                tenant_id: value.tenant_id?,
            })
        }
    }
    impl ::std::convert::From<super::BaseCognitiveEntity> for BaseCognitiveEntity {
        fn from(value: super::BaseCognitiveEntity) -> Self {
            Self {
                components: Ok(value.components),
                created_at: Ok(value.created_at),
                entity_id: Ok(value.entity_id),
                entity_type: Ok(value.entity_type),
                tenant_id: Ok(value.tenant_id),
            }
        }
    }
}
