use crate::types::{
    ability::Ability, damage_type::DamageType, save::SavingThrow, sheltering::Sheltering,
    skill::Skill, spell_power::SpellPower,
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
    #[display("{_0} Saving Throw")]
    SavingThrow(SavingThrow),
    ArmorCheckPenalty,
    Absorption(DamageType),
    Sheltering(Sheltering),
}

impl Attribute {
    #[must_use]
    pub const fn multiplicative(&self) -> bool {
        matches!(self, Self::Absorption(_))
    }
}
