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
#[doc = "A canonical enum of the machine-readable reasons for a Thread state change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/ThreadStateReason.v1.json\","]
#[doc = "  \"title\": \"Thread State Reason\","]
#[doc = "  \"description\": \"A canonical enum of the machine-readable reasons for a Thread state change.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserMarkedInactive\","]
#[doc = "    \"UserMarkedDeceased\","]
#[doc = "    \"SystemDetectedInactivity\","]
#[doc = "    \"LifecycleCompleted\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/types/lifecycles/ThreadStateReason.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ThreadStateReason {
    UserMarkedInactive,
    UserMarkedDeceased,
    SystemDetectedInactivity,
    LifecycleCompleted,
}
impl ::std::convert::From<&Self> for ThreadStateReason {
    fn from(value: &ThreadStateReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ThreadStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserMarkedInactive => write!(f, "UserMarkedInactive"),
            Self::UserMarkedDeceased => write!(f, "UserMarkedDeceased"),
            Self::SystemDetectedInactivity => write!(f, "SystemDetectedInactivity"),
            Self::LifecycleCompleted => write!(f, "LifecycleCompleted"),
        }
    }
}
impl ::std::str::FromStr for ThreadStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UserMarkedInactive" => Ok(Self::UserMarkedInactive),
            "UserMarkedDeceased" => Ok(Self::UserMarkedDeceased),
            "SystemDetectedInactivity" => Ok(Self::SystemDetectedInactivity),
            "LifecycleCompleted" => Ok(Self::LifecycleCompleted),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ThreadStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
