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
#[doc = "Represents a single user within a tenant, including their role and join date."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/security/Member.v1.json\","]
#[doc = "  \"title\": \"Tenant Member\","]
#[doc = "  \"description\": \"Represents a single user within a tenant, including their role and join date.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"joined_at\","]
#[doc = "    \"role\","]
#[doc = "    \"user_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"joined_at\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"role\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/security/Role.v1.json\","]
#[doc = "      \"title\": \"Tenant Member Role\","]
#[doc = "      \"description\": \"Defines the roles a user can have within a tenant.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Admin\","]
#[doc = "        \"Member\","]
#[doc = "        \"Child\","]
#[doc = "        \"Guardian\","]
#[doc = "        \"ReadOnly\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/security/Member.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct TenantMember {
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub joined_at: ::std::string::String,
    #[doc = "Defines the roles a user can have within a tenant."]
    pub role: TenantMemberRole,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub user_id: ::std::string::String,
}
impl ::std::convert::From<&TenantMember> for TenantMember {
    fn from(value: &TenantMember) -> Self {
        value.clone()
    }
}
impl TenantMember {
    pub fn builder() -> builder::TenantMember {
        Default::default()
    }
}
#[doc = "Defines the roles a user can have within a tenant."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/security/Role.v1.json\","]
#[doc = "  \"title\": \"Tenant Member Role\","]
#[doc = "  \"description\": \"Defines the roles a user can have within a tenant.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Admin\","]
#[doc = "    \"Member\","]
#[doc = "    \"Child\","]
#[doc = "    \"Guardian\","]
#[doc = "    \"ReadOnly\""]
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
pub enum TenantMemberRole {
    Admin,
    Member,
    Child,
    Guardian,
    ReadOnly,
}
impl ::std::convert::From<&Self> for TenantMemberRole {
    fn from(value: &TenantMemberRole) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TenantMemberRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Admin => write!(f, "Admin"),
            Self::Member => write!(f, "Member"),
            Self::Child => write!(f, "Child"),
            Self::Guardian => write!(f, "Guardian"),
            Self::ReadOnly => write!(f, "ReadOnly"),
        }
    }
}
impl ::std::str::FromStr for TenantMemberRole {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Admin" => Ok(Self::Admin),
            "Member" => Ok(Self::Member),
            "Child" => Ok(Self::Child),
            "Guardian" => Ok(Self::Guardian),
            "ReadOnly" => Ok(Self::ReadOnly),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TenantMemberRole {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TenantMemberRole {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TenantMemberRole {
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
    pub struct TenantMember {
        joined_at: ::std::result::Result<::std::string::String, ::std::string::String>,
        role: ::std::result::Result<super::TenantMemberRole, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TenantMember {
        fn default() -> Self {
            Self {
                joined_at: Err("no value supplied for joined_at".to_string()),
                role: Err("no value supplied for role".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl TenantMember {
        pub fn joined_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.joined_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for joined_at: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TenantMemberRole>,
            T::Error: ::std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
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
    impl ::std::convert::TryFrom<TenantMember> for super::TenantMember {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TenantMember,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                joined_at: value.joined_at?,
                role: value.role?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::TenantMember> for TenantMember {
        fn from(value: super::TenantMember) -> Self {
            Self {
                joined_at: Ok(value.joined_at),
                role: Ok(value.role),
                user_id: Ok(value.user_id),
            }
        }
    }
}
