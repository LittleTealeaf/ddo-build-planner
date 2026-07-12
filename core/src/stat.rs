use crate::{
    attribute::Attribute,
    types::{ability::Ability, skill::Skill, spell_power::SpellPower},
};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Display,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::From,
)]
pub enum Stat {
    #[display("Attribute: {_0}")]
    Attribute(Attribute),
    #[display("All Ability Scores")]
    AllAbilityScores,
    #[display("All Skills")]
    AllSkills,
    #[display("Potency")]
    Potency,
}

impl Stat {
    pub fn into_attributes(self) -> Vec<Attribute> {
        match self {
            Self::Attribute(attribute) => vec![attribute],
            Self::AllAbilityScores => Vec::from(Ability::VALUES.map(Attribute::AbilityScore)),
            Self::AllSkills => Vec::from(Skill::VALUES.map(Attribute::Skill)),
            Self::Potency => Vec::from(SpellPower::SPELL_POWERS.map(Attribute::SpellPower)),
        }
    }
}
