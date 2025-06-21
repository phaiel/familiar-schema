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
#[doc = "A 2-level classification for emotional content based on a simplified valence-arousal model."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/EmotionalValence.v1.schema.json\","]
#[doc = "  \"title\": \"Emotional Valence Taxonomy\","]
#[doc = "  \"description\": \"A 2-level classification for emotional content based on a simplified valence-arousal model.\","]
#[doc = "  \"category\": \"taxonomy\","]
#[doc = "  \"levels\": ["]
#[doc = "    \"valence\","]
#[doc = "    \"arousal\""]
#[doc = "  ],"]
#[doc = "  \"nodes\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Emotions associated with pleasant feelings.\","]
#[doc = "      \"embedding_context\": \"Positive emotions like joy, happiness, contentment, delight, love, pride, gratitude, serenity, interest, hope, amusement, inspiration, awe.\","]
#[doc = "      \"id\": \"positive\","]
#[doc = "      \"label\": \"Positive\","]
#[doc = "      \"level\": \"valence\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Emotions associated with unpleasant feelings.\","]
#[doc = "      \"embedding_context\": \"Negative emotions like sadness, anger, fear, disgust, guilt, shame, anxiety, frustration, disappointment, contempt.\","]
#[doc = "      \"id\": \"negative\","]
#[doc = "      \"label\": \"Negative\","]
#[doc = "      \"level\": \"valence\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"A lack of strong emotional valence.\","]
#[doc = "      \"embedding_context\": \"Neutral emotional states. Calm, observant, objective, factual, unemotional, placid, tranquil, indifferent.\","]
#[doc = "      \"id\": \"neutral\","]
#[doc = "      \"label\": \"Neutral\","]
#[doc = "      \"level\": \"valence\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Emotions that are activating and high-energy.\","]
#[doc = "      \"embedding_context\": \"High-energy emotions. Activating, intense, exciting, surprising, astonishing, ecstatic, furious, terrified, panicked, stressed, alert.\","]
#[doc = "      \"id\": \"high_arousal\","]
#[doc = "      \"label\": \"High Arousal\","]
#[doc = "      \"level\": \"arousal\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Emotions that are deactivating and low-energy.\","]
#[doc = "      \"embedding_context\": \"Low-energy emotions. Deactivating, calm, relaxed, serene, content, bored, tired, sleepy, depressed, gloomy, lethargic.\","]
#[doc = "      \"id\": \"low_arousal\","]
#[doc = "      \"label\": \"Low Arousal\","]
#[doc = "      \"level\": \"arousal\""]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"schema_version\": \"1.0.0\","]
#[doc = "  \"source_file\": \"taxonomy/EmotionalValence.schema.json\","]
#[doc = "  \"taxonomy_id\": \"emotional_valence_v1\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EmotionalValenceTaxonomy(pub ::serde_json::Value);
impl ::std::ops::Deref for EmotionalValenceTaxonomy {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<EmotionalValenceTaxonomy> for ::serde_json::Value {
    fn from(value: EmotionalValenceTaxonomy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EmotionalValenceTaxonomy> for EmotionalValenceTaxonomy {
    fn from(value: &EmotionalValenceTaxonomy) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for EmotionalValenceTaxonomy {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
