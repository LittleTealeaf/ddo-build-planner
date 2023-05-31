//! Represents each attribute that a character can have
pub mod flags;
pub mod selectors;
pub mod toggles;
mod traits;
pub mod types;

pub use traits::*;

use crate::{
    bonus::{Bonus, CloneBonus},
    player_class::PlayerClass,
};
use enum_map::Enum;
use std::fmt::Display;

use self::{
    selectors::SpellSelector,
    toggles::Toggle,
    types::{
        Ability, ArmorClass, SavingThrow, Sheltering, Skill, SpellPower, WeaponHandStat,
        _AbilityModifier, _AbilityScore, _SpellCriticalChance, _SpellCriticalDamage, _SpellPower,
    },
};

/// Describes various traits of a character, ranging from having feats, stats, and much more.
#[derive(Copy, Clone, Enum, Eq, PartialEq, Debug)]
pub enum Attribute {
    /// Behaves as a dummy variable
    /// 
    /// The use of `Dummy` is for the [`Compiler`], where a `Dummy` bonus can be added to remove
    /// all current [`Bonus`] entries for a given [`BonusSource`].
    ///
    /// [`Compiler`]: crate::compiler::Compiler
    /// [`BonusSource`]: crate::bonus::BonusSource
    Dummy,
    /// Results from the user interacting with toggles / sliders.
    ///
    /// When a user toggles a toggle, or changes a slider, these attributes are updated so that
    /// associated bonuses can react.
    Toggle(Toggle),
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
    Weapon(WeaponHandStat),
    /// Armor class values
    ArmorClass(ArmorClass),
    /// Physical or Magical Sheltering
    Sheltering(Sheltering),
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Dummy => write!(f, "Dummy"),
            Attribute::Ability(ability) => write!(f, "{} Score", ability),
            Attribute::AbilityModifier(ability) => write!(f, "{} Modifier", ability),
            Attribute::Skill(skill) => skill.fmt(f),
            Attribute::Toggle(toggle) => toggle.fmt(f),
            Attribute::SpellPower(sp) => write!(f, "{} Spell Power", sp),
            Attribute::SpellCriticalChance(sp) => write!(f, "{} Spell Critical Chance", sp),
            Attribute::SpellCriticalDamage(sp) => write!(f, "{} Spell Critical Damage", sp),
            Attribute::SavingThrow(saving_throw) => write!(f, "{} Saving Throw", saving_throw),
            Attribute::CasterLevel(selector) => write!(f, "{} Caster Level", selector),
            Attribute::MaxCasterLevel(selector) => write!(f, "{} Max Caster Level", selector),
            Attribute::SpellDC(selector) => write!(f, "{} Spell DC", selector),
            Attribute::Weapon(weapon) => weapon.fmt(f),
            Attribute::ArmorClass(ac) => ac.fmt(f),
            Attribute::Sheltering(sheltering) => sheltering.fmt(f),
            Attribute::ClassLevel(cl) => write!(f, "{} Level", cl),
        }
    }
}

impl Attribute {
    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Attribute::AbilityModifier(ability) => {
                GetBonuses::<_AbilityModifier>::get_bonuses(ability, value)
            }
            Attribute::Ability(ability) => GetBonuses::<_AbilityScore>::get_bonuses(ability, value),
            Attribute::Skill(skill) => skill.get_bonuses(value),
            Attribute::Toggle(toggle) => toggle.get_bonuses(value),
            Attribute::SpellPower(sp) => GetBonuses::<_SpellPower>::get_bonuses(sp, value),
            Attribute::SpellCriticalChance(sp) => {
                GetBonuses::<_SpellCriticalChance>::get_bonuses(sp, value)
            }
            Attribute::SpellCriticalDamage(sp) => {
                GetBonuses::<_SpellCriticalDamage>::get_bonuses(sp, value)
            }
            Attribute::Weapon(stat) => stat.get_bonuses(value),
            Attribute::ArmorClass(ac) => ac.get_bonuses(value),
            Attribute::ClassLevel(cl) => cl.get_bonuses(value),
            _ => None,
        }
    }
}

impl CloneBonus for Attribute {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::Ability(ability) => ability.clone_bonus(bonus),
            Self::Skill(skill) => skill.clone_bonus(bonus),
            Self::Sheltering(sheltering) => sheltering.clone_bonus(bonus),
            Self::SpellPower(sp)
            | Self::SpellCriticalChance(sp)
            | Self::SpellCriticalDamage(sp) => sp.clone_bonus(bonus),
            Self::SavingThrow(st) => st.clone_bonus(bonus),
            _ => None,
        }
    }
}
