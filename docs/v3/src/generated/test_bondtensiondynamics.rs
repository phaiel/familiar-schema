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
#[doc = "`BasePhysicsLawExecutionEnvelope`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BasePhysicsLawExecutionEnvelope\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"priority\","]
#[doc = "    \"trigger\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"priority\": {"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 10.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"timeout_ms\": {"]
#[doc = "      \"default\": 500,"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"trigger\": {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"event\","]
#[doc = "        \"user_request\","]
#[doc = "        \"observation\","]
#[doc = "        \"system_schedule\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BasePhysicsLawExecutionEnvelope {
    pub priority: ::std::num::NonZeroU64,
    #[serde(default = "defaults::default_u64::<i64, 500>")]
    pub timeout_ms: i64,
    pub trigger: BasePhysicsLawExecutionEnvelopeTrigger,
}
impl ::std::convert::From<&BasePhysicsLawExecutionEnvelope> for BasePhysicsLawExecutionEnvelope {
    fn from(value: &BasePhysicsLawExecutionEnvelope) -> Self {
        value.clone()
    }
}
impl BasePhysicsLawExecutionEnvelope {
    pub fn builder() -> builder::BasePhysicsLawExecutionEnvelope {
        Default::default()
    }
}
#[doc = "`BasePhysicsLawExecutionEnvelopeTrigger`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"event\","]
#[doc = "    \"user_request\","]
#[doc = "    \"observation\","]
#[doc = "    \"system_schedule\""]
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
pub enum BasePhysicsLawExecutionEnvelopeTrigger {
    #[serde(rename = "event")]
    Event,
    #[serde(rename = "user_request")]
    UserRequest,
    #[serde(rename = "observation")]
    Observation,
    #[serde(rename = "system_schedule")]
    SystemSchedule,
}
impl ::std::convert::From<&Self> for BasePhysicsLawExecutionEnvelopeTrigger {
    fn from(value: &BasePhysicsLawExecutionEnvelopeTrigger) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BasePhysicsLawExecutionEnvelopeTrigger {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Event => write!(f, "event"),
            Self::UserRequest => write!(f, "user_request"),
            Self::Observation => write!(f, "observation"),
            Self::SystemSchedule => write!(f, "system_schedule"),
        }
    }
}
impl ::std::str::FromStr for BasePhysicsLawExecutionEnvelopeTrigger {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "event" => Ok(Self::Event),
            "user_request" => Ok(Self::UserRequest),
            "observation" => Ok(Self::Observation),
            "system_schedule" => Ok(Self::SystemSchedule),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BasePhysicsLawExecutionEnvelopeTrigger {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BasePhysicsLawExecutionEnvelopeTrigger {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BasePhysicsLawExecutionEnvelopeTrigger {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Defines the common physics-related properties for components and laws."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "  \"title\": \"Base Physics Properties\","]
#[doc = "  \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"engine\","]
#[doc = "    \"is_quantum\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"collapse_sensitive\": {"]
#[doc = "      \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"engine\": {"]
#[doc = "      \"description\": \"The physics engine responsible for this object.\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"quantum\","]
#[doc = "        \"classical\","]
#[doc = "        \"hybrid\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"is_quantum\": {"]
#[doc = "      \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BasePhysicsProperties {
    #[doc = "Indicates if the object is affected by or triggers quantum collapse."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub collapse_sensitive: ::std::option::Option<bool>,
    #[doc = "The physics engine responsible for this object."]
    pub engine: BasePhysicsPropertiesEngine,
    #[doc = "Indicates if the object has quantum properties."]
    pub is_quantum: bool,
}
impl ::std::convert::From<&BasePhysicsProperties> for BasePhysicsProperties {
    fn from(value: &BasePhysicsProperties) -> Self {
        value.clone()
    }
}
impl BasePhysicsProperties {
    pub fn builder() -> builder::BasePhysicsProperties {
        Default::default()
    }
}
#[doc = "The physics engine responsible for this object."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The physics engine responsible for this object.\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"quantum\","]
#[doc = "    \"classical\","]
#[doc = "    \"hybrid\""]
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
pub enum BasePhysicsPropertiesEngine {
    #[serde(rename = "quantum")]
    Quantum,
    #[serde(rename = "classical")]
    Classical,
    #[serde(rename = "hybrid")]
    Hybrid,
}
impl ::std::convert::From<&Self> for BasePhysicsPropertiesEngine {
    fn from(value: &BasePhysicsPropertiesEngine) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BasePhysicsPropertiesEngine {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Quantum => write!(f, "quantum"),
            Self::Classical => write!(f, "classical"),
            Self::Hybrid => write!(f, "hybrid"),
        }
    }
}
impl ::std::str::FromStr for BasePhysicsPropertiesEngine {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "quantum" => Ok(Self::Quantum),
            "classical" => Ok(Self::Classical),
            "hybrid" => Ok(Self::Hybrid),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BasePhysicsPropertiesEngine {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Applies classical spring-damper physics to a Bond, simulating the tension and resilience of a relationship based on the distance between the connected Threads."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/laws/BondTensionDynamics.v1.schema.json\","]
#[doc = "  \"title\": \"Bond Tension Dynamics Law\","]
#[doc = "  \"description\": \"Applies classical spring-damper physics to a Bond, simulating the tension and resilience of a relationship based on the distance between the connected Threads.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"affected_components\","]
#[doc = "    \"execution_envelope\","]
#[doc = "    \"physics_properties\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"affected_components\": {"]
#[doc = "      \"description\": \"A list of component schema IDs that this law reads from or writes to.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"execution_envelope\": {"]
#[doc = "      \"title\": \"BasePhysicsLawExecutionEnvelope\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"priority\","]
#[doc = "        \"trigger\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"priority\": {"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 10.0,"]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"timeout_ms\": {"]
#[doc = "          \"default\": 500,"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        \"trigger\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"event\","]
#[doc = "            \"user_request\","]
#[doc = "            \"observation\","]
#[doc = "            \"system_schedule\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"physics_properties\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/base/PhysicsProperties.v1.schema.json\","]
#[doc = "      \"title\": \"Base Physics Properties\","]
#[doc = "      \"description\": \"Defines the common physics-related properties for components and laws.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"engine\","]
#[doc = "        \"is_quantum\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"collapse_sensitive\": {"]
#[doc = "          \"description\": \"Indicates if the object is affected by or triggers quantum collapse.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"engine\": {"]
#[doc = "          \"description\": \"The physics engine responsible for this object.\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"quantum\","]
#[doc = "            \"classical\","]
#[doc = "            \"hybrid\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"is_quantum\": {"]
#[doc = "          \"description\": \"Indicates if the object has quantum properties.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"$schema\": \"https://json-schema.org/draft/2020-12/schema\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"affected_components\": ["]
#[doc = "    \"https://familiar.dev/schemas/components/BondTensionComponent.v1.schema.json\","]
#[doc = "    \"https://familiar.dev/schemas/components/MemoryManifoldPosition.v1.schema.json\""]
#[doc = "  ],"]
#[doc = "  \"category\": \"laws\","]
#[doc = "  \"execution_envelope\": {"]
#[doc = "    \"priority\": 5,"]
#[doc = "    \"timeout_ms\": 50,"]
#[doc = "    \"trigger\": \"system_schedule\""]
#[doc = "  },"]
#[doc = "  \"physics_properties\": {"]
#[doc = "    \"collapse_sensitive\": false,"]
#[doc = "    \"engine\": \"classical\","]
#[doc = "    \"is_quantum\": false"]
#[doc = "  },"]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"laws/BondTensionDynamics.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondTensionDynamicsLaw {
    #[doc = "A list of component schema IDs that this law reads from or writes to."]
    pub affected_components: ::std::vec::Vec<::std::string::String>,
    pub execution_envelope: BasePhysicsLawExecutionEnvelope,
    pub physics_properties: BasePhysicsProperties,
}
impl ::std::convert::From<&BondTensionDynamicsLaw> for BondTensionDynamicsLaw {
    fn from(value: &BondTensionDynamicsLaw) -> Self {
        value.clone()
    }
}
impl BondTensionDynamicsLaw {
    pub fn builder() -> builder::BondTensionDynamicsLaw {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct BasePhysicsLawExecutionEnvelope {
        priority: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        timeout_ms: ::std::result::Result<i64, ::std::string::String>,
        trigger: ::std::result::Result<
            super::BasePhysicsLawExecutionEnvelopeTrigger,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BasePhysicsLawExecutionEnvelope {
        fn default() -> Self {
            Self {
                priority: Err("no value supplied for priority".to_string()),
                timeout_ms: Ok(super::defaults::default_u64::<i64, 500>()),
                trigger: Err("no value supplied for trigger".to_string()),
            }
        }
    }
    impl BasePhysicsLawExecutionEnvelope {
        pub fn priority<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.priority = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for priority: {}", e));
            self
        }
        pub fn timeout_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.timeout_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timeout_ms: {}", e));
            self
        }
        pub fn trigger<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsLawExecutionEnvelopeTrigger>,
            T::Error: ::std::fmt::Display,
        {
            self.trigger = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trigger: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BasePhysicsLawExecutionEnvelope>
        for super::BasePhysicsLawExecutionEnvelope
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BasePhysicsLawExecutionEnvelope,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                priority: value.priority?,
                timeout_ms: value.timeout_ms?,
                trigger: value.trigger?,
            })
        }
    }
    impl ::std::convert::From<super::BasePhysicsLawExecutionEnvelope>
        for BasePhysicsLawExecutionEnvelope
    {
        fn from(value: super::BasePhysicsLawExecutionEnvelope) -> Self {
            Self {
                priority: Ok(value.priority),
                timeout_ms: Ok(value.timeout_ms),
                trigger: Ok(value.trigger),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BasePhysicsProperties {
        collapse_sensitive:
            ::std::result::Result<::std::option::Option<bool>, ::std::string::String>,
        engine: ::std::result::Result<super::BasePhysicsPropertiesEngine, ::std::string::String>,
        is_quantum: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for BasePhysicsProperties {
        fn default() -> Self {
            Self {
                collapse_sensitive: Ok(Default::default()),
                engine: Err("no value supplied for engine".to_string()),
                is_quantum: Err("no value supplied for is_quantum".to_string()),
            }
        }
    }
    impl BasePhysicsProperties {
        pub fn collapse_sensitive<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.collapse_sensitive = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for collapse_sensitive: {}",
                    e
                )
            });
            self
        }
        pub fn engine<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsPropertiesEngine>,
            T::Error: ::std::fmt::Display,
        {
            self.engine = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for engine: {}", e));
            self
        }
        pub fn is_quantum<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_quantum = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_quantum: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BasePhysicsProperties> for super::BasePhysicsProperties {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BasePhysicsProperties,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                collapse_sensitive: value.collapse_sensitive?,
                engine: value.engine?,
                is_quantum: value.is_quantum?,
            })
        }
    }
    impl ::std::convert::From<super::BasePhysicsProperties> for BasePhysicsProperties {
        fn from(value: super::BasePhysicsProperties) -> Self {
            Self {
                collapse_sensitive: Ok(value.collapse_sensitive),
                engine: Ok(value.engine),
                is_quantum: Ok(value.is_quantum),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondTensionDynamicsLaw {
        affected_components:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        execution_envelope:
            ::std::result::Result<super::BasePhysicsLawExecutionEnvelope, ::std::string::String>,
        physics_properties:
            ::std::result::Result<super::BasePhysicsProperties, ::std::string::String>,
    }
    impl ::std::default::Default for BondTensionDynamicsLaw {
        fn default() -> Self {
            Self {
                affected_components: Err("no value supplied for affected_components".to_string()),
                execution_envelope: Err("no value supplied for execution_envelope".to_string()),
                physics_properties: Err("no value supplied for physics_properties".to_string()),
            }
        }
    }
    impl BondTensionDynamicsLaw {
        pub fn affected_components<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.affected_components = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for affected_components: {}",
                    e
                )
            });
            self
        }
        pub fn execution_envelope<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsLawExecutionEnvelope>,
            T::Error: ::std::fmt::Display,
        {
            self.execution_envelope = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for execution_envelope: {}",
                    e
                )
            });
            self
        }
        pub fn physics_properties<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BasePhysicsProperties>,
            T::Error: ::std::fmt::Display,
        {
            self.physics_properties = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for physics_properties: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<BondTensionDynamicsLaw> for super::BondTensionDynamicsLaw {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondTensionDynamicsLaw,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                affected_components: value.affected_components?,
                execution_envelope: value.execution_envelope?,
                physics_properties: value.physics_properties?,
            })
        }
    }
    impl ::std::convert::From<super::BondTensionDynamicsLaw> for BondTensionDynamicsLaw {
        fn from(value: super::BondTensionDynamicsLaw) -> Self {
            Self {
                affected_components: Ok(value.affected_components),
                execution_envelope: Ok(value.execution_envelope),
                physics_properties: Ok(value.physics_properties),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_u64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<u64>,
        <T as ::std::convert::TryFrom<u64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(V).unwrap()
    }
}
