use itertools::chain;

use crate::{
    traits::IterValues,
    types::{
        ability::Ability, damage_type::DamageType, health::PlayerHealth, level::PlayerLevel, past_life::PastLife, player_race::PlayerRace, save::SavingThrow, sheltering::Sheltering, skill::Skill, spell_damage_type::SpellDamageType, spell_selector::SpellSelector, weapon_attribute::WeaponAttribute, weapon_slot::WeaponSlot
    },
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
    #[cfg(feature = "debug")]
    #[from(skip)]
    Debug(u32),
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
    #[display("{_0} Level")]
    Level(PlayerLevel),
    Health(PlayerHealth),
    IsRace(PlayerRace),
    PastLife(PastLife),
}

impl Attribute {
    #[must_use]
    pub const fn multiplicative(&self) -> bool {
        matches!(self, Self::Absorption(_))
    }
}

impl IterValues for Attribute {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            Ability::values().flat_map(|abil| [abil.modifier(), abil.score()]),
            Skill::values().map(Into::into),
            SpellDamageType::values().flat_map(|ty| [
                ty.spell_power(),
                ty.spell_critical_chance(),
                ty.spell_critical_damage()
            ]),
            SpellSelector::values().flat_map(|sel| { [sel.caster_level(), sel.spell_dc()] }),
            SavingThrow::values().map(Into::into),
            DamageType::values().map(Self::Absorption),
            Sheltering::values().map(Into::into),
            [
                Self::Doublestrike,
                Self::Doubleshot,
                Self::MeleePower,
                Self::RangedPower,
            ],
            WeaponAttribute::values().flat_map(|attr| {
                WeaponSlot::values().map(move |slot| Self::Weapon(attr, slot))
            }),
            PlayerLevel::values().map(Into::into),
            PlayerHealth::values().map(Into::into),
            PlayerRace::values().map(Into::into),
            PastLife::values().map(Into::into),
        )
    }
}
