mod shield_class;
mod weapon_class;

use serde::{Deserialize, Serialize};
pub use shield_class::*;
pub use weapon_class::*;

use std::fmt::Display;

use crate::{
    bonus::{Bonus, CloneBonus},
    feat::Feat,
    types::item::WeaponType,
};

/// Proficiencies for Weapons and Armor
#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Proficiency {
    /// Proficiency for each weapon.
    WeaponProficiency(WeaponType),
    /// Proficiency for Simple Weapons
    SimpleWeaponProficiency,
    /// Proficiency for Martial Weapons
    MartialWeaponProficiency,
    /// Shield Proficiency
    Shield(ShieldProficiency),
    /// Rune Arm Proficiency
    RuneArm,
}

impl CloneBonus for Proficiency {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        match self {
            Self::SimpleWeaponProficiency => Some(
                [
                    WeaponType::Club,
                    WeaponType::Dagger,
                    WeaponType::LightMace,
                    WeaponType::HeavyMace,
                    WeaponType::Morningstar,
                    WeaponType::Quarterstaff,
                    WeaponType::Sickle,
                    WeaponType::Dart,
                    WeaponType::LightCrossbow,
                    WeaponType::HeavyCrossbow,
                    WeaponType::ThrowingDagger,
                ]
                .map(|weapon| {
                    bonus.clone_into_attribute(Feat::Proficiency(Self::WeaponProficiency(weapon)))
                })
                .to_vec(),
            ),
            Self::MartialWeaponProficiency => Some(
                [
                    WeaponType::Falchion,
                    WeaponType::GreatAxe,
                    WeaponType::GreatClub,
                    WeaponType::GreatSword,
                    WeaponType::Maul,
                    WeaponType::BattleAxe,
                    WeaponType::Handaxe,
                    WeaponType::LightHammer,
                    WeaponType::Kukri,
                    WeaponType::LongSword,
                    WeaponType::LightPick,
                    WeaponType::HeavyPick,
                    WeaponType::Rapier,
                    WeaponType::Scimitar,
                    WeaponType::ShortSword,
                    WeaponType::WarHammer,
                    WeaponType::LongBow,
                    WeaponType::ShortBow,
                    WeaponType::ThrowingAxe,
                    WeaponType::ThrowingHammer,
                ]
                .map(|weapon| {
                    bonus.clone_into_attribute(Feat::Proficiency(Self::WeaponProficiency(weapon)))
                })
                .to_vec(),
            ),
            _ => None,
        }
    }
}

impl Display for Proficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WeaponProficiency(weapon) => write!(f, "{weapon} Proficiency"),
            Self::SimpleWeaponProficiency => write!(f, "Simple Weapon Proficiency"),
            Self::MartialWeaponProficiency => write!(f, "Martial Weapon Proficiency"),
            Self::Shield(shield) => write!(f, "{shield} Shield Proficiency"),
            Self::RuneArm => write!(f, "Rune Arm Proficiency"),
        }
    }
}

impl From<WeaponType> for Proficiency {
    fn from(value: WeaponType) -> Self {
        Self::WeaponProficiency(value)
    }
}

impl From<ShieldProficiency> for Proficiency {
    fn from(value: ShieldProficiency) -> Self {
        Self::Shield(value)
    }
}
