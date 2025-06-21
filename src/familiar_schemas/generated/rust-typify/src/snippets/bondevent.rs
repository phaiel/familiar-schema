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
#[doc = "Represents a significant event that impacted a bond's strength or state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/social/BondEvent.v1.json\","]
#[doc = "  \"title\": \"Bond Event Type\","]
#[doc = "  \"description\": \"Represents a significant event that impacted a bond's strength or state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"impact\","]
#[doc = "    \"moment_id\","]
#[doc = "    \"timestamp\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"impact\": {"]
#[doc = "      \"description\": \"How much this event affected the bond, from -1.0 (negative) to 1.0 (positive).\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": -1.0"]
#[doc = "    },"]
#[doc = "    \"moment_id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/social/BondEvent.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondEventType {
    pub impact: f64,
    pub moment_id: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
}
impl ::std::convert::From<&BondEventType> for BondEventType {
    fn from(value: &BondEventType) -> Self {
        value.clone()
    }
}
impl BondEventType {
    pub fn builder() -> builder::BondEventType {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BondEventType {
        impact: ::std::result::Result<f64, ::std::string::String>,
        moment_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BondEventType {
        fn default() -> Self {
            Self {
                impact: Err("no value supplied for impact".to_string()),
                moment_id: Err("no value supplied for moment_id".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
            }
        }
    }
    impl BondEventType {
        pub fn impact<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.impact = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for impact: {}", e));
            self
        }
        pub fn moment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.moment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for moment_id: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondEventType> for super::BondEventType {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondEventType,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                impact: value.impact?,
                moment_id: value.moment_id?,
                timestamp: value.timestamp?,
            })
        }
    }
    impl ::std::convert::From<super::BondEventType> for BondEventType {
        fn from(value: super::BondEventType) -> Self {
            Self {
                impact: Ok(value.impact),
                moment_id: Ok(value.moment_id),
                timestamp: Ok(value.timestamp),
            }
        }
    }
}
