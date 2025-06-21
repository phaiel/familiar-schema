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
#[doc = "A canonical enum of the machine-readable reasons for a Bond state change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondStateReason.v1.json\","]
#[doc = "  \"title\": \"Bond State Reason\","]
#[doc = "  \"description\": \"A canonical enum of the machine-readable reasons for a Bond state change.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserInitiated\","]
#[doc = "    \"SystemDetectedStrain\","]
#[doc = "    \"PartnerAccountErased\","]
#[doc = "    \"RelationshipEnded\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/lifecycles/BondStateReason.json\""]
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
pub enum BondStateReason {
    UserInitiated,
    SystemDetectedStrain,
    PartnerAccountErased,
    RelationshipEnded,
}
impl ::std::convert::From<&Self> for BondStateReason {
    fn from(value: &BondStateReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BondStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserInitiated => write!(f, "UserInitiated"),
            Self::SystemDetectedStrain => write!(f, "SystemDetectedStrain"),
            Self::PartnerAccountErased => write!(f, "PartnerAccountErased"),
            Self::RelationshipEnded => write!(f, "RelationshipEnded"),
        }
    }
}
impl ::std::str::FromStr for BondStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UserInitiated" => Ok(Self::UserInitiated),
            "SystemDetectedStrain" => Ok(Self::SystemDetectedStrain),
            "PartnerAccountErased" => Ok(Self::PartnerAccountErased),
            "RelationshipEnded" => Ok(Self::RelationshipEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BondStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BondStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BondStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
