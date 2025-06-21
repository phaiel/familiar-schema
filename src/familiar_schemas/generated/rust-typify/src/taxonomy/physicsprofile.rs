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
#[doc = "The 4-level classification system for assigning a physics profile to a cognitive entity based on its semantic content."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://familiar.dev/schemas/PhysicsProfile.v1.schema.json\","]
#[doc = "  \"title\": \"Physics Profile Taxonomy\","]
#[doc = "  \"description\": \"The 4-level classification system for assigning a physics profile to a cognitive entity based on its semantic content.\","]
#[doc = "  \"category\": \"taxonomy\","]
#[doc = "  \"levels\": ["]
#[doc = "    \"cognitive_complexity\","]
#[doc = "    \"social_scope\","]
#[doc = "    \"temporal_frame\","]
#[doc = "    \"domain_context\""]
#[doc = "  ],"]
#[doc = "  \"nodes\": ["]
#[doc = "    {"]
#[doc = "      \"description\": \"Direct factual logging with minimal processing.\","]
#[doc = "      \"embedding_context\": \"Simple factual recording of events. Basic logging of what happened without analysis or interpretation. Direct observation and documentation. Straightforward data capture. Recording facts and activities like 'ate lunch' or 'slept for 3 hours'.\","]
#[doc = "      \"id\": \"simple\","]
#[doc = "      \"label\": \"Simple\","]
#[doc = "      \"level\": \"cognitive_complexity\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Pattern recognition and correlation analysis.\","]
#[doc = "      \"embedding_context\": \"Pattern recognition and correlation analysis. Noticing connections between events. Identifying trends and relationships. Moderate cognitive processing. Comparing experiences and finding similarities, like 'I always feel tired on Tuesdays' or 'the baby seems happier after a long nap'.\","]
#[doc = "      \"id\": \"moderate\","]
#[doc = "      \"label\": \"Moderate\","]
#[doc = "      \"level\": \"cognitive_complexity\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Self-reflection, introspection, and meta-cognition.\","]
#[doc = "      \"embedding_context\": \"Deep self-reflection and introspection. Meta-cognitive analysis and self-awareness. Complex emotional processing. Understanding personal motivations and behaviors. Analyzing thoughts about thoughts. Deep psychological insight and self-examination, like 'I wonder why I react that way' or 'I realized my anxiety is connected to work'.\","]
#[doc = "      \"id\": \"complex\","]
#[doc = "      \"label\": \"Complex\","]
#[doc = "      \"level\": \"cognitive_complexity\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Actions or states concerning a single person.\","]
#[doc = "      \"embedding_context\": \"An individual's personal experience, thought, or action. Something done alone or concerning only one person. Solitary activities, personal reflections, and self-contained states. I, me, my, myself, he, she, they (singular).\","]
#[doc = "      \"id\": \"individual\","]
#[doc = "      \"label\": \"Individual\","]
#[doc = "      \"level\": \"social_scope\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Interactions or relationships between two people.\","]
#[doc = "      \"embedding_context\": \"An interaction between two people. A dyad. A one-on-one conversation, activity, or relationship. We (two people), us, you and I, they (two people).\","]
#[doc = "      \"id\": \"dyadic\","]
#[doc = "      \"label\": \"Dyadic\","]
#[doc = "      \"level\": \"social_scope\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Activities or states involving a group of three or more people.\","]
#[doc = "      \"embedding_context\": \"A group activity or dynamic. Involving multiple people, a family, a team, or a social gathering. We (a group), they (a group), everyone, the family, the team.\","]
#[doc = "      \"id\": \"group\","]
#[doc = "      \"label\": \"Group\","]
#[doc = "      \"level\": \"social_scope\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Events that happened just now or in the very recent past (minutes to hours).\","]
#[doc = "      \"embedding_context\": \"An event that just occurred. Immediate actions, recent happenings, things that took place in the last few hours. Real-time, now, present moment, just a moment ago, this morning.\","]
#[doc = "      \"id\": \"immediate\","]
#[doc = "      \"label\": \"Immediate\","]
#[doc = "      \"level\": \"temporal_frame\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Events from the recent past, typically within the last few days to a week.\","]
#[doc = "      \"embedding_context\": \"Something that happened recently. In the past few days, this past week. Not immediate, but not long ago. Recent history. Yesterday, a few days ago.\","]
#[doc = "      \"id\": \"recent\","]
#[doc = "      \"label\": \"Recent\","]
#[doc = "      \"level\": \"temporal_frame\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Patterns or events that span a longer duration, from weeks to months or more.\","]
#[doc = "      \"embedding_context\": \"A long-term pattern or event. Something that has been happening for a while, over several weeks or months. An extended period of time. A long-running situation. Over the last year, this past month.\","]
#[doc = "      \"id\": \"extended\","]
#[doc = "      \"label\": \"Extended\","]
#[doc = "      \"level\": \"temporal_frame\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to sleep, naps, bedtime, and rest.\","]
#[doc = "      \"embedding_context\": \"Sleep, napping, bedtime routines, rest, circadian rhythms, waking up, feeling tired or rested. All aspects of sleeping.\","]
#[doc = "      \"id\": \"sleep\","]
#[doc = "      \"label\": \"Sleep\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"low_energy_classical\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to eating, meals, breastfeeding, bottles.\","]
#[doc = "      \"embedding_context\": \"Eating, meals, feeding, breastfeeding, bottle feeding, nutrition, food, hunger, being full. All aspects of nourishment.\","]
#[doc = "      \"id\": \"feeding\","]
#[doc = "      \"label\": \"Feeding\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"moderate_energy_classical\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to health, wellness, illness, and medical events.\","]
#[doc = "      \"embedding_context\": \"Health, wellness, sickness, illness, doctor visits, medication, symptoms, checkups, medical procedures. All aspects of physical health.\","]
#[doc = "      \"id\": \"health\","]
#[doc = "      \"label\": \"Health\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"high_energy_classical\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to social interactions, bonding, and conflicts.\","]
#[doc = "      \"embedding_context\": \"Social interaction, relationships, bonding, conflict, arguments, playing together, family time, friendship, love. All aspects of interpersonal connection.\","]
#[doc = "      \"id\": \"relationship\","]
#[doc = "      \"label\": \"Relationship\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"high_energy_quantum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to developmental milestones, learning, and achievements.\","]
#[doc = "      \"embedding_context\": \"Learning, development, milestones, achievements, practicing a skill, crawling, walking, talking, firsts. All aspects of personal growth.\","]
#[doc = "      \"id\": \"development\","]
#[doc = "      \"label\": \"Development\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"very_high_energy_quantum\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"description\": \"Related to daily routines, habits, and chores.\","]
#[doc = "      \"embedding_context\": \"Daily routines, habits, chores, schedules, repeated actions, automatic behaviors. All aspects of regular, predictable life.\","]
#[doc = "      \"id\": \"routine\","]
#[doc = "      \"label\": \"Routine\","]
#[doc = "      \"level\": \"domain_context\","]
#[doc = "      \"metadata\": {"]
#[doc = "        \"physics_profile_affinity\": \"low_energy_classical\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"schema_version\": \"1.1.0\","]
#[doc = "  \"source_file\": \"taxonomy/PhysicsProfile.schema.json\","]
#[doc = "  \"taxonomy_id\": \"physics_profile_v1\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct PhysicsProfileTaxonomy(pub ::serde_json::Value);
impl ::std::ops::Deref for PhysicsProfileTaxonomy {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<PhysicsProfileTaxonomy> for ::serde_json::Value {
    fn from(value: PhysicsProfileTaxonomy) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PhysicsProfileTaxonomy> for PhysicsProfileTaxonomy {
    fn from(value: &PhysicsProfileTaxonomy) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Value> for PhysicsProfileTaxonomy {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
