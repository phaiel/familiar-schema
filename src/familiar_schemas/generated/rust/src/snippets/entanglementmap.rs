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
#[doc = "Schema for EntanglementMap"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/EntanglementMap.v1.schema.json\","]
#[doc = "  \"title\": \"EntanglementMap\","]
#[doc = "  \"description\": \"Schema for EntanglementMap\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"additionalProperties\": {"]
#[doc = "    \"type\": \"number\","]
#[doc = "    \"maximum\": 1.0,"]
#[doc = "    \"minimum\": 0.0"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/physics/EntanglementMap.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EntanglementMap(pub ::std::collections::HashMap<::std::string::String, f64>);
impl ::std::ops::Deref for EntanglementMap {
    type Target = ::std::collections::HashMap<::std::string::String, f64>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, f64> {
        &self.0
    }
}
impl ::std::convert::From<EntanglementMap>
    for ::std::collections::HashMap<::std::string::String, f64>
{
    fn from(value: EntanglementMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EntanglementMap> for EntanglementMap {
    fn from(value: &EntanglementMap) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, f64>>
    for EntanglementMap
{
    fn from(value: ::std::collections::HashMap<::std::string::String, f64>) -> Self {
        Self(value)
    }
}
