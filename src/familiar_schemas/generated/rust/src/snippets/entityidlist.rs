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
#[doc = "A canonical definition for a list of unique entity identifiers (UUIDs)."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/primitives/EntityIdList.v1.json\","]
#[doc = "  \"title\": \"Entity ID List\","]
#[doc = "  \"description\": \"A canonical definition for a list of unique entity identifiers (UUIDs).\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"string\""]
#[doc = "  },"]
#[doc = "  \"uniqueItems\": true,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/EntityIdList.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EntityIdList(pub Vec<::std::string::String>);
impl ::std::ops::Deref for EntityIdList {
    type Target = Vec<::std::string::String>;
    fn deref(&self) -> &Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<EntityIdList> for Vec<::std::string::String> {
    fn from(value: EntityIdList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EntityIdList> for EntityIdList {
    fn from(value: &EntityIdList) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::std::string::String>> for EntityIdList {
    fn from(value: Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
