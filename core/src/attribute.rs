use crate::types::{
    ability::Ability, damage_type::DamageType, save::SavingThrow, sheltering::Sheltering,
    skill::Skill, spell_damage_type::SpellDamageType, spell_selector::SpellSelector,
    weapon_attribute::WeaponAttribute, weapon_slot::WeaponSlot,
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
    #[from(skip)]
    SpellPower(SpellDamageType),
    #[display("Spell Critical Chance: {_0}")]
    #[from(skip)]
    SpellCriticalChance(SpellDamageType),
    #[display("Spell Critical Damage: {_0}")]
    #[from(skip)]
    SpellCriticalDamage(SpellDamageType),
    #[display("Caster Level: {_0}")]
    #[from(skip)]
    CasterLevel(SpellSelector),
    #[display("Spell DC: {_0}")]
    #[from(skip)]
    SpellDC(SpellSelector),
    #[display("{_0} Saving Throw")]
    SavingThrow(SavingThrow),
    #[display("Armor Check Penalty")]
    ArmorCheckPenalty,
    #[display("{_0} Absorption")]
    Absorption(DamageType),
    #[display("{_0}")]
    Sheltering(Sheltering),
    Doublestrike,
    Doubleshot,
    #[display("Melee Power")]
    MeleePower,
    #[display("Ranged Power")]
    RangedPower,
    #[display("{_0} {_1}")]
    Weapon(WeaponAttribute, WeaponSlot),
    #[display("Feat: {_0}")]
    Feat(String),
}

impl Attribute {
    #[must_use]
    pub const fn multiplicative(&self) -> bool {
        matches!(self, Self::Absorption(_))
    }
}
