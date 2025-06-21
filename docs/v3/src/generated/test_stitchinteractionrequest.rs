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
#[doc = "The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/payloads/StitchInteractionRequest.schema.json\","]
#[doc = "  \"title\": \"StitchInteractionRequest\","]
#[doc = "  \"description\": \"The structured request for human input, embedded within a StitchEntity, used when the system requires user collaboration to resolve a cognitive dissonance.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"interaction_id\","]
#[doc = "    \"interaction_type\","]
#[doc = "    \"prompt_details\","]
#[doc = "    \"prompt_title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"agent_context\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"interaction_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/UUID.v1.json\","]
#[doc = "      \"title\": \"UUID\","]
#[doc = "      \"description\": \"A canonical definition for a Universally Unique Identifier (UUID).\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"interaction_type\": {"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"options\","]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"options\": {"]
#[doc = "              \"type\": \"array\","]
#[doc = "              \"items\": {"]
#[doc = "                \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"id\","]
#[doc = "                  \"label\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"description\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                    \"title\": \"Description Field\","]
#[doc = "                    \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                    \"default\": \"\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  },"]
#[doc = "                  \"label\": {"]
#[doc = "                    \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                    \"title\": \"Label Field\","]
#[doc = "                    \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                    \"type\": \"string\""]
#[doc = "                  }"]
#[doc = "                }"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"MultipleChoice\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"confirm_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"deny_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"Confirmation\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"object\","]
#[doc = "          \"required\": ["]
#[doc = "            \"type\""]
#[doc = "          ],"]
#[doc = "          \"properties\": {"]
#[doc = "            \"placeholder\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"submit_text\": {"]
#[doc = "              \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "              \"title\": \"Label Field\","]
#[doc = "              \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "              \"type\": \"string\""]
#[doc = "            },"]
#[doc = "            \"type\": {"]
#[doc = "              \"const\": \"FreeText\""]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"prompt_details\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"prompt_title\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "      \"title\": \"Label Field\","]
#[doc = "      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"payloads\","]
#[doc = "  \"source_file\": \"payloads/StitchInteractionRequest.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchInteractionRequest {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    #[serde(default)]
    pub agent_context: ::std::string::String,
    #[doc = "A canonical definition for a Universally Unique Identifier (UUID)."]
    pub interaction_id: ::std::string::String,
    pub interaction_type: StitchInteractionRequestInteractionType,
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    pub prompt_details: ::std::string::String,
    #[doc = "A canonical definition for a short, human-readable UI label or title."]
    pub prompt_title: ::std::string::String,
}
impl ::std::convert::From<&StitchInteractionRequest> for StitchInteractionRequest {
    fn from(value: &StitchInteractionRequest) -> Self {
        value.clone()
    }
}
impl StitchInteractionRequest {
    pub fn builder() -> builder::StitchInteractionRequest {
        Default::default()
    }
}
#[doc = "`StitchInteractionRequestInteractionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"options\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"options\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\","]
#[doc = "              \"label\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"description\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                \"title\": \"Description Field\","]
#[doc = "                \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                \"default\": \"\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"label\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                \"title\": \"Label Field\","]
#[doc = "                \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"MultipleChoice\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"confirm_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"deny_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"Confirmation\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"placeholder\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"submit_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"FreeText\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StitchInteractionRequestInteractionType {
    MultipleChoice {
        options: ::std::vec::Vec<StitchInteractionRequestOneofItemItems>,
    },
    Confirmation {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        confirm_text: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        deny_text: ::std::option::Option<::std::string::String>,
    },
    FreeText {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        placeholder: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        submit_text: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for StitchInteractionRequestInteractionType {
    fn from(value: &StitchInteractionRequestInteractionType) -> Self {
        value.clone()
    }
}
#[doc = "`StitchInteractionRequestOneofItemItems`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"label\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"description\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "      \"title\": \"Description Field\","]
#[doc = "      \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "      \"default\": \"\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"label\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "      \"title\": \"Label Field\","]
#[doc = "      \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct StitchInteractionRequestOneofItemItems {
    #[doc = "A canonical, reusable definition for a human-readable description field."]
    #[serde(default)]
    pub description: ::std::string::String,
    pub id: ::std::string::String,
    #[doc = "A canonical definition for a short, human-readable UI label or title."]
    pub label: ::std::string::String,
}
impl ::std::convert::From<&StitchInteractionRequestOneofItemItems>
    for StitchInteractionRequestOneofItemItems
{
    fn from(value: &StitchInteractionRequestOneofItemItems) -> Self {
        value.clone()
    }
}
impl StitchInteractionRequestOneofItemItems {
    pub fn builder() -> builder::StitchInteractionRequestOneofItemItems {
        Default::default()
    }
}
#[doc = "`StitchInteractionType`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"options\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"options\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"title\": \"StitchInteractionRequestOneofItemItems\","]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\","]
#[doc = "              \"label\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"description\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Description.v1.json\","]
#[doc = "                \"title\": \"Description Field\","]
#[doc = "                \"description\": \"A canonical, reusable definition for a human-readable description field.\","]
#[doc = "                \"default\": \"\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"id\": {"]
#[doc = "                \"type\": \"string\""]
#[doc = "              },"]
#[doc = "              \"label\": {"]
#[doc = "                \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "                \"title\": \"Label Field\","]
#[doc = "                \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "                \"type\": \"string\""]
#[doc = "              }"]
#[doc = "            }"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"MultipleChoice\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"confirm_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"deny_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"Confirmation\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"placeholder\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"submit_text\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/Label.v1.json\","]
#[doc = "          \"title\": \"Label Field\","]
#[doc = "          \"description\": \"A canonical definition for a short, human-readable UI label or title.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"const\": \"FreeText\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum StitchInteractionType {
    MultipleChoice {
        options: ::std::vec::Vec<StitchInteractionRequestOneofItemItems>,
    },
    Confirmation {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        confirm_text: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        deny_text: ::std::option::Option<::std::string::String>,
    },
    FreeText {
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        placeholder: ::std::option::Option<::std::string::String>,
        #[doc = "A canonical definition for a short, human-readable UI label or title."]
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        submit_text: ::std::option::Option<::std::string::String>,
    },
}
impl ::std::convert::From<&Self> for StitchInteractionType {
    fn from(value: &StitchInteractionType) -> Self {
        value.clone()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct StitchInteractionRequest {
        agent_context: ::std::result::Result<::std::string::String, ::std::string::String>,
        interaction_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        interaction_type: ::std::result::Result<
            super::StitchInteractionRequestInteractionType,
            ::std::string::String,
        >,
        prompt_details: ::std::result::Result<::std::string::String, ::std::string::String>,
        prompt_title: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StitchInteractionRequest {
        fn default() -> Self {
            Self {
                agent_context: Ok(Default::default()),
                interaction_id: Err("no value supplied for interaction_id".to_string()),
                interaction_type: Err("no value supplied for interaction_type".to_string()),
                prompt_details: Err("no value supplied for prompt_details".to_string()),
                prompt_title: Err("no value supplied for prompt_title".to_string()),
            }
        }
    }
    impl StitchInteractionRequest {
        pub fn agent_context<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.agent_context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for agent_context: {}", e));
            self
        }
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
        pub fn interaction_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::StitchInteractionRequestInteractionType>,
            T::Error: ::std::fmt::Display,
        {
            self.interaction_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for interaction_type: {}",
                    e
                )
            });
            self
        }
        pub fn prompt_details<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_details = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_details: {}", e));
            self
        }
        pub fn prompt_title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.prompt_title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for prompt_title: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchInteractionRequest> for super::StitchInteractionRequest {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchInteractionRequest,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                agent_context: value.agent_context?,
                interaction_id: value.interaction_id?,
                interaction_type: value.interaction_type?,
                prompt_details: value.prompt_details?,
                prompt_title: value.prompt_title?,
            })
        }
    }
    impl ::std::convert::From<super::StitchInteractionRequest> for StitchInteractionRequest {
        fn from(value: super::StitchInteractionRequest) -> Self {
            Self {
                agent_context: Ok(value.agent_context),
                interaction_id: Ok(value.interaction_id),
                interaction_type: Ok(value.interaction_type),
                prompt_details: Ok(value.prompt_details),
                prompt_title: Ok(value.prompt_title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StitchInteractionRequestOneofItemItems {
        description: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<::std::string::String, ::std::string::String>,
        label: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for StitchInteractionRequestOneofItemItems {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                id: Err("no value supplied for id".to_string()),
                label: Err("no value supplied for label".to_string()),
            }
        }
    }
    impl StitchInteractionRequestOneofItemItems {
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
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for label: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<StitchInteractionRequestOneofItemItems>
        for super::StitchInteractionRequestOneofItemItems
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: StitchInteractionRequestOneofItemItems,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                id: value.id?,
                label: value.label?,
            })
        }
    }
    impl ::std::convert::From<super::StitchInteractionRequestOneofItemItems>
        for StitchInteractionRequestOneofItemItems
    {
        fn from(value: super::StitchInteractionRequestOneofItemItems) -> Self {
            Self {
                description: Ok(value.description),
                id: Ok(value.id),
                label: Ok(value.label),
            }
        }
    }
}
