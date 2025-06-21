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
#[doc = "A canonical enum of all possible relationship types between Threads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/social/RelationshipType.v1.json\","]
#[doc = "  \"title\": \"Relationship Type\","]
#[doc = "  \"description\": \"A canonical enum of all possible relationship types between Threads.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Family\","]
#[doc = "    \"Friend\","]
#[doc = "    \"Romantic\","]
#[doc = "    \"Professional\","]
#[doc = "    \"Acquaintance\","]
#[doc = "    \"Adversarial\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/social/RelationshipType.json\""]
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
pub enum RelationshipType {
    Family,
    Friend,
    Romantic,
    Professional,
    Acquaintance,
    Adversarial,
}
impl ::std::convert::From<&Self> for RelationshipType {
    fn from(value: &RelationshipType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Family => write!(f, "Family"),
            Self::Friend => write!(f, "Friend"),
            Self::Romantic => write!(f, "Romantic"),
            Self::Professional => write!(f, "Professional"),
            Self::Acquaintance => write!(f, "Acquaintance"),
            Self::Adversarial => write!(f, "Adversarial"),
        }
    }
}
impl ::std::str::FromStr for RelationshipType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Family" => Ok(Self::Family),
            "Friend" => Ok(Self::Friend),
            "Romantic" => Ok(Self::Romantic),
            "Professional" => Ok(Self::Professional),
            "Acquaintance" => Ok(Self::Acquaintance),
            "Adversarial" => Ok(Self::Adversarial),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RelationshipType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RelationshipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RelationshipType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
