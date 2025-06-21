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
#[doc = "Applies a natural, exponential decay to the energy level of all Thread entities over time, representing the natural settling of cognitive activation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/laws/ThreadEnergyDecay.v1.schema.json\","]
#[doc = "  \"title\": \"Thread Energy Decay Law\","]
#[doc = "  \"description\": \"Applies a natural, exponential decay to the energy level of all Thread entities over time, representing the natural settling of cognitive activation.\","]
#[doc = "  \"affected_components\": ["]
#[doc = "    \"https://familiar.dev/schemas/components/UniversalPhysicsState.v1.schema.json\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"laws\","]
#[doc = "  \"execution_envelope\": {"]
#[doc = "    \"priority\": 8,"]
#[doc = "    \"timeout_ms\": 20,"]
#[doc = "    \"trigger\": \"system_schedule\""]
#[doc = "  },"]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"collapse_sensitive\": false,"]
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"laws/ThreadEnergyDecay.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ThreadEnergyDecayLaw(pub ::serde_json::Value);
impl ::std::ops::Deref for ThreadEnergyDecayLaw {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<ThreadEnergyDecayLaw> for ::serde_json::Value {
    fn from(value: ThreadEnergyDecayLaw) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ThreadEnergyDecayLaw> for ThreadEnergyDecayLaw {
    fn from(value: &ThreadEnergyDecayLaw) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for ThreadEnergyDecayLaw {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
