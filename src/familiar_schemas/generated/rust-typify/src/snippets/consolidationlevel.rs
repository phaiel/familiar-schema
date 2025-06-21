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
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/system/ConsolidationLevel.json\""]
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
