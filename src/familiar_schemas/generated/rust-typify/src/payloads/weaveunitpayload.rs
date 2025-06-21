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
#[doc = "`BaseUserContext`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BaseUserContext\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"user_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"session_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseUserContext {
    #[doc = "A reusable definition for a unique entity identifier."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub session_id: ::std::option::Option<::std::string::String>,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub user_id: ::std::string::String,
}
impl ::std::convert::From<&BaseUserContext> for BaseUserContext {
    fn from(value: &BaseUserContext) -> Self {
        value.clone()
    }
}
impl BaseUserContext {
    pub fn builder() -> builder::BaseUserContext {
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
#[doc = "`WeaveUnitData`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"WeaveUnitData\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"parent_weave_id\","]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"parent_weave_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"suggested_entity_type\": {"]
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
#[doc = "    \"text\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeaveUnitData {
    #[doc = "A reusable definition for a unique entity identifier."]
    pub parent_weave_id: ::std::string::String,
    #[doc = "A canonical enum of all 7 cognitive entity types."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub suggested_entity_type: ::std::option::Option<EntityType>,
    pub text: ::std::string::String,
}
impl ::std::convert::From<&WeaveUnitData> for WeaveUnitData {
    fn from(value: &WeaveUnitData) -> Self {
        value.clone()
    }
}
impl WeaveUnitData {
    pub fn builder() -> builder::WeaveUnitData {
        Default::default()
    }
}
#[doc = "A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/WeaveUnitPayload.v1.schema.json\","]
#[doc = "  \"title\": \"Weave Unit Payload\","]
#[doc = "  \"description\": \"A single, logical strand of information deconstructed from a Weave, enhanced with an entity type hint.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"data\","]
#[doc = "    \"user_context\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"data\": {"]
#[doc = "      \"title\": \"WeaveUnitData\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"parent_weave_id\","]
#[doc = "        \"text\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"parent_weave_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"suggested_entity_type\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/classification/EntityType.v1.json\","]
#[doc = "          \"title\": \"Entity Type\","]
#[doc = "          \"description\": \"A canonical enum of all 7 cognitive entity types.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Focus\","]
#[doc = "            \"Filament\","]
#[doc = "            \"Motif\","]
#[doc = "            \"Intent\","]
#[doc = "            \"Moment\","]
#[doc = "            \"Bond\","]
#[doc = "            \"Thread\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"text\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"user_context\": {"]
#[doc = "      \"title\": \"BaseUserContext\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"user_id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"session_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"user_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "          \"title\": \"UUID\","]
#[doc = "          \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"payloads\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"payloads/WeaveUnitPayload.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct WeaveUnitPayload {
    pub data: WeaveUnitData,
    pub user_context: BaseUserContext,
}
impl ::std::convert::From<&WeaveUnitPayload> for WeaveUnitPayload {
    fn from(value: &WeaveUnitPayload) -> Self {
        value.clone()
    }
}
impl WeaveUnitPayload {
    pub fn builder() -> builder::WeaveUnitPayload {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseUserContext {
        session_id: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseUserContext {
        fn default() -> Self {
            Self {
                session_id: Ok(Default::default()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl BaseUserContext {
        pub fn session_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for session_id: {}", e));
            self
        }
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseUserContext> for super::BaseUserContext {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseUserContext,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                session_id: value.session_id?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::BaseUserContext> for BaseUserContext {
        fn from(value: super::BaseUserContext) -> Self {
            Self {
                session_id: Ok(value.session_id),
                user_id: Ok(value.user_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeaveUnitData {
        parent_weave_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        suggested_entity_type:
            ::std::result::Result<::std::option::Option<super::EntityType>, ::std::string::String>,
        text: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WeaveUnitData {
        fn default() -> Self {
            Self {
                parent_weave_id: Err("no value supplied for parent_weave_id".to_string()),
                suggested_entity_type: Ok(Default::default()),
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl WeaveUnitData {
        pub fn parent_weave_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.parent_weave_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent_weave_id: {}", e));
            self
        }
        pub fn suggested_entity_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EntityType>>,
            T::Error: ::std::fmt::Display,
        {
            self.suggested_entity_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for suggested_entity_type: {}",
                    e
                )
            });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeaveUnitData> for super::WeaveUnitData {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeaveUnitData,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                parent_weave_id: value.parent_weave_id?,
                suggested_entity_type: value.suggested_entity_type?,
                text: value.text?,
            })
        }
    }
    impl ::std::convert::From<super::WeaveUnitData> for WeaveUnitData {
        fn from(value: super::WeaveUnitData) -> Self {
            Self {
                parent_weave_id: Ok(value.parent_weave_id),
                suggested_entity_type: Ok(value.suggested_entity_type),
                text: Ok(value.text),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WeaveUnitPayload {
        data: ::std::result::Result<super::WeaveUnitData, ::std::string::String>,
        user_context: ::std::result::Result<super::BaseUserContext, ::std::string::String>,
    }
    impl ::std::default::Default for WeaveUnitPayload {
        fn default() -> Self {
            Self {
                data: Err("no value supplied for data".to_string()),
                user_context: Err("no value supplied for user_context".to_string()),
            }
        }
    }
    impl WeaveUnitPayload {
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WeaveUnitData>,
            T::Error: ::std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
            self
        }
        pub fn user_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseUserContext>,
            T::Error: ::std::fmt::Display,
        {
            self.user_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_context: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WeaveUnitPayload> for super::WeaveUnitPayload {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WeaveUnitPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                data: value.data?,
                user_context: value.user_context?,
            })
        }
    }
    impl ::std::convert::From<super::WeaveUnitPayload> for WeaveUnitPayload {
        fn from(value: super::WeaveUnitPayload) -> Self {
            Self {
                data: Ok(value.data),
                user_context: Ok(value.user_context),
            }
        }
    }
}
