use std::fmt::Display;

use enum_map::Enum;
use serde::{Serialize, Deserialize};

use crate::{
    attribute::{Attribute, GetBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
};

use super::SpellPower;

/// Different skills that the character can have.
#[derive(Clone, Copy, PartialEq, Eq, Enum, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    /// Balance Skill
    Balance,
    /// Bluff Skill
    Bluff,
    /// Concentration Skill
    Concentration,
    /// Diplomacy Skill
    Diplomacy,
    /// Disable Device Skill
    DisableDevice,
    /// Haggle Skill
    Haggle,
    /// Heal Skill
    Heal,
    /// Hide Skill
    Hide,
    /// Intimidate Skill
    Intimidate,
    /// Jump Skill
    Jump,
    /// Listen Skill
    Listen,
    /// Move Silently Skill
    MoveSilently,
    /// Open Lock Skill
    OpenLock,
    /// Perform Skill
    Perform,
    /// Repair Skill
    Repair,
    /// Search Skill
    Search,
    /// Spellcraft Skill
    Spellcraft,
    /// Spot Skill
    Spot,
    /// Swim Skill
    Swim,
    /// Tumble Skill
    Tumble,
    /// Use Magical Device Skill
    #[serde(rename = "UMD")]
    UseMagicalDevice,
    /// All Skills
    All,
}

impl Skill {
    /// Returns every possible value of [`Skill`] except for [`Skill::All`]
    pub const VALUES: [Skill; 21] = [
        Skill::Balance,
        Skill::Bluff,
        Skill::Concentration,
        Skill::Diplomacy,
        Skill::DisableDevice,
        Skill::Haggle,
        Skill::Heal,
        Skill::Hide,
        Skill::Intimidate,
        Skill::Jump,
        Skill::Listen,
        Skill::MoveSilently,
        Skill::OpenLock,
        Skill::Perform,
        Skill::Repair,
        Skill::Search,
        Skill::Spellcraft,
        Skill::Spot,
        Skill::Swim,
        Skill::Tumble,
        Skill::UseMagicalDevice,
    ];

    fn spell_power_bonus(&self, sp: SpellPower, value: f32) -> Bonus {
        Bonus::new(
            Attribute::SpellPower(sp),
            BonusType::Stacking,
            value.into(),
            Attribute::Skill(*self).into(),
            None,
        )
    }
}

impl GetBonuses for Skill {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Skill::Heal => Some(vec![
                self.spell_power_bonus(SpellPower::Positive, value),
                self.spell_power_bonus(SpellPower::Negative, value),
            ]),
            Skill::Perform => Some(vec![self.spell_power_bonus(SpellPower::Sonic, value)]),
            Skill::Spellcraft => Some(vec![
                self.spell_power_bonus(SpellPower::Acid, value),
                self.spell_power_bonus(SpellPower::Cold, value),
                self.spell_power_bonus(SpellPower::Electric, value),
                self.spell_power_bonus(SpellPower::Fire, value),
                self.spell_power_bonus(SpellPower::Force, value),
                self.spell_power_bonus(SpellPower::Light, value),
                self.spell_power_bonus(SpellPower::Poison, value),
            ]),
            _ => None,
        }
    }
}

impl CloneBonus for Skill {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::VALUES
                .map(|skill| {
                    Bonus::new(
                        skill.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

impl Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Skill::Balance => write!(f, "Balance"),
            Skill::Bluff => write!(f, "Bluff"),
            Skill::Concentration => write!(f, "Concentration"),
            Skill::Diplomacy => write!(f, "Diplomacy"),
            Skill::DisableDevice => write!(f, "Disable Device"),
            Skill::Haggle => write!(f, "Haggle"),
            Skill::Heal => write!(f, "Heal"),
            Skill::Hide => write!(f, "Hide"),
            Skill::Intimidate => write!(f, "Intimidate"),
            Skill::Jump => write!(f, "Jump"),
            Skill::Listen => write!(f, "Listen"),
            Skill::MoveSilently => write!(f, "Move Silently"),
            Skill::OpenLock => write!(f, "Open Lock"),
            Skill::Perform => write!(f, "Perform"),
            Skill::Repair => write!(f, "Repair"),
            Skill::Search => write!(f, "Search"),
            Skill::Spellcraft => write!(f, "Spellcraft"),
            Skill::Spot => write!(f, "Spot"),
            Skill::Swim => write!(f, "Swim"),
            Skill::Tumble => write!(f, "Tumble"),
            Skill::UseMagicalDevice => write!(f, "Use Magical Device"),
            Skill::All => write!(f, "All Skills"),
        }
    }
}

impl TrackAttribute for Skill {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Attribute::Skill(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_is_not_tracked() {
        assert!(!Skill::All.is_tracked());
        assert!(!Attribute::from(Skill::All).is_tracked());
    }

    #[test]
    fn skills_are_tracked() {
        for skill in Skill::VALUES {
            assert!(skill.is_tracked());
            assert!(Attribute::from(skill).is_tracked());
        }
    }
}
