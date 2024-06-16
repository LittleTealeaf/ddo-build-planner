use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticValues;

/// Weapon Types
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponType {
    /// Bastard Sword
    #[serde(rename = "bs", alias = "BastardSword")]
    BastardSword,
    /// Battle Axe
    #[serde(rename = "ba", alias = "BattleAxe")]
    BattleAxe,
    /// Club
    #[serde(rename = "c", alias = "Club")]
    Club,
    /// Dagger
    #[serde(rename = "d", alias = "Dagger")]
    Dagger,
    /// Dart
    #[serde(rename = "da", alias = "Dart")]
    Dart,
    /// Dwarven War Axe
    #[serde(rename = "dwa", alias = "DwarvenWarAxe")]
    DwarvenWarAxe,
    /// Falchion
    #[serde(rename = "f", alias = "Flachion")]
    Falchion,
    /// Great Axe
    #[serde(rename = "ga", alias = "GreatAxe")]
    GreatAxe,
    /// Great Club
    #[serde(rename = "gc", alias = "GreatClub")]
    GreatClub,
    /// Great Crossbow
    #[serde(rename = "gx", alias = "GreatCrossbow")]
    GreatCrossbow,
    /// Great Sword
    #[serde(rename = "gs", alias = "GreatSword")]
    GreatSword,
    /// Handaxe
    #[serde(rename = "ha", alias = "Handaxe")]
    Handaxe,
    /// Handwraps
    #[serde(rename = "h", alias = "Handwraps")]
    Handwraps,
    /// Heavy Crossbow
    #[serde(rename = "hc", alias = "HeavyCrossbow")]
    HeavyCrossbow,
    /// Heavy Mace
    #[serde(rename = "hm", alias = "HeavyMace")]
    HeavyMace,
    /// Heavy Pick
    #[serde(rename = "hp", alias = "HeavyPick")]
    HeavyPick,
    /// Kama
    #[serde(rename = "ka", alias = "Kama")]
    Kama,
    /// Khopesh
    #[serde(rename = "kh", alias = "Khopesh")]
    Khopesh,
    /// Kukri
    #[serde(rename = "ku", alias = "Kukri")]
    Kukri,
    /// Light Crossbow
    #[serde(rename = "lc", alias = "LightCrossbow")]
    LightCrossbow,
    /// Light Hammer
    #[serde(rename = "lh", alias = "LightHammer")]
    LightHammer,
    /// Light Mace
    #[serde(rename = "lm", alias = "LightMace")]
    LightMace,
    /// Light Pick
    #[serde(rename = "lp", alias = "LightPick")]
    LightPick,
    /// Long Bow
    #[serde(rename = "lb", alias = "LongBow")]
    LongBow,
    /// Long Sword
    #[serde(rename = "ls", alias = "LongSword")]
    LongSword,
    /// Maul
    #[serde(rename = "m", alias = "Maul")]
    Maul,
    /// Morningstar
    #[serde(rename = "ms", alias = "Morningstar")]
    Morningstar,
    /// Quarterstaff
    #[serde(rename = "q", alias = "Quarterstaff")]
    Quarterstaff,
    /// Rapier
    #[serde(rename = "r", alias = "Rapier")]
    Rapier,
    /// Repeating Heavy Crossbow
    #[serde(rename = "rh", alias = "RepeatingHeavyCrossbow")]
    RepeatingHeavyCrossbow,
    /// Repeating Light Crossbow
    #[serde(rename = "rl", alias = "RepeatingLightCrossbow")]
    RepeatingLightCrossbow,
    /// Scimitar
    #[serde(rename = "sc", alias = "Scimitar")]
    Scimitar,
    /// Sickle
    #[serde(rename = "si", alias = "Sickle")]
    Sickle,
    /// Short Sword
    #[serde(rename = "ss", alias = "ShortSword")]
    ShortSword,
    /// Short Bow
    #[serde(rename = "sb", alias = "ShortBow")]
    ShortBow,
    /// Shuriken
    #[serde(rename = "s", alias = "Shuriken")]
    Shuriken,
    /// Throwing Axe
    #[serde(rename = "ta", alias = "ThrowingAxe")]
    ThrowingAxe,
    /// Throwing Dagger
    #[serde(rename = "td", alias = "ThrowingDagger")]
    ThrowingDagger,
    /// Throwing Hammer
    #[serde(rename = "th", alias = "ThrowingHammer")]
    ThrowingHammer,
    /// War Hammer
    #[serde(rename = "w", alias = "WarHammer")]
    WarHammer,
}

impl WeaponType {
    /// List of every weapon type
    pub const ALL: [Self; 40] = [
        Self::Club,
        Self::Quarterstaff,
        Self::Dagger,
        Self::Sickle,
        Self::LightMace,
        Self::HeavyMace,
        Self::Morningstar,
        Self::HeavyCrossbow,
        Self::LightCrossbow,
        Self::Handaxe,
        Self::BattleAxe,
        Self::GreatAxe,
        Self::Kukri,
        Self::LongSword,
        Self::GreatSword,
        Self::Scimitar,
        Self::Falchion,
        Self::LongBow,
        Self::ShortSword,
        Self::Rapier,
        Self::HeavyPick,
        Self::LightPick,
        Self::LightHammer,
        Self::WarHammer,
        Self::Maul,
        Self::GreatClub,
        Self::ShortBow,
        Self::BastardSword,
        Self::DwarvenWarAxe,
        Self::Kama,
        Self::Khopesh,
        Self::Handwraps,
        Self::GreatCrossbow,
        Self::RepeatingHeavyCrossbow,
        Self::RepeatingLightCrossbow,
        Self::ThrowingAxe,
        Self::ThrowingDagger,
        Self::ThrowingHammer,
        Self::Dart,
        Self::Shuriken,
    ];

    /// Sub-set of thrown weapons.
    pub const THROWING_WEAPONS: [Self; 5] = [
        Self::Dart,
        Self::Shuriken,
        Self::ThrowingHammer,
        Self::ThrowingDagger,
        Self::ThrowingAxe,
    ];

    /// Sub-set of ranged weapons
    pub const RANGED_WEAPONS: [Self; 7] = [
        Self::LightCrossbow,
        Self::HeavyCrossbow,
        Self::RepeatingHeavyCrossbow,
        Self::RepeatingLightCrossbow,
        Self::LongBow,
        Self::ShortBow,
        Self::GreatCrossbow,
    ];

    /// All Melee Weapons
    pub const MELEE_WEAPONS: [Self; 27] = [
        Self::Club,
        Self::Quarterstaff,
        Self::Dagger,
        Self::Sickle,
        Self::LightMace,
        Self::HeavyMace,
        Self::Morningstar,
        Self::Handaxe,
        Self::GreatAxe,
        Self::Kukri,
        Self::LongSword,
        Self::GreatSword,
        Self::Scimitar,
        Self::Falchion,
        Self::ShortSword,
        Self::Rapier,
        Self::HeavyPick,
        Self::LightPick,
        Self::LightHammer,
        Self::WarHammer,
        Self::Maul,
        Self::GreatClub,
        Self::BastardSword,
        Self::DwarvenWarAxe,
        Self::Kama,
        Self::Khopesh,
        Self::Handwraps,
    ];

    /// Weapons that count as melee two-handed-fighting weapons
    pub const TWO_HANDED_MELEE_WEAPONS: [Self; 5] = [
        Self::Falchion,
        Self::GreatAxe,
        Self::GreatClub,
        Self::Quarterstaff,
        Self::Maul,
    ];

    /// Weapons that are single-handed melee weapons
    pub const ONE_HANDED_MELEE_WEAPONS: [Self; 20] = [
        Self::Club,
        Self::Dagger,
        Self::Sickle,
        Self::LightMace,
        Self::HeavyMace,
        Self::Morningstar,
        Self::Handaxe,
        Self::Kukri,
        Self::LongSword,
        Self::Scimitar,
        Self::ShortSword,
        Self::Rapier,
        Self::HeavyPick,
        Self::LightPick,
        Self::LightHammer,
        Self::WarHammer,
        Self::BastardSword,
        Self::DwarvenWarAxe,
        Self::Kama,
        Self::Khopesh,
    ];
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Club => write!(f, "Club"),
            Self::Quarterstaff => write!(f, "Quarterstaff"),
            Self::Dagger => write!(f, "Dagger"),
            Self::Sickle => write!(f, "Sickle"),
            Self::LightMace => write!(f, "Light Mace"),
            Self::HeavyMace => write!(f, "Heavy Mace"),
            Self::Morningstar => write!(f, "Morningstar"),
            Self::HeavyCrossbow => write!(f, "Heavy Crossbow"),
            Self::LightCrossbow => write!(f, "Light Crossbow"),
            Self::Handaxe => write!(f, "Handaxe"),
            Self::BattleAxe => write!(f, "Battle Axe"),
            Self::GreatAxe => write!(f, "Great Axe"),
            Self::Kukri => write!(f, "Kukri"),
            Self::LongSword => write!(f, "Long Sword"),
            Self::GreatSword => write!(f, "Great Sword"),
            Self::Scimitar => write!(f, "Scimitar"),
            Self::Falchion => write!(f, "Falchion"),
            Self::LongBow => write!(f, "Long Bow"),
            Self::ShortSword => write!(f, "Short Sword"),
            Self::Rapier => write!(f, "Rapier"),
            Self::HeavyPick => write!(f, "Heavy Pick"),
            Self::LightPick => write!(f, "Light Pick"),
            Self::LightHammer => write!(f, "Light Hammer"),
            Self::WarHammer => write!(f, "War Hammer"),
            Self::Maul => write!(f, "Maul"),
            Self::GreatClub => write!(f, "Great Club"),
            Self::ShortBow => write!(f, "Short Bow"),
            Self::BastardSword => write!(f, "Bastard Sword"),
            Self::DwarvenWarAxe => write!(f, "Dwarven War Axe"),
            Self::Kama => write!(f, "Kama"),
            Self::Khopesh => write!(f, "Khopesh"),
            Self::Handwraps => write!(f, "Handwraps"),
            Self::GreatCrossbow => write!(f, "Great Crossbow"),
            Self::RepeatingHeavyCrossbow => write!(f, "Repeating Heavy Crossbow"),
            Self::RepeatingLightCrossbow => write!(f, "Repeating Light Crossbow"),
            Self::ThrowingAxe => write!(f, "Throwing Axe"),
            Self::ThrowingDagger => write!(f, "Throwing Dagger"),
            Self::ThrowingHammer => write!(f, "Throwing Hammer"),
            Self::Dart => write!(f, "Dart"),
            Self::Shuriken => write!(f, "Shuriken"),
        }
    }
}

impl StaticValues for WeaponType {
    fn values() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_repeats_in_thrown_weapons() {
        let mut types = Vec::new();

        for item in WeaponType::THROWING_WEAPONS {
            assert!(!types.contains(&item));
            types.push(item);
        }
    }

    #[test]
    fn no_repeats_in_ranged_weapons() {
        let mut types = Vec::new();

        for item in WeaponType::RANGED_WEAPONS {
            assert!(!types.contains(&item));
            types.push(item);
        }
    }
}
