//! Represents each attribute that a character can have
pub mod bonuses;
pub mod flags;
pub mod impls;
pub mod selectors;
pub mod toggles;
mod traits;

mod from;

pub use from::*;
use itertools::chain;
use serde::{Deserialize, Serialize};
pub use traits::*;

use crate::{
    bonus::{Bonus, CloneBonus},
    feat::Feat,
    player_class::PlayerClass,
    types::{Ability, Skill},
};
use std::fmt::Display;

use self::{
    bonuses::{
        ArmorClass, EnergyResistance, SavingThrow, Sheltering, SpellPower, WeaponAttribute,
        _SpellCriticalChance, _SpellCriticalDamage, _SpellPower,
    },
    flags::Flag,
    selectors::SpellSelector,
    toggles::Toggle,
};

/// Describes various traits of a character, ranging from having feats, stats, and much more.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Copy, Clone, Eq, PartialEq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Attribute {
    /// Behaves as a debuggable attribute
    #[cfg(test)]
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
    EnergyResistance(EnergyResistance),
    /// % Damage reduced from energy sources
    EnergyAbsorption(EnergyResistance),
    /// Spell Resistance
    SpellResistance,
    /// Spell Penetration
    SpellPenetration,
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(test)]
            Self::Debug(val) => write!(f, "Debug {val}"),
            Self::Dummy => write!(f, "Dummy"),
            Self::Ability(ability) => write!(f, "{ability} Score"),
            Self::AbilityModifier(ability) => write!(f, "{ability} Modifier"),
            Self::Skill(skill) => skill.fmt(f),
            Self::Toggle(toggle) => toggle.fmt(f),
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
            Self::Flag(fl) => fl.fmt(f),
            Self::EnergyResistance(energy) => write!(f, "{energy} Resistance"),
            Self::EnergyAbsorption(energy) => write!(f, "{energy} Absorption"),
            Self::Feat(feat) => write!(f, "Feat: {feat}"),
            Self::SpellResistance => write!(f, "Spell Resistance"),
            Self::SpellPenetration => write!(f, "Spell Penetration"),
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
    pub fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Self::Toggle(toggle) => toggle.get_bonuses(value),
            Self::SpellPower(sp) => GetBonuses::<_SpellPower>::get_bonuses(sp, value),
            Self::SpellCriticalChance(sp) => {
                GetBonuses::<_SpellCriticalChance>::get_bonuses(sp, value)
            }
            Self::SpellCriticalDamage(sp) => {
                GetBonuses::<_SpellCriticalDamage>::get_bonuses(sp, value)
            }
            Self::Weapon(stat) => stat.get_bonuses(value),
            Self::ClassLevel(cl) => cl.get_bonuses(value),
            Self::Flag(flag) => flag.get_bonuses(value),
            Self::Feat(feat) => feat.get_bonuses(value),
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
            Self::Weapon(stat) => stat.clone_bonus(bonus),
            _ => None,
        }
    }
}

impl TrackAttribute for Attribute {
    fn is_tracked(&self) -> bool {
        match self {
            Self::Dummy => false,
            Self::Ability(ability) | Self::AbilityModifier(ability) => ability.is_tracked(),
            Self::Skill(skill) => skill.is_tracked(),
            Self::SavingThrow(st) => st.is_tracked(),
            Self::Weapon(stat) => stat.is_tracked(),
            Self::Sheltering(sheltering) => sheltering.is_tracked(),
            Self::SpellPower(sp)
            | Self::SpellCriticalChance(sp)
            | Self::SpellCriticalDamage(sp) => sp.is_tracked(),
            _ => true,
        }
    }
}

impl Attribute {
    /// Returns the default bonuses for all attributes. These default bonuses should be included in every new bonus compiler.
    pub fn get_default_bonuses() -> impl Iterator<Item = Bonus> {
        chain!(
            Ability::get_default_bonuses(),
            SavingThrow::get_default_bonuses(),
            SpellPower::get_default_bonuses(),
            ArmorClass::get_default_bonuses(),
            Skill::get_default_bonuses()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(Attribute);

    #[test]
    fn dummy_is_not_tracked() {
        assert!(!Attribute::Dummy.is_tracked());
    }

    mod all_attributes {
        use std::collections::HashSet;

        use crate::bonus::{BonusSource, BonusType};

        use enum_map::Enum;

        use super::*;

        fn get_all_attributes() -> impl Iterator<Item = Attribute> {
            let max = Attribute::LENGTH;

            (0..max).map(Attribute::from_usize)
        }

        #[test]
        fn returns_is_tracked() {
            get_all_attributes().for_each(|attr| {
                attr.is_tracked();
            });
        }

        #[test]
        fn has_unique_display() {
            let mut unique_names = HashSet::new();

            get_all_attributes().for_each(|attr| {
                let name = attr.to_string();
                assert!(
                    !unique_names.contains(&name),
                    "Duplicate Name Found: {attr}"
                );
                unique_names.insert(name);
            });
        }

        #[test]
        fn do_not_clone_to_themselves() {
            get_all_attributes()
                .filter_map(|attr| {
                    Some((
                        attr,
                        attr.clone_bonus(&Bonus::new(
                            attr,
                            BonusType::Stacking,
                            10f32.into(),
                            BonusSource::Debug(0),
                            None,
                        ))?,
                    ))
                })
                .for_each(|(attr, bonuses)| {
                    for bonus in bonuses {
                        assert_ne!(bonus.get_attribute(), attr);
                    }
                });
        }

        #[test]
        fn do_not_clone_into_cloneable_bonuses() {
            get_all_attributes()
                .filter_map(|attr| {
                    Some((
                        attr,
                        attr.clone_bonus(&Bonus::new(
                            attr,
                            BonusType::Stacking,
                            10f32.into(),
                            BonusSource::Debug(0),
                            None,
                        ))?,
                    ))
                })
                .for_each(|(attr, bonuses)| {
                    for bonus in bonuses {
                        assert!(
                            bonus.get_attribute().clone_bonus(&bonus).is_none(),
                            "{} bonus cloned from {} clones into other bonuses",
                            bonus.get_attribute(),
                            attr
                        );
                    }
                });
        }
    }
}
