mod shield_class;
mod weapon_class;

use core::fmt;

use itertools::chain;
use serde::{Deserialize, Serialize};
pub use shield_class::*;
use utils::enums::StaticOptions;
pub use weapon_class::*;

use fmt::Display;

use crate::{
    bonus::{Bonus, CloneBonus},
    feat::{Feat, ToFeat},
    types::item_type::WeaponType,
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
                .map(|weapon| bonus.clone_into_attribute(Self::WeaponProficiency(weapon)))
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
                .map(|weapon| bonus.clone_into_attribute(Self::WeaponProficiency(weapon)))
                .to_vec(),
            ),
            _ => None,
        }
    }
}

impl Display for Proficiency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

impl ToFeat for Proficiency {
    fn to_feat(self) -> Feat {
        Feat::Proficiency(self)
    }
}

impl StaticOptions for Proficiency {
    fn get_static() -> impl Iterator<Item = Self> {
        chain!(
            [
                Self::SimpleWeaponProficiency,
                Self::MartialWeaponProficiency,
                Self::RuneArm
            ],
            WeaponType::get_static().map(Self::WeaponProficiency),
            ShieldProficiency::get_static().map(Self::Shield),
        )
    }
}
