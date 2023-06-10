use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Weapon Types
#[cfg_attr(test, derive(enum_map::Enum))]
#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Serialize, Deserialize)]
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
}

impl Display for WeaponType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WeaponType::Club => write!(f, "Club"),
            WeaponType::Quarterstaff => write!(f, "Quarterstaff"),
            WeaponType::Dagger => write!(f, "Dagger"),
            WeaponType::Sickle => write!(f, "Sickle"),
            WeaponType::LightMace => write!(f, "Light Mace"),
            WeaponType::HeavyMace => write!(f, "Heavy Mace"),
            WeaponType::Morningstar => write!(f, "Morningstar"),
            WeaponType::HeavyCrossbow => write!(f, "Heavy Crossbow"),
            WeaponType::LightCrossbow => write!(f, "Light Crossbow"),
            WeaponType::Handaxe => write!(f, "Handaxe"),
            WeaponType::BattleAxe => write!(f, "Battle Axe"),
            WeaponType::GreatAxe => write!(f, "Great Axe"),
            WeaponType::Kukri => write!(f, "Kukri"),
            WeaponType::LongSword => write!(f, "Long Sword"),
            WeaponType::GreatSword => write!(f, "Great Sword"),
            WeaponType::Scimitar => write!(f, "Scimitar"),
            WeaponType::Falchion => write!(f, "Falchion"),
            WeaponType::LongBow => write!(f, "Long Bow"),
            WeaponType::ShortSword => write!(f, "Short Sword"),
            WeaponType::Rapier => write!(f, "Rapier"),
            WeaponType::HeavyPick => write!(f, "Heavy Pick"),
            WeaponType::LightPick => write!(f, "Light Pick"),
            WeaponType::LightHammer => write!(f, "Light Hammer"),
            WeaponType::WarHammer => write!(f, "War Hammer"),
            WeaponType::Maul => write!(f, "Maul"),
            WeaponType::GreatClub => write!(f, "Great Club"),
            WeaponType::ShortBow => write!(f, "Short Bow"),
            WeaponType::BastardSword => write!(f, "Bastard Sword"),
            WeaponType::DwarvenWarAxe => write!(f, "Dwarven War Axe"),
            WeaponType::Kama => write!(f, "Kama"),
            WeaponType::Khopesh => write!(f, "Khopesh"),
            WeaponType::Handwraps => write!(f, "Handwraps"),
            WeaponType::GreatCrossbow => write!(f, "Great Crossbow"),
            WeaponType::RepeatingHeavyCrossbow => write!(f, "Repeating Heavy Crossbow"),
            WeaponType::RepeatingLightCrossbow => write!(f, "Repeating Light Crossbow"),
            WeaponType::ThrowingAxe => write!(f, "Throwing Axe"),
            WeaponType::ThrowingDagger => write!(f, "Throwing Dagger"),
            WeaponType::ThrowingHammer => write!(f, "Throwing Hammer"),
            WeaponType::Dart => write!(f, "Dart"),
            WeaponType::Shuriken => write!(f, "Shuriken"),
        }
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
