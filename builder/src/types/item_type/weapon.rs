use core::fmt;

use fmt::Display;

use serde::{Deserialize, Serialize};
use utils::enums::StaticOptions;

/// Weapon Types
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
pub enum WeaponType {
    /// Club
    Club,
    /// Quarterstaff
    Quarterstaff,
    /// Dagger
    Dagger,
    /// Sickle
    Sickle,
    /// Light Mace
    LightMace,
    /// Heavy Mace
    HeavyMace,
    /// Morningstar
    Morningstar,
    /// Heavy Crossbow
    HeavyCrossbow,
    /// Light Crossbow
    LightCrossbow,
    /// Handaxe
    Handaxe,
    /// Battle Axe
    BattleAxe,
    /// Great Axe
    GreatAxe,
    /// Kukri
    Kukri,
    /// Long Sword
    LongSword,
    /// Great Sword
    GreatSword,
    /// Scimitar
    Scimitar,
    /// Falchion
    Falchion,
    /// Long Bow
    LongBow,
    /// Short Sword
    ShortSword,
    /// Rapier
    Rapier,
    /// Heavy Pick
    HeavyPick,
    /// Light Pick
    LightPick,
    /// Light Hammer
    LightHammer,
    /// War Hammer
    WarHammer,
    /// Maul
    Maul,
    /// Great Club
    GreatClub,
    /// Short Bow
    ShortBow,
    /// Bastard Sword
    BastardSword,
    /// Dwarven War Axe
    DwarvenWarAxe,
    /// Kama
    Kama,
    /// Khopesh
    Khopesh,
    /// Handwraps
    Handwraps,
    /// Great Crossbow
    GreatCrossbow,
    /// Repeating Heavy Crossbow
    RepeatingHeavyCrossbow,
    /// Repeating Light Crossbow
    RepeatingLightCrossbow,
    /// Throwing Axe
    ThrowingAxe,
    /// Throwing Dagger
    ThrowingDagger,
    /// Throwing Hammer
    ThrowingHammer,
    /// Dart
    Dart,
    /// Shuriken
    Shuriken,
}

impl WeaponType {
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
    pub const MELEE_WEAPONS: [Self; 29] = [
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
    pub const TWO_HANDED_FIGHTING: [Self; 5] = [
        Self::Falchion,
        Self::GreatAxe,
        Self::GreatClub,
        Self::Quarterstaff,
        Self::Maul,
    ];

    /// Weapons that are single-handed melee weapons
    pub const SINGLE_HANDED_WEAPON: [Self; 20] = [
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

impl StaticOptions for WeaponType {
    fn get_static() -> impl Iterator<Item = Self> {
        [
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
        ]
        .into_iter()
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
