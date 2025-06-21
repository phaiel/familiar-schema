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
#[doc = "Represents a complex number with real and imaginary parts."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/physics/ComplexNumber.v1.json\","]
#[doc = "  \"title\": \"Complex Number\","]
#[doc = "  \"description\": \"Represents a complex number with real and imaginary parts.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"imaginary\","]
#[doc = "    \"real\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"imaginary\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"real\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/physics/ComplexNumber.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ComplexNumber {
    pub imaginary: f64,
    pub real: f64,
}
impl ::std::convert::From<&ComplexNumber> for ComplexNumber {
    fn from(value: &ComplexNumber) -> Self {
        value.clone()
    }
}
impl ComplexNumber {
    pub fn builder() -> builder::ComplexNumber {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct ComplexNumber {
        imaginary: ::std::result::Result<f64, ::std::string::String>,
        real: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for ComplexNumber {
        fn default() -> Self {
            Self {
                imaginary: Err("no value supplied for imaginary".to_string()),
                real: Err("no value supplied for real".to_string()),
            }
        }
    }
    impl ComplexNumber {
        pub fn imaginary<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.imaginary = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for imaginary: {}", e));
            self
        }
        pub fn real<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.real = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for real: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ComplexNumber> for super::ComplexNumber {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ComplexNumber,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                imaginary: value.imaginary?,
                real: value.real?,
            })
        }
    }
    impl ::std::convert::From<super::ComplexNumber> for ComplexNumber {
        fn from(value: super::ComplexNumber) -> Self {
            Self {
                imaginary: Ok(value.imaginary),
                real: Ok(value.real),
            }
        }
    }
}
