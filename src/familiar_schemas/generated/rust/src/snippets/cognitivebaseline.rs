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
#[doc = "Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/CognitiveBaseline.v1.schema.json\","]
#[doc = "  \"title\": \"CognitiveBaseline\","]
#[doc = "  \"description\": \"Defines the innate 'personality' or temperament of a Thread, modulating its physics interactions.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bond_damping_factor\","]
#[doc = "    \"consolidation_rate_modifier\","]
#[doc = "    \"emotional_volatility\","]
#[doc = "    \"exploration_bias\","]
#[doc = "    \"social_energy_factor\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bond_damping_factor\": {"]
#[doc = "      \"description\": \"How resistant their bonds are to change. (UI Label: Relationship Stability)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 2.0,"]
#[doc = "      \"minimum\": 0.5"]
#[doc = "    },"]
#[doc = "    \"consolidation_rate_modifier\": {"]
#[doc = "      \"description\": \"How quickly their memories become stable. (UI Label: Memory Consolidation)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.5,"]
#[doc = "      \"minimum\": 0.8"]
#[doc = "    },"]
#[doc = "    \"emotional_volatility\": {"]
#[doc = "      \"description\": \"How strongly emotional events affect this person. (UI Label: Emotional Reactivity)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 2.0,"]
#[doc = "      \"minimum\": 0.5"]
#[doc = "    },"]
#[doc = "    \"exploration_bias\": {"]
#[doc = "      \"description\": \"Tendency for creative, associative thinking. (UI Label: Openness to Experience)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.3,"]
#[doc = "      \"minimum\": 0.7"]
#[doc = "    },"]
#[doc = "    \"social_energy_factor\": {"]
#[doc = "      \"description\": \"How energized they are by social interactions. (UI Label: Social Energy)\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.2,"]
#[doc = "      \"minimum\": 0.8"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"snippets\","]
#[doc = "  \"source_file\": \"snippets/fields/CognitiveBaseline.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CognitiveBaseline {
    pub bond_damping_factor: f64,
    pub consolidation_rate_modifier: f64,
    pub emotional_volatility: f64,
    pub exploration_bias: f64,
    pub social_energy_factor: f64,
}
impl ::std::convert::From<&CognitiveBaseline> for CognitiveBaseline {
    fn from(value: &CognitiveBaseline) -> Self {
        value.clone()
    }
}
impl CognitiveBaseline {
    pub fn builder() -> builder::CognitiveBaseline {
        Default::default()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct CognitiveBaseline {
        bond_damping_factor: ::std::result::Result<f64, ::std::string::String>,
        consolidation_rate_modifier: ::std::result::Result<f64, ::std::string::String>,
        emotional_volatility: ::std::result::Result<f64, ::std::string::String>,
        exploration_bias: ::std::result::Result<f64, ::std::string::String>,
        social_energy_factor: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for CognitiveBaseline {
        fn default() -> Self {
            Self {
                bond_damping_factor: Err("no value supplied for bond_damping_factor".to_string()),
                consolidation_rate_modifier: Err(
                    "no value supplied for consolidation_rate_modifier".to_string(),
                ),
                emotional_volatility: Err("no value supplied for emotional_volatility".to_string()),
                exploration_bias: Err("no value supplied for exploration_bias".to_string()),
                social_energy_factor: Err("no value supplied for social_energy_factor".to_string()),
            }
        }
    }
    impl CognitiveBaseline {
        pub fn bond_damping_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bond_damping_factor = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for bond_damping_factor: {}",
                    e
                )
            });
            self
        }
        pub fn consolidation_rate_modifier<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.consolidation_rate_modifier = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for consolidation_rate_modifier: {}",
                    e
                )
            });
            self
        }
        pub fn emotional_volatility<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.emotional_volatility = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for emotional_volatility: {}",
                    e
                )
            });
            self
        }
        pub fn exploration_bias<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.exploration_bias = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for exploration_bias: {}",
                    e
                )
            });
            self
        }
        pub fn social_energy_factor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.social_energy_factor = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for social_energy_factor: {}",
                    e
                )
            });
            self
        }
    }
    impl ::std::convert::TryFrom<CognitiveBaseline> for super::CognitiveBaseline {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CognitiveBaseline,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bond_damping_factor: value.bond_damping_factor?,
                consolidation_rate_modifier: value.consolidation_rate_modifier?,
                emotional_volatility: value.emotional_volatility?,
                exploration_bias: value.exploration_bias?,
                social_energy_factor: value.social_energy_factor?,
            })
        }
    }
    impl ::std::convert::From<super::CognitiveBaseline> for CognitiveBaseline {
        fn from(value: super::CognitiveBaseline) -> Self {
            Self {
                bond_damping_factor: Ok(value.bond_damping_factor),
                consolidation_rate_modifier: Ok(value.consolidation_rate_modifier),
                emotional_volatility: Ok(value.emotional_volatility),
                exploration_bias: Ok(value.exploration_bias),
                social_energy_factor: Ok(value.social_energy_factor),
            }
        }
    }
}
