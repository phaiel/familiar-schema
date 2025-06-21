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
#[doc = "The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/QuantumCoherence.v1.schema.json\","]
#[doc = "  \"title\": \"QuantumCoherence\","]
#[doc = "  \"description\": \"The quantum coherence level of the entity, representing its degree of superposition. Null for classical entities.\","]
#[doc = "  \"default\": null,"]
#[doc = "  \"type\": ["]
#[doc = "    \"number\","]
#[doc = "    \"null\""]
#[doc = "  ],"]
#[doc = "  \"maximum\": 1.0,"]
#[doc = "  \"minimum\": 0.0,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/QuantumCoherence.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct QuantumCoherence(pub ::std::option::Option<f64>);
impl ::std::ops::Deref for QuantumCoherence {
    type Target = ::std::option::Option<f64>;
    fn deref(&self) -> &::std::option::Option<f64> {
        &self.0
    }
}
impl ::std::convert::From<QuantumCoherence> for ::std::option::Option<f64> {
    fn from(value: QuantumCoherence) -> Self {
        value.0
    }
}
impl ::std::convert::From<&QuantumCoherence> for QuantumCoherence {
    fn from(value: &QuantumCoherence) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::option::Option<f64>> for QuantumCoherence {
    fn from(value: ::std::option::Option<f64>) -> Self {
        Self(value)
    }
}
