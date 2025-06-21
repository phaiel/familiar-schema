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
#[doc = "A canonical definition for an optional ISO 8601 timestamp with timezone."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/primitives/NullableTimestamp.v1.json\","]
#[doc = "  \"title\": \"Nullable Timestamp\","]
#[doc = "  \"description\": \"A canonical definition for an optional ISO 8601 timestamp with timezone.\","]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/EndDate.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct NullableTimestamp(pub ::std::option::Option<::std::string::String>);
impl ::std::ops::Deref for NullableTimestamp {
    type Target = ::std::option::Option<::std::string::String>;
    fn deref(&self) -> &::std::option::Option<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<NullableTimestamp> for ::std::option::Option<::std::string::String> {
    fn from(value: NullableTimestamp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NullableTimestamp> for NullableTimestamp {
    fn from(value: &NullableTimestamp) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::option::Option<::std::string::String>> for NullableTimestamp {
    fn from(value: ::std::option::Option<::std::string::String>) -> Self {
        Self(value)
    }
}
