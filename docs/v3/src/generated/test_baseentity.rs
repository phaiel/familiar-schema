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
#[doc = "The absolute base schema for any persistent, identifiable object in the system, whether cognitive or systemic. Defines the universal properties shared by all entities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Entity.v1.schema.json\","]
#[doc = "  \"title\": \"Base Entity\","]
#[doc = "  \"description\": \"The absolute base schema for any persistent, identifiable object in the system, whether cognitive or systemic. Defines the universal properties shared by all entities.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"created_at\","]
#[doc = "    \"description\","]
#[doc = "    \"entity_id\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"tenant_id\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"entity_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tenant_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"_base/BaseEntity.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseEntity {
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub created_at: ::std::string::String,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub entity_id: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub tenant_id: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseEntity> for BaseEntity {
    fn from(value: &BaseEntity) -> Self {
        value.clone()
    }
}
impl BaseEntity {
    pub fn builder() -> builder::BaseEntity {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseEntity {
        created_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        entity_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        tenant_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseEntity {
        fn default() -> Self {
            Self {
                created_at: Err("no value supplied for created_at".to_string()),
                description: Err("no value supplied for description".to_string()),
                entity_id: Err("no value supplied for entity_id".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                tenant_id: Err("no value supplied for tenant_id".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseEntity {
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
    impl ::std::convert::TryFrom<BaseEntity> for super::BaseEntity {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseEntity,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                created_at: value.created_at?,
                description: value.description?,
                entity_id: value.entity_id?,
                schema_version: value.schema_version?,
                tenant_id: value.tenant_id?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseEntity> for BaseEntity {
        fn from(value: super::BaseEntity) -> Self {
            Self {
                created_at: Ok(value.created_at),
                description: Ok(value.description),
                entity_id: Ok(value.entity_id),
                schema_version: Ok(value.schema_version),
                tenant_id: Ok(value.tenant_id),
                title: Ok(value.title),
            }
        }
    }
}
