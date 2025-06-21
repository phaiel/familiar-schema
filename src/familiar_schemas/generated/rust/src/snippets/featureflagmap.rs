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
#[doc = "Schema for FeatureFlagMap"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/FeatureFlagMap.v1.schema.json\","]
#[doc = "  \"title\": \"FeatureFlagMap\","]
#[doc = "  \"description\": \"Schema for FeatureFlagMap\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"boolean\""]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/config/FeatureFlagMap.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct FeatureFlagMap(pub ::std::collections::HashMap<::std::string::String, bool>);
impl ::std::ops::Deref for FeatureFlagMap {
    type Target = ::std::collections::HashMap<::std::string::String, bool>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, bool> {
        &self.0
    }
}
impl ::std::convert::From<FeatureFlagMap>
    for ::std::collections::HashMap<::std::string::String, bool>
{
    fn from(value: FeatureFlagMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&FeatureFlagMap> for FeatureFlagMap {
    fn from(value: &FeatureFlagMap) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, bool>>
    for FeatureFlagMap
{
    fn from(value: ::std::collections::HashMap<::std::string::String, bool>) -> Self {
        Self(value)
    }
}
