//! Represents each attribute that a character can have
mod traits;

mod to_attribute;
use itertools::chain;
pub use to_attribute::*;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
pub use traits::*;
use utils::enums::StaticOptions;

use crate::{
    bonus::{Bonus, BonusTemplate, CloneBonus},
    feat::Feat,
    types::{
        ability::Ability, absorption::Absorption, armor_class::ArmorClass, damage_type::DamageType,
        flag::Flag, heal_amp::HealingAmplification, health::Health, player_class::PlayerClass,
        saving_throw::SavingThrow, sheltering::Sheltering, skill::Skill, spell_points::SpellPoints,
        spell_power::SpellPower, spell_selector::SpellSelector,
        summoned_attribute::SummonedAttribute, toggle::Toggle, weapon_attribute::WeaponAttribute,
    },
};
use std::fmt::Display;

/// Describes various traits of a character, ranging from having feats, stats, and much more.
#[derive(Hash, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Attribute {
    /// Behaves as a debuggable attribute
    #[cfg(feature = "debug")]
    Debug(u8),
    /// Behaves as a dummy variable
    ///
    /// The use of `Dummy` is for the [`Compiler`], where a `Dummy` bonus can be added to remove
    /// all current [`Bonus`] entries for a given [`BonusSource`].
    ///
    /// [`Compiler`]: crate::compiler::Compiler
    /// [`BonusSource`]: crate::bonus::BonusSource
    Dummy,
    /// Indicates that the user has some flag
    Flag(Flag),
    /// Results from the user interacting with toggles / sliders.
    ///
    /// When a user toggles a toggle, or changes a slider, these attributes are updated so that
    /// associated bonuses can react.
    Toggle(Toggle),
    /// Does the user have the feat.
    Feat(Feat),
    /// The ability score of the character.
    Ability(Ability),
    /// The modifier, calculated from [`Attribute::Ability`].
    AbilityModifier(Ability),
    /// Indicates how many levels the character has of a given class.
    ClassLevel(PlayerClass),
    /// The different skills available in the game.
    Skill(Skill),
    /// Both simple and complex saving throws.
    SavingThrow(SavingThrow),
    /// Character Spell Power.
    ///
    /// For every spell power unit, the character gains `1%` more damage with spells of that given
    /// [`SpellPower`]. For example, having `102` spell power gives a `+102%` spell damage boost,
    /// which results in an overall damage scale of `202%`.
    SpellPower(SpellPower),
    /// The chance that the user has to critically hit with spells.
    SpellCriticalChance(SpellPower),
    /// The bonus to damage that the user has with critical hits on spells.
    SpellCriticalDamage(SpellPower),
    /// Bonuses to caster levels of certain spells.
    CasterLevel(SpellSelector),
    /// Bonsues to maximum caster level of certain spells.
    MaxCasterLevel(SpellSelector),
    /// Bonuses to the DCs of certain spells.
    SpellDC(SpellSelector),
    /// Bonuses to stats to either the main hand or off hand.
    Weapon(WeaponAttribute),
    /// Armor class values
    ArmorClass(ArmorClass),
    /// Physical or Magical Sheltering
    Sheltering(Sheltering),
    /// Damage reduced from energy sources
    Resistance(DamageType),
    /// % Damage reduced from energy sources
    Absorption(Absorption),
    /// Spell Resistance
    SpellResistance,
    /// Spell Penetration
    SpellPenetration,
    /// Health
    Health(Health),
    /// Spell Points
    SpellPoints(SpellPoints),
    /// Total Character Level
    TotalCharacterLevel,
    /// Summoned Creature Bonuses
    SummonedAttribute(SummonedAttribute),
    /// Armor Check Penalty
    ArmorCheckPenalty,
    /// Set Bonus
    SetBonus(String),
    /// Healing Amplification
    HealingAmplification(HealingAmplification),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "debug")]
            Self::Debug(val) => write!(f, "Debug {val}"),
            Self::Dummy => write!(f, "Dummy"),
            Self::Ability(ability) => write!(f, "{ability} Score"),
            Self::AbilityModifier(ability) => write!(f, "{ability} Modifier"),
            Self::Skill(skill) => skill.fmt(f),
            Self::Toggle(toggle) => write!(f, "Toggle: {toggle}"),
            Self::SpellPower(sp) => write!(f, "{sp} Spell Power"),
            Self::SpellCriticalChance(sp) => write!(f, "{sp} Spell Critical Chance"),
            Self::SpellCriticalDamage(sp) => write!(f, "{sp} Spell Critical Damage"),
            Self::SavingThrow(saving_throw) => write!(f, "{saving_throw} Saving Throw"),
            Self::CasterLevel(selector) => write!(f, "{selector} Caster Level"),
            Self::MaxCasterLevel(selector) => write!(f, "{selector} Max Caster Level"),
            Self::SpellDC(selector) => write!(f, "{selector} Spell DC"),
            Self::Weapon(weapon) => weapon.fmt(f),
            Self::ArmorClass(ac) => ac.fmt(f),
            Self::Sheltering(sheltering) => sheltering.fmt(f),
            Self::ClassLevel(cl) => write!(f, "{cl} Level"),
            Self::Flag(flag) => write!(f, "Flag: {flag}"),
            Self::Resistance(energy) => write!(f, "{energy} Resistance"),
            Self::Absorption(absorption) => absorption.fmt(f),
            Self::Feat(feat) => write!(f, "Feat: {feat}"),
            Self::SpellResistance => write!(f, "Spell Resistance"),
            Self::SpellPenetration => write!(f, "Spell Penetration"),
            Self::Health(health) => health.fmt(f),
            Self::SpellPoints(sp) => sp.fmt(f),
            Self::TotalCharacterLevel => write!(f, "Total Character Level"),
            Self::SummonedAttribute(attribute) => write!(f, "Summoned Creatures: {attribute}"),
            Self::ArmorCheckPenalty => write!(f, "Armor Check Penalty"),
            Self::SetBonus(set) => write!(f, "Set Bonus: {set}"),
            Self::HealingAmplification(heal_amp) => heal_amp.fmt(f),
        }
    }
}

impl Attribute {
    /// Gets any subsidary bonuses from a given attribute.
    ///
    /// This allows for bonuses like [`Attribute::Ability`] to automatically add bonuses to
    /// [`Attribute::AbilityModifier`] using some formula.
    ///
    /// If an attribute has no bonuses associated with it, then `None` is returned.
    #[must_use]
    pub fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        match self {
            Self::Toggle(toggle) => toggle.get_bonuses(value),
            Self::Weapon(stat) => stat.get_bonuses(value),
            Self::ClassLevel(cl) => cl.get_bonuses(value),
            Self::Flag(flag) => flag.get_bonuses(value),
            Self::Feat(feat) => feat.get_bonuses(value),
            Self::SummonedAttribute(attribute) => attribute.get_bonuses(value),
            _ => None,
        }
    }
}

impl CloneBonus for Attribute {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::Ability(ability) => ability.clone_bonus(bonus),
            Self::Skill(skill) => skill.clone_bonus(bonus),
            Self::Feat(feat) => feat.clone_bonus(bonus),
            Self::Sheltering(sheltering) => sheltering.clone_bonus(bonus),
            Self::SpellPower(sp)
            | Self::SpellCriticalChance(sp)
            | Self::SpellCriticalDamage(sp) => sp.clone_bonus(bonus),
            Self::SavingThrow(st) => st.clone_bonus(bonus),
            Self::Weapon(stat) => stat.clone_bonus(bonus),
            Self::SummonedAttribute(attribute) => attribute.clone_bonus(bonus),
            Self::HealingAmplification(heal_amp) => heal_amp.clone_bonus(bonus),
            _ => None,
        }
    }
}

impl StaticOptions for Attribute {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::Dummy,
                Self::SpellResistance,
                Self::SpellPenetration,
                Self::TotalCharacterLevel,
                Self::ArmorCheckPenalty
            ],
            Ability::get_static()
                .flat_map(|ability| [Self::Ability(ability), Self::AbilityModifier(ability)]),
            Skill::get_static().map(Self::Skill),
            SavingThrow::get_static().map(Self::SavingThrow),
            SpellPower::get_static().flat_map(|sp| {
                [
                    Self::SpellPower(sp),
                    Self::SpellCriticalChance(sp),
                    Self::SpellCriticalDamage(sp),
                ]
            }),
            Toggle::get_static().map(Self::Toggle),
            Flag::get_static().map(Self::Flag),
            Feat::get_static().map(Self::Feat),
            PlayerClass::get_static().map(Self::ClassLevel),
            SpellSelector::get_static().flat_map(|selector| {
                [
                    Self::CasterLevel(selector),
                    Self::MaxCasterLevel(selector),
                    Self::SpellDC(selector),
                ]
            }),
            WeaponAttribute::get_static().map(Self::Weapon),
            ArmorClass::get_static().map(Self::ArmorClass),
            Sheltering::get_static().map(Self::Sheltering),
            DamageType::get_static().map(Self::Resistance),
            Absorption::get_static().map(Self::Absorption),
            Health::get_static().map(Self::Health),
            SpellPoints::get_static().map(Self::SpellPoints),
            SummonedAttribute::get_static().map(Self::SummonedAttribute),
            HealingAmplification::get_static().map(Self::HealingAmplification),
        )
    }
}
