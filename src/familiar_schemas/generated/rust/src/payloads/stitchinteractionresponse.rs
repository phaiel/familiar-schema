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
#[doc = "Schema for the Redpanda event carrying the user's response back to the system to resolve a cognitive dissonance."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/StitchInteractionResponse.schema.json\","]
#[doc = "  \"title\": \"StitchInteractionResponse\","]
#[doc = "  \"description\": \"Schema for the Redpanda event carrying the user's response back to the system to resolve a cognitive dissonance.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"interaction_id\","]
#[doc = "    \"response_payload\","]
#[doc = "    \"stitch_id\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"user_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"interaction_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"response_payload\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"Choice\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"Choice\": {"]
#[doc = "              \"description\": \"The ID of the chosen option from a MultipleChoice request.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"Confirmed\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"Confirmed\": {"]
#[doc = "              \"description\": \"The boolean result of a Confirmation request.\","]
#[doc = "              \"type\": \"boolean\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"Text\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"Text\": {"]
#[doc = "              \"description\": \"The text provided in a FreeText request.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"stitch_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"user_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"payloads\","]
#[doc = "  \"source_file\": \"payloads/StitchInteractionResponse.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchInteractionResponse {
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub interaction_id: ::std::string::String,
    pub response_payload: StitchInteractionResponseResponsePayload,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub stitch_id: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub user_id: ::std::string::String,
}
impl ::std::convert::From<&StitchInteractionResponse> for StitchInteractionResponse {
    fn from(value: &StitchInteractionResponse) -> Self {
        value.clone()
    }
}
impl StitchInteractionResponse {
    pub fn builder() -> builder::StitchInteractionResponse {
        Default::default()
    }
}
#[doc = "`StitchInteractionResponseResponsePayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Choice\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Choice\": {"]
#[doc = "          \"description\": \"The ID of the chosen option from a MultipleChoice request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Confirmed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Confirmed\": {"]
#[doc = "          \"description\": \"The boolean result of a Confirmation request.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Text\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Text\": {"]
#[doc = "          \"description\": \"The text provided in a FreeText request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum StitchInteractionResponseResponsePayload {
    Choice(::std::string::String),
    Confirmed(bool),
    Text(::std::string::String),
}
impl ::std::convert::From<&Self> for StitchInteractionResponseResponsePayload {
    fn from(value: &StitchInteractionResponseResponsePayload) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for StitchInteractionResponseResponsePayload {
    fn from(value: bool) -> Self {
        Self::Confirmed(value)
    }
}
#[doc = "`StitchResponsePayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Choice\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Choice\": {"]
#[doc = "          \"description\": \"The ID of the chosen option from a MultipleChoice request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Confirmed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Confirmed\": {"]
#[doc = "          \"description\": \"The boolean result of a Confirmation request.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"Text\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"Text\": {"]
#[doc = "          \"description\": \"The text provided in a FreeText request.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub enum StitchResponsePayload {
    Choice(::std::string::String),
    Confirmed(bool),
    Text(::std::string::String),
}
impl ::std::convert::From<&Self> for StitchResponsePayload {
    fn from(value: &StitchResponsePayload) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for StitchResponsePayload {
    fn from(value: bool) -> Self {
        Self::Confirmed(value)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct StitchInteractionResponse {
        interaction_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        response_payload: ::std::result::Result<
            super::StitchInteractionResponseResponsePayload,
            ::std::string::String,
        >,
        stitch_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
        user_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StitchInteractionResponse {
        fn default() -> Self {
            Self {
                interaction_id: Err("no value supplied for interaction_id".to_string()),
                response_payload: Err("no value supplied for response_payload".to_string()),
                stitch_id: Err("no value supplied for stitch_id".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                user_id: Err("no value supplied for user_id".to_string()),
            }
        }
    }
    impl StitchInteractionResponse {
        pub fn interaction_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.interaction_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for interaction_id: {}", e));
            self
        }
        pub fn response_payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchInteractionResponseResponsePayload>,
            T::Error: ::std::fmt::Display,
        {
            self.response_payload = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for response_payload: {}",
                    e
                )
            });
            self
        }
        pub fn stitch_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.stitch_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for stitch_id: {}", e));
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
        pub fn user_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.user_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for user_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchInteractionResponse> for super::StitchInteractionResponse {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchInteractionResponse,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                interaction_id: value.interaction_id?,
                response_payload: value.response_payload?,
                stitch_id: value.stitch_id?,
                timestamp: value.timestamp?,
                user_id: value.user_id?,
            })
        }
    }
    impl ::std::convert::From<super::StitchInteractionResponse> for StitchInteractionResponse {
        fn from(value: super::StitchInteractionResponse) -> Self {
            Self {
                interaction_id: Ok(value.interaction_id),
                response_payload: Ok(value.response_payload),
                stitch_id: Ok(value.stitch_id),
                timestamp: Ok(value.timestamp),
                user_id: Ok(value.user_id),
            }
        }
    }
}
