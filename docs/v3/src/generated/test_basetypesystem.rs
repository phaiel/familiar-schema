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
#[doc = "The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "  \"title\": \"Base Type System\","]
#[doc = "  \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\","]
#[doc = "        \"boolean\","]
#[doc = "        \"integer\","]
#[doc = "        \"number\","]
#[doc = "        \"f32\","]
#[doc = "        \"f64\","]
#[doc = "        \"i32\","]
#[doc = "        \"i64\","]
#[doc = "        \"u32\","]
#[doc = "        \"u64\","]
#[doc = "        \"uuid\","]
#[doc = "        \"date-time\","]
#[doc = "        \"duration\","]
#[doc = "        \"object\","]
#[doc = "        \"array\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"$ref\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"$ref\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseTypeSystem.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum BaseTypeSystem {
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "string")]
    String,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "boolean")]
    Boolean,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "integer")]
    Integer,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "number")]
    Number,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "f32")]
    F32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "f64")]
    F64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "i32")]
    I32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "i64")]
    I64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "u32")]
    U32,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "u64")]
    U64,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "uuid")]
    Uuid,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "date-time")]
    DateTime,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "duration")]
    Duration,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "object")]
    Object,
    #[doc = "A primitive type, represented as a string from a controlled vocabulary."]
    #[serde(rename = "array")]
    Array,
    #[doc = "A complex, contrived meta-type, represented by a reference to its canonical schema file."]
    #[serde(rename = "$ref")]
    Ref(::std::string::String),
}
impl ::std::convert::From<&Self> for BaseTypeSystem {
    fn from(value: &BaseTypeSystem) -> Self {
        value.clone()
    }
}
