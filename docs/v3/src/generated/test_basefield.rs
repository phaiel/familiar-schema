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
#[doc = "Defines the structure for a single, inline field definition within a component."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Field.v1.schema.json\","]
#[doc = "  \"title\": \"Base Field\","]
#[doc = "  \"description\": \"Defines the structure for a single, inline field definition within a component.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"description\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"constraints\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "      \"title\": \"Constraint Definition\","]
#[doc = "      \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"enum\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"maxLength\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"maximum\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"minLength\": {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"minimum\": {"]
#[doc = "          \"type\": \"number\""]
#[doc = "        },"]
#[doc = "        \"pattern\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"default_value\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/AnyValue.v1.json\","]
#[doc = "      \"title\": \"Any Value\","]
#[doc = "      \"description\": \"Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"number\","]
#[doc = "        \"integer\","]
#[doc = "        \"boolean\","]
#[doc = "        \"object\","]
#[doc = "        \"array\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/base/TypeSystem.v1.schema.json\","]
#[doc = "      \"title\": \"Base Type System\","]
#[doc = "      \"description\": \"The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema.\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"description\": \"A primitive type, represented as a string from a controlled vocabulary.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"string\","]
#[doc = "            \"boolean\","]
#[doc = "            \"integer\","]
#[doc = "            \"number\","]
#[doc = "            \"f32\","]
#[doc = "            \"f64\","]
#[doc = "            \"i32\","]
#[doc = "            \"i64\","]
#[doc = "            \"u32\","]
#[doc = "            \"u64\","]
#[doc = "            \"uuid\","]
#[doc = "            \"date-time\","]
#[doc = "            \"duration\","]
#[doc = "            \"object\","]
#[doc = "            \"array\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"description\": \"A complex, contrived meta-type, represented by a reference to its canonical schema file.\","]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"$ref\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"$ref\": {"]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          },"]
#[doc = "          \"additionalProperties\": false"]
#[doc = "        }"]
#[doc = "      ],"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    },"]
#[doc = "    \"ui_label\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseField.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseField {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub constraints: ::std::option::Option<ConstraintDefinition>,
    #[doc = "Represents any valid JSON value. Used for fields with dynamic or unknown types, like 'default' values."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub default_value: ::std::option::Option<::serde_json::Value>,
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub description: ::std::string::String,
    #[doc = "The canonical list of all valid primitive and complex data types used within the Familiar engine. A type can be a primitive string or a reference to a complex type schema."]
    #[serde(rename = "type")]
    pub type_: BaseTypeSystem,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ui_label: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&BaseField> for BaseField {
    fn from(value: &BaseField) -> Self {
        value.clone()
    }
}
impl BaseField {
    pub fn builder() -> builder::BaseField {
        Default::default()
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
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
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
#[doc = "A canonical definition for field validation constraints."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/validation/ConstraintDefinition.v1.json\","]
#[doc = "  \"title\": \"Constraint Definition\","]
#[doc = "  \"description\": \"A canonical definition for field validation constraints.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"enum\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"maxLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"maximum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"minLength\": {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"minimum\": {"]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConstraintDefinition {
    #[serde(
        rename = "enum",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub enum_: ::std::vec::Vec<::std::string::String>,
    #[serde(
        rename = "maxLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub max_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maximum: ::std::option::Option<f64>,
    #[serde(
        rename = "minLength",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub min_length: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub minimum: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub pattern: ::std::option::Option<::std::string::String>,
}
impl ::std::convert::From<&ConstraintDefinition> for ConstraintDefinition {
    fn from(value: &ConstraintDefinition) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for ConstraintDefinition {
    fn default() -> Self {
        Self {
            enum_: Default::default(),
            max_length: Default::default(),
            maximum: Default::default(),
            min_length: Default::default(),
            minimum: Default::default(),
            pattern: Default::default(),
        }
    }
}
impl ConstraintDefinition {
    pub fn builder() -> builder::ConstraintDefinition {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseField {
        constraints: ::std::result::Result<
            ::std::option::Option<super::ConstraintDefinition>,
            ::std::string::String,
        >,
        default_value: ::std::result::Result<
            ::std::option::Option<::serde_json::Value>,
            ::std::string::String,
        >,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        type_: ::std::result::Result<super::BaseTypeSystem, ::std::string::String>,
        ui_label: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BaseField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                default_value: Ok(Default::default()),
                description: Err("no value supplied for description".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                ui_label: Ok(Default::default()),
            }
        }
    }
    impl BaseField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::ConstraintDefinition>>,
            T::Error: ::std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn default_value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::serde_json::Value>>,
            T::Error: ::std::fmt::Display,
        {
            self.default_value = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for default_value: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BaseTypeSystem>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn ui_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ui_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ui_label: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseField> for super::BaseField {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseField,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                constraints: value.constraints?,
                default_value: value.default_value?,
                description: value.description?,
                type_: value.type_?,
                ui_label: value.ui_label?,
            })
        }
    }
    impl ::std::convert::From<super::BaseField> for BaseField {
        fn from(value: super::BaseField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                default_value: Ok(value.default_value),
                description: Ok(value.description),
                type_: Ok(value.type_),
                ui_label: Ok(value.ui_label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConstraintDefinition {
        enum_: ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        max_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        maximum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min_length: ::std::result::Result<::std::option::Option<i64>, ::std::string::String>,
        minimum: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        pattern: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ConstraintDefinition {
        fn default() -> Self {
            Self {
                enum_: Ok(Default::default()),
                max_length: Ok(Default::default()),
                maximum: Ok(Default::default()),
                min_length: Ok(Default::default()),
                minimum: Ok(Default::default()),
                pattern: Ok(Default::default()),
            }
        }
    }
    impl ConstraintDefinition {
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {}", e));
            self
        }
        pub fn maximum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.maximum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for maximum: {}", e));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<i64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {}", e));
            self
        }
        pub fn minimum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.minimum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for minimum: {}", e));
            self
        }
        pub fn pattern<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pattern: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ConstraintDefinition> for super::ConstraintDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConstraintDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                enum_: value.enum_?,
                max_length: value.max_length?,
                maximum: value.maximum?,
                min_length: value.min_length?,
                minimum: value.minimum?,
                pattern: value.pattern?,
            })
        }
    }
    impl ::std::convert::From<super::ConstraintDefinition> for ConstraintDefinition {
        fn from(value: super::ConstraintDefinition) -> Self {
            Self {
                enum_: Ok(value.enum_),
                max_length: Ok(value.max_length),
                maximum: Ok(value.maximum),
                min_length: Ok(value.min_length),
                minimum: Ok(value.minimum),
                pattern: Ok(value.pattern),
            }
        }
    }
}
