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
#[doc = "A canonical enum of all possible lifecycle states for a Bond entity."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondState.v1.json\","]
#[doc = "  \"title\": \"Bond State\","]
#[doc = "  \"description\": \"A canonical enum of all possible lifecycle states for a Bond entity.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Active\","]
#[doc = "    \"Strained\","]
#[doc = "    \"Dissolved\","]
#[doc = "    \"Rekindled\""]
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
pub enum BondState {
    Active,
    Strained,
    Dissolved,
    Rekindled,
}
impl ::std::convert::From<&Self> for BondState {
    fn from(value: &BondState) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BondState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Active => write!(f, "Active"),
            Self::Strained => write!(f, "Strained"),
            Self::Dissolved => write!(f, "Dissolved"),
            Self::Rekindled => write!(f, "Rekindled"),
        }
    }
}
impl ::std::str::FromStr for BondState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "Active" => Ok(Self::Active),
            "Strained" => Ok(Self::Strained),
            "Dissolved" => Ok(Self::Dissolved),
            "Rekindled" => Ok(Self::Rekindled),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BondState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BondState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BondState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "An event published to request a change in a Bond's lifecycle state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/events/BondStateChangeRequested.v1.schema.json\","]
#[doc = "  \"title\": \"Bond State Change Requested Event\","]
#[doc = "  \"description\": \"An event published to request a change in a Bond's lifecycle state.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event_id\","]
#[doc = "    \"payload\","]
#[doc = "    \"source_service\","]
#[doc = "    \"timestamp\","]
#[doc = "    \"trace_id\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"event_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"payload\": {"]
#[doc = "      \"title\": \"BondStateChangeRequestedPayload\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"bond_id\","]
#[doc = "        \"new_state\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"bond_id\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "          \"title\": \"Entity ID Field\","]
#[doc = "          \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"new_state\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondState.v1.json\","]
#[doc = "          \"title\": \"Bond State\","]
#[doc = "          \"description\": \"A canonical enum of all possible lifecycle states for a Bond entity.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"Active\","]
#[doc = "            \"Strained\","]
#[doc = "            \"Dissolved\","]
#[doc = "            \"Rekindled\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"reason\": {"]
#[doc = "          \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondStateReason.v1.json\","]
#[doc = "          \"title\": \"Bond State Reason\","]
#[doc = "          \"description\": \"A canonical enum of the machine-readable reasons for a Bond state change.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"UserInitiated\","]
#[doc = "            \"SystemDetectedStrain\","]
#[doc = "            \"PartnerAccountErased\","]
#[doc = "            \"RelationshipEnded\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"source_service\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"timestamp\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/primitives/Timestamp.v1.json\","]
#[doc = "      \"title\": \"Timestamp\","]
#[doc = "      \"description\": \"A canonical definition for an ISO 8601 timestamp with timezone.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"trace_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"category\": \"events\","]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"events/BondStateChangeRequested.event.schema.json\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondStateChangeRequestedEvent {
    #[doc = "A reusable definition for a unique entity identifier."]
    pub event_id: ::std::string::String,
    pub payload: BondStateChangeRequestedPayload,
    pub source_service: ::std::string::String,
    #[doc = "A canonical definition for an ISO 8601 timestamp with timezone."]
    pub timestamp: ::std::string::String,
    #[doc = "A reusable definition for a unique entity identifier."]
    pub trace_id: ::std::string::String,
}
impl ::std::convert::From<&BondStateChangeRequestedEvent> for BondStateChangeRequestedEvent {
    fn from(value: &BondStateChangeRequestedEvent) -> Self {
        value.clone()
    }
}
impl BondStateChangeRequestedEvent {
    pub fn builder() -> builder::BondStateChangeRequestedEvent {
        Default::default()
    }
}
#[doc = "`BondStateChangeRequestedPayload`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"BondStateChangeRequestedPayload\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bond_id\","]
#[doc = "    \"new_state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bond_id\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/fields/EntityId.v1.json\","]
#[doc = "      \"title\": \"Entity ID Field\","]
#[doc = "      \"description\": \"A reusable definition for a unique entity identifier.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"new_state\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondState.v1.json\","]
#[doc = "      \"title\": \"Bond State\","]
#[doc = "      \"description\": \"A canonical enum of all possible lifecycle states for a Bond entity.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"Active\","]
#[doc = "        \"Strained\","]
#[doc = "        \"Dissolved\","]
#[doc = "        \"Rekindled\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"reason\": {"]
#[doc = "      \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondStateReason.v1.json\","]
#[doc = "      \"title\": \"Bond State Reason\","]
#[doc = "      \"description\": \"A canonical enum of the machine-readable reasons for a Bond state change.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"UserInitiated\","]
#[doc = "        \"SystemDetectedStrain\","]
#[doc = "        \"PartnerAccountErased\","]
#[doc = "        \"RelationshipEnded\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct BondStateChangeRequestedPayload {
    #[doc = "A reusable definition for a unique entity identifier."]
    pub bond_id: ::std::string::String,
    #[doc = "A canonical enum of all possible lifecycle states for a Bond entity."]
    pub new_state: BondState,
    #[doc = "A canonical enum of the machine-readable reasons for a Bond state change."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub reason: ::std::option::Option<BondStateReason>,
}
impl ::std::convert::From<&BondStateChangeRequestedPayload> for BondStateChangeRequestedPayload {
    fn from(value: &BondStateChangeRequestedPayload) -> Self {
        value.clone()
    }
}
impl BondStateChangeRequestedPayload {
    pub fn builder() -> builder::BondStateChangeRequestedPayload {
        Default::default()
    }
}
#[doc = "A canonical enum of the machine-readable reasons for a Bond state change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/types/lifecycles/BondStateReason.v1.json\","]
#[doc = "  \"title\": \"Bond State Reason\","]
#[doc = "  \"description\": \"A canonical enum of the machine-readable reasons for a Bond state change.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"UserInitiated\","]
#[doc = "    \"SystemDetectedStrain\","]
#[doc = "    \"PartnerAccountErased\","]
#[doc = "    \"RelationshipEnded\""]
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
pub enum BondStateReason {
    UserInitiated,
    SystemDetectedStrain,
    PartnerAccountErased,
    RelationshipEnded,
}
impl ::std::convert::From<&Self> for BondStateReason {
    fn from(value: &BondStateReason) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for BondStateReason {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::UserInitiated => write!(f, "UserInitiated"),
            Self::SystemDetectedStrain => write!(f, "SystemDetectedStrain"),
            Self::PartnerAccountErased => write!(f, "PartnerAccountErased"),
            Self::RelationshipEnded => write!(f, "RelationshipEnded"),
        }
    }
}
impl ::std::str::FromStr for BondStateReason {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "UserInitiated" => Ok(Self::UserInitiated),
            "SystemDetectedStrain" => Ok(Self::SystemDetectedStrain),
            "PartnerAccountErased" => Ok(Self::PartnerAccountErased),
            "RelationshipEnded" => Ok(Self::RelationshipEnded),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BondStateReason {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BondStateReason {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BondStateReason {
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
    pub struct BondStateChangeRequestedEvent {
        event_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        payload:
            ::std::result::Result<super::BondStateChangeRequestedPayload, ::std::string::String>,
        source_service: ::std::result::Result<::std::string::String, ::std::string::String>,
        timestamp: ::std::result::Result<::std::string::String, ::std::string::String>,
        trace_id: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for BondStateChangeRequestedEvent {
        fn default() -> Self {
            Self {
                event_id: Err("no value supplied for event_id".to_string()),
                payload: Err("no value supplied for payload".to_string()),
                source_service: Err("no value supplied for source_service".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                trace_id: Err("no value supplied for trace_id".to_string()),
            }
        }
    }
    impl BondStateChangeRequestedEvent {
        pub fn event_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event_id: {}", e));
            self
        }
        pub fn payload<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BondStateChangeRequestedPayload>,
            T::Error: ::std::fmt::Display,
        {
            self.payload = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for payload: {}", e));
            self
        }
        pub fn source_service<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source_service = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source_service: {}", e));
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
        pub fn trace_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.trace_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for trace_id: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondStateChangeRequestedEvent>
        for super::BondStateChangeRequestedEvent
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondStateChangeRequestedEvent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event_id: value.event_id?,
                payload: value.payload?,
                source_service: value.source_service?,
                timestamp: value.timestamp?,
                trace_id: value.trace_id?,
            })
        }
    }
    impl ::std::convert::From<super::BondStateChangeRequestedEvent> for BondStateChangeRequestedEvent {
        fn from(value: super::BondStateChangeRequestedEvent) -> Self {
            Self {
                event_id: Ok(value.event_id),
                payload: Ok(value.payload),
                source_service: Ok(value.source_service),
                timestamp: Ok(value.timestamp),
                trace_id: Ok(value.trace_id),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BondStateChangeRequestedPayload {
        bond_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        new_state: ::std::result::Result<super::BondState, ::std::string::String>,
        reason: ::std::result::Result<
            ::std::option::Option<super::BondStateReason>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for BondStateChangeRequestedPayload {
        fn default() -> Self {
            Self {
                bond_id: Err("no value supplied for bond_id".to_string()),
                new_state: Err("no value supplied for new_state".to_string()),
                reason: Ok(Default::default()),
            }
        }
    }
    impl BondStateChangeRequestedPayload {
        pub fn bond_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.bond_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bond_id: {}", e));
            self
        }
        pub fn new_state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::BondState>,
            T::Error: ::std::fmt::Display,
        {
            self.new_state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for new_state: {}", e));
            self
        }
        pub fn reason<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BondStateReason>>,
            T::Error: ::std::fmt::Display,
        {
            self.reason = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for reason: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<BondStateChangeRequestedPayload>
        for super::BondStateChangeRequestedPayload
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: BondStateChangeRequestedPayload,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bond_id: value.bond_id?,
                new_state: value.new_state?,
                reason: value.reason?,
            })
        }
    }
    impl ::std::convert::From<super::BondStateChangeRequestedPayload>
        for BondStateChangeRequestedPayload
    {
        fn from(value: super::BondStateChangeRequestedPayload) -> Self {
            Self {
                bond_id: Ok(value.bond_id),
                new_state: Ok(value.new_state),
                reason: Ok(value.reason),
            }
        }
    }
}
