use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::{Attribute, DefaultBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
};

use super::SpellPower;

/// Different skills that the character can have.
#[cfg_attr(feature = "enum_ord", derive(enum_map::Enum))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Skill {
    /// All Skills
    All,
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
}

impl Skill {
    /// Returns every possible value of [`Skill`] except for [`Skill::All`]
    pub const VALUES: [Self; 21] = [
        Self::Balance,
        Self::Bluff,
        Self::Concentration,
        Self::Diplomacy,
        Self::DisableDevice,
        Self::Haggle,
        Self::Heal,
        Self::Hide,
        Self::Intimidate,
        Self::Jump,
        Self::Listen,
        Self::MoveSilently,
        Self::OpenLock,
        Self::Perform,
        Self::Repair,
        Self::Search,
        Self::Spellcraft,
        Self::Spot,
        Self::Swim,
        Self::Tumble,
        Self::UseMagicalDevice,
    ];

    fn spell_power_bonus(self, sp: SpellPower, value: f32) -> Bonus {
        Bonus::new(
            Attribute::SpellPower(sp),
            BonusType::Stacking,
            value.into(),
            Attribute::Skill(self).into(),
            None,
        )
    }
}

// impl GetBonuses for Skill {
//     fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
//         match self {
//             Skill::Heal => Some(vec![
//                 self.spell_power_bonus(SpellPower::Positive, value),
//                 self.spell_power_bonus(SpellPower::Negative, value),
//             ]),
//             Skill::Perform => Some(vec![self.spell_power_bonus(SpellPower::Sonic, value)]),
//             Skill::Spellcraft => Some(vec![
//                 self.spell_power_bonus(SpellPower::Acid, value),
//                 self.spell_power_bonus(SpellPower::Cold, value),
//                 self.spell_power_bonus(SpellPower::Electric, value),
//                 self.spell_power_bonus(SpellPower::Fire, value),
//                 self.spell_power_bonus(SpellPower::Force, value),
//                 self.spell_power_bonus(SpellPower::Light, value),
//                 self.spell_power_bonus(SpellPower::Poison, value),
//             ]),
//             _ => None,
//         }
//     }
// }

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
            Self::Balance => write!(f, "Balance"),
            Self::Bluff => write!(f, "Bluff"),
            Self::Concentration => write!(f, "Concentration"),
            Self::Diplomacy => write!(f, "Diplomacy"),
            Self::DisableDevice => write!(f, "Disable Device"),
            Self::Haggle => write!(f, "Haggle"),
            Self::Heal => write!(f, "Heal"),
            Self::Hide => write!(f, "Hide"),
            Self::Intimidate => write!(f, "Intimidate"),
            Self::Jump => write!(f, "Jump"),
            Self::Listen => write!(f, "Listen"),
            Self::MoveSilently => write!(f, "Move Silently"),
            Self::OpenLock => write!(f, "Open Lock"),
            Self::Perform => write!(f, "Perform"),
            Self::Repair => write!(f, "Repair"),
            Self::Search => write!(f, "Search"),
            Self::Spellcraft => write!(f, "Spellcraft"),
            Self::Spot => write!(f, "Spot"),
            Self::Swim => write!(f, "Swim"),
            Self::Tumble => write!(f, "Tumble"),
            Self::UseMagicalDevice => write!(f, "Use Magical Device"),
            Self::All => write!(f, "All Skills"),
        }
    }
}

macro_rules! skill_ability_bonus {
    ($ability: ident, $skill: ident) => {
        Bonus::new(
            $crate::attribute::Attribute::Skill(Skill::$skill).into(),
            $crate::bonus::BonusType::AbilityModifier,
            $crate::attribute::Attribute::AbilityModifier(
                $crate::attribute::types::Ability::$ability,
            )
            .into(),
            $crate::bonus::BonusSource::Base,
            None,
        )
    };
}

impl DefaultBonuses for Skill {
    fn get_default_bonuses() -> Vec<Bonus> {
        vec![
            skill_ability_bonus!(Dexterity, Balance),
            skill_ability_bonus!(Charisma, Bluff),
            skill_ability_bonus!(Constitution, Concentration),
            skill_ability_bonus!(Charisma, Diplomacy),
            skill_ability_bonus!(Intelligence, DisableDevice),
            skill_ability_bonus!(Charisma, Haggle),
            skill_ability_bonus!(Wisdom, Heal),
            skill_ability_bonus!(Dexterity, Hide),
            skill_ability_bonus!(Charisma, Intimidate),
            skill_ability_bonus!(Strength, Jump),
            skill_ability_bonus!(Wisdom, Listen),
            skill_ability_bonus!(Dexterity, MoveSilently),
            skill_ability_bonus!(Dexterity, OpenLock),
            skill_ability_bonus!(Charisma, Perform),
            skill_ability_bonus!(Intelligence, Repair),
            skill_ability_bonus!(Intelligence, Search),
            skill_ability_bonus!(Intelligence, Spellcraft),
            skill_ability_bonus!(Wisdom, Spot),
            skill_ability_bonus!(Strength, Swim),
            skill_ability_bonus!(Dexterity, Tumble),
            skill_ability_bonus!(Charisma, UseMagicalDevice),
        ]
    }
}

impl TrackAttribute for Skill {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(Skill);

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
