use crate::types::{ability::Ability, skill::Skill, spell_power::SpellPower};

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
pub enum Attribute {
    #[from(skip)]
    #[display("{_0} Score")]
    AbilityScore(Ability),
    #[from(skip)]
    #[display("{_0} Modifier")]
    AbilityModifier(Ability),
    #[display("{_0}")]
    Skill(Skill),
    #[display("Spell Power: {_0}")]
    SpellPower(SpellPower),
}
