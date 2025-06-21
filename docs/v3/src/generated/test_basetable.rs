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
#[doc = "The base schema for defining a database table."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/Table.v1.schema.json\","]
#[doc = "  \"title\": \"Base Database Table\","]
#[doc = "  \"description\": \"The base schema for defining a database table.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"columns\","]
#[doc = "    \"description\","]
#[doc = "    \"schema_version\","]
#[doc = "    \"tableName\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"columns\": {"]
#[doc = "      \"title\": \"BaseDatabaseColumns\","]
#[doc = "      \"description\": \"A map of column names to their definitions.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$id\": \"https://familiar.dev/schemas/types/database/ColumnDefinition.v1.json\","]
#[doc = "        \"title\": \"Column Definition\","]
#[doc = "        \"description\": \"Schema for defining a database table column.\","]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"name\","]
#[doc = "          \"type\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"name\": {"]
#[doc = "            \"title\": \"Name Field\","]
#[doc = "            \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"nullable\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"primary_key\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          },"]
#[doc = "          \"type\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"enum\": ["]
#[doc = "              \"string\","]
#[doc = "              \"integer\","]
#[doc = "              \"boolean\","]
#[doc = "              \"timestamp\","]
#[doc = "              \"uuid\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"unique\": {"]
#[doc = "            \"default\": false,"]
#[doc = "            \"type\": \"boolean\""]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"description\": {"]
#[doc = "      \"description\": \"A clear, complete sentence explaining the object's purpose and function within the system.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"schema_version\": {"]
#[doc = "      \"description\": \"The semantic version of this schema definition (e.g., '1.0.0').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"tableName\": {"]
#[doc = "      \"description\": \"The physical name of the table in the database.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState').\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"_base\","]
#[doc = "  \"source_file\": \"_base/BaseTable.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BaseDatabaseTable {
    #[doc = "A map of column names to their definitions."]
    pub columns: ::std::collections::HashMap<::std::string::String, ColumnDefinition>,
    #[doc = "A clear, complete sentence explaining the object's purpose and function within the system."]
    pub description: ::std::string::String,
    #[doc = "The semantic version of this schema definition (e.g., '1.0.0')."]
    pub schema_version: ::std::string::String,
    #[doc = "The physical name of the table in the database."]
    #[serde(rename = "tableName")]
    pub table_name: ::std::string::String,
    #[doc = "The human-readable, PascalCase name of the object (e.g., 'UniversalPhysicsState')."]
    pub title: ::std::string::String,
}
impl ::std::convert::From<&BaseDatabaseTable> for BaseDatabaseTable {
    fn from(value: &BaseDatabaseTable) -> Self {
        value.clone()
    }
}
impl BaseDatabaseTable {
    pub fn builder() -> builder::BaseDatabaseTable {
        Default::default()
    }
}
#[doc = "Schema for defining a database table column."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/database/ColumnDefinition.v1.json\","]
#[doc = "  \"title\": \"Column Definition\","]
#[doc = "  \"description\": \"Schema for defining a database table column.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"name\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"name\": {"]
#[doc = "      \"title\": \"Name Field\","]
#[doc = "      \"description\": \"The primary, human-readable name of an entity.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"nullable\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"primary_key\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"boolean\","]
#[doc = "        \"timestamp\","]
#[doc = "        \"uuid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"unique\": {"]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ColumnDefinition {
    #[doc = "The primary, human-readable name of an entity."]
    pub name: ::std::string::String,
    #[serde(default)]
    pub nullable: bool,
    #[serde(default)]
    pub primary_key: bool,
    #[serde(rename = "type")]
    pub type_: ColumnDefinitionType,
    #[serde(default)]
    pub unique: bool,
}
impl ::std::convert::From<&ColumnDefinition> for ColumnDefinition {
    fn from(value: &ColumnDefinition) -> Self {
        value.clone()
    }
}
impl ColumnDefinition {
    pub fn builder() -> builder::ColumnDefinition {
        Default::default()
    }
}
#[doc = "`ColumnDefinitionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"boolean\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"uuid\""]
#[doc = "  ]"]
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
pub enum ColumnDefinitionType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "timestamp")]
    Timestamp,
    #[serde(rename = "uuid")]
    Uuid,
}
impl ::std::convert::From<&Self> for ColumnDefinitionType {
    fn from(value: &ColumnDefinitionType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ColumnDefinitionType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::String => write!(f, "string"),
            Self::Integer => write!(f, "integer"),
            Self::Boolean => write!(f, "boolean"),
            Self::Timestamp => write!(f, "timestamp"),
            Self::Uuid => write!(f, "uuid"),
        }
    }
}
impl ::std::str::FromStr for ColumnDefinitionType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "string" => Ok(Self::String),
            "integer" => Ok(Self::Integer),
            "boolean" => Ok(Self::Boolean),
            "timestamp" => Ok(Self::Timestamp),
            "uuid" => Ok(Self::Uuid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ColumnDefinitionType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BaseDatabaseTable {
        columns: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::ColumnDefinition>,
            ::std::string::String,
        >,
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        schema_version: ::std::result::Result<::std::string::String, ::std::string::String>,
        table_name: ::std::result::Result<::std::string::String, ::std::string::String>,
        title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BaseDatabaseTable {
        fn default() -> Self {
            Self {
                columns: Err("no value supplied for columns".to_string()),
                description: Err("no value supplied for description".to_string()),
                schema_version: Err("no value supplied for schema_version".to_string()),
                table_name: Err("no value supplied for table_name".to_string()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl BaseDatabaseTable {
        pub fn columns<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, super::ColumnDefinition>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.columns = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for columns: {}", e));
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
        pub fn schema_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.schema_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema_version: {}", e));
            self
        }
        pub fn table_name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.table_name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for table_name: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BaseDatabaseTable> for super::BaseDatabaseTable {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BaseDatabaseTable,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                columns: value.columns?,
                description: value.description?,
                schema_version: value.schema_version?,
                table_name: value.table_name?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::BaseDatabaseTable> for BaseDatabaseTable {
        fn from(value: super::BaseDatabaseTable) -> Self {
            Self {
                columns: Ok(value.columns),
                description: Ok(value.description),
                schema_version: Ok(value.schema_version),
                table_name: Ok(value.table_name),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ColumnDefinition {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        nullable: ::std::result::Result<bool, ::std::string::String>,
        primary_key: ::std::result::Result<bool, ::std::string::String>,
        type_: ::std::result::Result<super::ColumnDefinitionType, ::std::string::String>,
        unique: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ColumnDefinition {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                nullable: Ok(Default::default()),
                primary_key: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
                unique: Ok(Default::default()),
            }
        }
    }
    impl ColumnDefinition {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn nullable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.nullable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for nullable: {}", e));
            self
        }
        pub fn primary_key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.primary_key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for primary_key: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ColumnDefinitionType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn unique<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.unique = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unique: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ColumnDefinition> for super::ColumnDefinition {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ColumnDefinition,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                nullable: value.nullable?,
                primary_key: value.primary_key?,
                type_: value.type_?,
                unique: value.unique?,
            })
        }
    }
    impl ::std::convert::From<super::ColumnDefinition> for ColumnDefinition {
        fn from(value: super::ColumnDefinition) -> Self {
            Self {
                name: Ok(value.name),
                nullable: Ok(value.nullable),
                primary_key: Ok(value.primary_key),
                type_: Ok(value.type_),
                unique: Ok(value.unique),
            }
        }
    }
}
