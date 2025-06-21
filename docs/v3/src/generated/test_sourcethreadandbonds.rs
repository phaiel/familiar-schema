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
#[doc = "A list of Thread and Bond entity IDs that are the source for a derived cognitive entity like a Filament."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/fields/SourceThreadsAndBonds.v1.json\","]
#[doc = "  \"title\": \"Source Threads and Bonds Field\","]
#[doc = "  \"description\": \"A list of Thread and Bond entity IDs that are the source for a derived cognitive entity like a Filament.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "    \"title\": \"Entity ID Field\","]
#[doc = "    \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/SourceThreadAndBonds.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SourceThreadsAndBondsField(pub ::std::vec::Vec<::std::string::String>);
impl ::std::ops::Deref for SourceThreadsAndBondsField {
    type Target = ::std::vec::Vec<::std::string::String>;
    fn deref(&self) -> &::std::vec::Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<SourceThreadsAndBondsField> for ::std::vec::Vec<::std::string::String> {
    fn from(value: SourceThreadsAndBondsField) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SourceThreadsAndBondsField> for SourceThreadsAndBondsField {
    fn from(value: &SourceThreadsAndBondsField) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for SourceThreadsAndBondsField {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
