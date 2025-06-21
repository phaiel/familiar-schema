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
#[doc = "A canonical enum for the types of cognitive dissonance the system can detect."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/system/DissonanceType.v1.json\","]
#[doc = "  \"title\": \"Dissonance Type\","]
#[doc = "  \"description\": \"A canonical enum for the types of cognitive dissonance the system can detect.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"TemporalOverlap\","]
#[doc = "    \"LogicalContradiction\","]
#[doc = "    \"LowConfidencePattern\","]
#[doc = "    \"UnresolvedEntity\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/system/DissonanceType.json\""]
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
pub enum DissonanceType {
    TemporalOverlap,
    LogicalContradiction,
    LowConfidencePattern,
    UnresolvedEntity,
}
impl ::std::convert::From<&Self> for DissonanceType {
    fn from(value: &DissonanceType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DissonanceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::TemporalOverlap => write!(f, "TemporalOverlap"),
            Self::LogicalContradiction => write!(f, "LogicalContradiction"),
            Self::LowConfidencePattern => write!(f, "LowConfidencePattern"),
            Self::UnresolvedEntity => write!(f, "UnresolvedEntity"),
        }
    }
}
impl ::std::str::FromStr for DissonanceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "TemporalOverlap" => Ok(Self::TemporalOverlap),
            "LogicalContradiction" => Ok(Self::LogicalContradiction),
            "LowConfidencePattern" => Ok(Self::LowConfidencePattern),
            "UnresolvedEntity" => Ok(Self::UnresolvedEntity),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DissonanceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
