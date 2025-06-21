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
#[doc = "Describes the method or reason for an entity access event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/fields/AccessType.v1.json\","]
#[doc = "  \"title\": \"Access Type Field\","]
#[doc = "  \"description\": \"Describes the method or reason for an entity access event.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"DirectQuery\","]
#[doc = "    \"SearchResult\","]
#[doc = "    \"ConsolidationProcess\","]
#[doc = "    \"AgentAnalysis\","]
#[doc = "    \"UserInteraction\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/AccessType.json\""]
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
pub enum AccessTypeField {
    DirectQuery,
    SearchResult,
    ConsolidationProcess,
    AgentAnalysis,
    UserInteraction,
}
impl ::std::convert::From<&Self> for AccessTypeField {
    fn from(value: &AccessTypeField) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for AccessTypeField {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DirectQuery => write!(f, "DirectQuery"),
            Self::SearchResult => write!(f, "SearchResult"),
            Self::ConsolidationProcess => write!(f, "ConsolidationProcess"),
            Self::AgentAnalysis => write!(f, "AgentAnalysis"),
            Self::UserInteraction => write!(f, "UserInteraction"),
        }
    }
}
impl ::std::str::FromStr for AccessTypeField {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "DirectQuery" => Ok(Self::DirectQuery),
            "SearchResult" => Ok(Self::SearchResult),
            "ConsolidationProcess" => Ok(Self::ConsolidationProcess),
            "AgentAnalysis" => Ok(Self::AgentAnalysis),
            "UserInteraction" => Ok(Self::UserInteraction),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AccessTypeField {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
