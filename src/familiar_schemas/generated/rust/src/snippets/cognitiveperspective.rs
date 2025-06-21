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
#[doc = "A 3D vector representing the 'spin' or perspective on a cognitive entity, orthogonal to its manifold position."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/physics/CognitivePerspective.v1.json\","]
#[doc = "  \"title\": \"Cognitive Perspective Vector\","]
#[doc = "  \"description\": \"A 3D vector representing the 'spin' or perspective on a cognitive entity, orthogonal to its manifold position.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"number\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 3,"]
#[doc = "  \"minItems\": 3,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/physics/CognitivePerspective.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct CognitivePerspectiveVector(pub [f64; 3usize]);
impl ::std::ops::Deref for CognitivePerspectiveVector {
    type Target = [f64; 3usize];
    fn deref(&self) -> &[f64; 3usize] {
        &self.0
    }
}
impl ::std::convert::From<CognitivePerspectiveVector> for [f64; 3usize] {
    fn from(value: CognitivePerspectiveVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&CognitivePerspectiveVector> for CognitivePerspectiveVector {
    fn from(value: &CognitivePerspectiveVector) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<[f64; 3usize]> for CognitivePerspectiveVector {
    fn from(value: [f64; 3usize]) -> Self {
        Self(value)
    }
}
