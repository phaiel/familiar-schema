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
#[doc = "A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/physics/Vec6.v1.json\","]
#[doc = "  \"title\": \"6D Physics Vector\","]
#[doc = "  \"description\": \"A reusable data type for a vector of six f64 numbers, used for momentum and forces in the 6 spatial dimensions of the manifold.\","]
#[doc = "  \"type\": \"array\","]
#[doc = "  \"items\": {"]
#[doc = "    \"type\": \"number\""]
#[doc = "  },"]
#[doc = "  \"maxItems\": 6,"]
#[doc = "  \"minItems\": 6,"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/physics/Vec6.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct X6dPhysicsVector(pub [f64; 6usize]);
impl ::std::ops::Deref for X6dPhysicsVector {
    type Target = [f64; 6usize];
    fn deref(&self) -> &[f64; 6usize] {
        &self.0
    }
}
impl ::std::convert::From<X6dPhysicsVector> for [f64; 6usize] {
    fn from(value: X6dPhysicsVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&X6dPhysicsVector> for X6dPhysicsVector {
    fn from(value: &X6dPhysicsVector) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<[f64; 6usize]> for X6dPhysicsVector {
    fn from(value: [f64; 6usize]) -> Self {
        Self(value)
    }
}
