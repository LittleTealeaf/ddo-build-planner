//! Attributes focused on Spellcasting

use core::fmt;
use core::fmt::Display;

use serde::{Deserialize, Serialize};
use utils::{chain_tree, enums::StaticOptions, public_modules};

use crate::{
    attribute::{Attribute, ToAttribute},
    bonus::{Bonus, CloneBonus},
};

public_modules!(points, power, school, selector);

/// Attributes related to spellcasting
#[derive(Hash, PartialEq, Eq, Clone, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Spellcasting {
    /// Spell Points
    #[serde(rename = "p", alias = "Points")]
    Points(SpellPoints),
    /// Spell Power
    #[serde(rename = "sp", alias = "SpellPower")]
    SpellPower(SpellPower),
    /// Spell Critical Chance
    #[serde(rename = "cc", alias = "CriticalChance")]
    CriticalChance(SpellPower),
    /// Spell Critical Damage
    #[serde(rename = "cd", alias = "CriticalDamage")]
    CriticalDamage(SpellPower),
    /// Caster Level
    #[serde(rename = "cl", alias = "CasterLevel")]
    CasterLevel(SpellSelector),
    /// Max Caster Level
    #[serde(rename = "mc", alias = "MaxCasterLevel")]
    MaxCasterLevel(SpellSelector),
    /// Spell DC
    #[serde(rename = "dc", alias = "SpellDC")]
    SpellDC(SpellSelector),
    /// Spell Penetration
    #[serde(rename = "pe", alias = "SpellPenetration")]
    SpellPenetration,
}

impl Display for Spellcasting {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Points(sp) => write!(f, "{sp}"),
            Self::SpellPower(sp) => write!(f, "{sp} Spell Power"),
            Self::CriticalChance(sp) => write!(f, "{sp} Spell Critical Chance"),
            Self::CriticalDamage(sp) => write!(f, "{sp} Spell Critical Damage"),
            Self::CasterLevel(cl) => write!(f, "{cl} Caster Level"),
            Self::MaxCasterLevel(cl) => write!(f, "{cl} Max Caster Level"),
            Self::SpellDC(cl) => write!(f, "{cl} Spell DC"),
            Self::SpellPenetration => write!(f, "Spell Penetration"),
        }
    }
}

impl ToAttribute for Spellcasting {
    fn to_attribute(self) -> Attribute {
        Attribute::Spellcasting(self)
    }
}

impl CloneBonus for Spellcasting {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::SpellPower(sp) | Self::CriticalChance(sp) | Self::CriticalDamage(sp) => {
                sp.clone_bonus(bonus)
            }
            _ => None,
        }
    }
}

impl StaticOptions for Spellcasting {
    fn get_static() -> impl Iterator<Item = Self> {
        chain_tree!(
            [Self::SpellPenetration,],
            SpellPoints::get_static().map(Self::Points),
            SpellPower::get_static().flat_map(|sp| {
                [
                    Self::SpellPower(sp),
                    Self::CriticalChance(sp),
                    Self::CriticalDamage(sp),
                ]
            }),
            SpellSelector::get_static().flat_map(|s| {
                [
                    Self::CasterLevel(s),
                    Self::MaxCasterLevel(s),
                    Self::SpellDC(s),
                ]
            })
        )
    }
}
