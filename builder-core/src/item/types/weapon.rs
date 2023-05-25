use crate::item::types::WeaponCategory;
use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum WeaponType {
    Club,
    QuarterStaff,
    Dagger,
    Sickle,
    LightMace,
    HeavyMace,
    Morningstar,
    HeavyCrossbow,
    LightCrossbow,
    Handaxe,
    Battleaxe,
    GreatAxe,
    Kukri,
    LongSword,
    GreatSword,
    Scimitar,
    Falchion,
    LongBow,
    ShortSword,
    Rapier,
    HeavyPick,
    LightPick,
    LightHammer,
    WarHammer,
    Maul,
    GreatClub,
    ShortBow,
    BastardSword,
    DwarvenWarAxe,
    Kama,
    Khopesh,
    Handwraps,
    // RuneArm, Rune arm is a shield
    GreatCrossbow,
    RepeatingHeavyCrossbow,
    RepeatingLightCrossbow,
    ThrowingAxe,
    ThrowingDagger,
    ThrowingHammer,
    Dart,
    Shuriken,
}

impl ToString for WeaponType {
    fn to_string(&self) -> String {
        match self {
            WeaponType::QuarterStaff => String::from("Quarterstaff"),
            WeaponType::Club => String::from("Club"),
            WeaponType::Dagger => String::from("Dagger"),
            WeaponType::Sickle => String::from("Sickle"),
            WeaponType::LightMace => String::from("Light Mace"),
            WeaponType::HeavyMace => String::from("Heavy Mace"),
            WeaponType::Morningstar => String::from("Morningstar"),
            WeaponType::HeavyCrossbow => String::from("Heavy Crossbow"),
            WeaponType::LightCrossbow => String::from("Light Crossbow"),
            WeaponType::Handaxe => String::from("Handaxe"),
            WeaponType::Battleaxe => String::from("Battleaxe"),
            WeaponType::GreatAxe => String::from("Great Axe"),
            WeaponType::Kukri => String::from("Kukri"),
            WeaponType::LongSword => String::from("Long Sword"),
            WeaponType::GreatSword => String::from("Great Sword"),
            WeaponType::Scimitar => String::from("Scimitar"),
            WeaponType::Falchion => String::from("Falchion"),
            WeaponType::LongBow => String::from("Long Bow"),
            WeaponType::ShortSword => String::from("Short Sword"),
            WeaponType::Rapier => String::from("Rapier"),
            WeaponType::HeavyPick => String::from("Heavy Pick"),
            WeaponType::LightPick => String::from("Light Pick"),
            WeaponType::LightHammer => String::from("Light Hammer"),
            WeaponType::WarHammer => String::from("War Hammer"),
            WeaponType::Maul => String::from("Maul"),
            WeaponType::GreatClub => String::from("Great Club"),
            WeaponType::ShortBow => String::from("Short Bow"),
            WeaponType::BastardSword => String::from("Bastard Sword"),
            WeaponType::DwarvenWarAxe => String::from("Dwarven War Axe"),
            WeaponType::Kama => String::from("Kama"),
            WeaponType::Khopesh => String::from("Khopesh"),
            WeaponType::Handwraps => String::from("Handwraps"),
            WeaponType::GreatCrossbow => String::from("Great Crossbow"),
            WeaponType::RepeatingHeavyCrossbow => String::from("Repeating Heavy Crossbow"),
            WeaponType::RepeatingLightCrossbow => String::from("Repeating Light Crossbow"),
            WeaponType::ThrowingAxe => String::from("Throwing Axe"),
            WeaponType::ThrowingDagger => String::from("Throwing Dagger"),
            WeaponType::ThrowingHammer => String::from("Throwing Hammer"),
            WeaponType::Dart => String::from("Dart"),
            WeaponType::Shuriken => String::from("Shuriken"),
        }
    }
}

impl From<WeaponType> for WeaponCategory {
    fn from(value: WeaponType) -> Self {
        match value {
            WeaponType::Club
            | WeaponType::QuarterStaff
            | WeaponType::LightMace
            | WeaponType::HeavyMace
            | WeaponType::Morningstar
            | WeaponType::LightHammer
            | WeaponType::WarHammer
            | WeaponType::Maul
            | WeaponType::GreatClub
            | WeaponType::Handwraps => WeaponCategory::Bludgeoning,
            WeaponType::Dagger
            | WeaponType::ShortSword
            | WeaponType::Rapier
            | WeaponType::HeavyPick
            | WeaponType::LightPick => WeaponCategory::Piercing,
            WeaponType::Sickle
            | WeaponType::Handaxe
            | WeaponType::Battleaxe
            | WeaponType::GreatAxe
            | WeaponType::Kukri
            | WeaponType::LongSword
            | WeaponType::GreatSword
            | WeaponType::Scimitar
            | WeaponType::Falchion
            | WeaponType::BastardSword
            | WeaponType::DwarvenWarAxe
            | WeaponType::Kama
            | WeaponType::Khopesh => WeaponCategory::Slashing,
            WeaponType::HeavyCrossbow
            | WeaponType::LightCrossbow
            | WeaponType::LongBow
            | WeaponType::ShortBow
            | WeaponType::GreatCrossbow
            | WeaponType::RepeatingHeavyCrossbow
            | WeaponType::RepeatingLightCrossbow => WeaponCategory::Ranged,
            WeaponType::ThrowingAxe
            | WeaponType::ThrowingDagger
            | WeaponType::ThrowingHammer
            | WeaponType::Dart
            | WeaponType::Shuriken => WeaponCategory::Thrown,
        }
    }
}
