use core::fmt::{self, Display};

use itertools::chain;
use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, CloneBonus},
    types::{damage_type::DamageType, spellcasting::Spellcasting},
};

/// Defines specific spell powers that a player can boost to increase damage for spells of that
/// type.
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpellPower {
    /// Provides a Universal typed bonus to all spell powers
    #[serde(rename = "uni", alias = "Universal")]
    Universal,
    /// Provides an individual bonus of the same type to all spell powers
    #[serde(rename = "ptc", alias = "Potency")]
    Potency,
    /// Spell Power for specific damage types
    #[serde(rename = "d", alias = "Damage")]
    Damage(DamageType),
}

impl SpellPower {
    /// The list of all tracked spell powers in the character sheet
    pub const SPELL_POWERS: [Self; 12] = [
        Self::Damage(DamageType::Acid),
        Self::Damage(DamageType::Fire),
        Self::Damage(DamageType::Cold),
        Self::Damage(DamageType::Electric),
        Self::Damage(DamageType::Sonic),
        Self::Damage(DamageType::Positive),
        Self::Damage(DamageType::Negative),
        Self::Damage(DamageType::Poison),
        Self::Damage(DamageType::Repair),
        Self::Damage(DamageType::Rust),
        Self::Damage(DamageType::Alignment),
        Self::Damage(DamageType::Light),
    ];
}

impl From<DamageType> for SpellPower {
    fn from(value: DamageType) -> Self {
        Self::Damage(value)
    }
}

impl Display for SpellPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Universal => write!(f, "Universal"),
            Self::Potency => write!(f, "Potency"),
            Self::Damage(damage) => damage.fmt(f),
        }
    }
}

impl StaticOptions for SpellPower {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [Self::Universal, Self::Potency,],
            DamageType::get_static().map(Self::Damage)
        )
    }
}

impl CloneBonus for SpellPower {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        let Attribute::Spellcasting(atr) = bonus.attribute() else {
            return None;
        };

        Some(
            match atr {
                Spellcasting::SpellPower(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Spellcasting::SpellPower))
                }
                Spellcasting::CriticalChance(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Spellcasting::CriticalChance))
                }
                Spellcasting::CriticalDamage(Self::Potency) => {
                    Some(Self::SPELL_POWERS.map(Spellcasting::CriticalDamage))
                }
                _ => None,
            }?
            .map(|attribute| bonus.clone_into_attribute(attribute))
            .to_vec(),
        )
    }
}
