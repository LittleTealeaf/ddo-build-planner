use enum_map::Enum;

use crate::item::types::WeaponType;

/// Proficiency Class for weapons
#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum ProficiencyClass {
    /// Simple Weapons
    Simple,
    /// Martial Weapons
    Martial,
    /// Exotic WEapons
    Exotic,
}

impl ProficiencyClass {
    // TODO: Make this more efficient by reversing the process?
    fn get_weapon_types(&self) -> Vec<WeaponType> {
        (0..WeaponType::LENGTH)
            .into_iter()
            .map(WeaponType::from_usize)
            .filter(|wt| ProficiencyClass::from(*wt).eq(self))
            .collect()
    }
}

impl From<WeaponType> for ProficiencyClass {
    fn from(value: WeaponType) -> Self {
        match value {
            WeaponType::Club
            | WeaponType::Quarterstaff
            | WeaponType::Dagger
            | WeaponType::Sickle
            | WeaponType::LightMace
            | WeaponType::HeavyMace
            | WeaponType::Morningstar
            | WeaponType::HeavyCrossbow
            | WeaponType::LightCrossbow
            | WeaponType::ThrowingAxe
            | WeaponType::ThrowingDagger
            | WeaponType::ThrowingHammer
            | WeaponType::Dart => Self::Simple,
            WeaponType::Handaxe
            | WeaponType::BattleAxe
            | WeaponType::GreatAxe
            | WeaponType::Kukri
            | WeaponType::LongSword
            | WeaponType::GreatSword
            | WeaponType::Scimitar
            | WeaponType::Falchion
            | WeaponType::LongBow
            | WeaponType::ShortSword
            | WeaponType::Rapier
            | WeaponType::HeavyPick
            | WeaponType::LightPick
            | WeaponType::LightHammer
            | WeaponType::WarHammer
            | WeaponType::Maul
            | WeaponType::GreatClub
            | WeaponType::ShortBow => Self::Martial,
            WeaponType::BastardSword
            | WeaponType::DwarvenWarAxe
            | WeaponType::Kama
            | WeaponType::Khopesh
            | WeaponType::Handwraps
            | WeaponType::GreatCrossbow
            | WeaponType::RepeatingHeavyCrossbow
            | WeaponType::RepeatingLightCrossbow
            | WeaponType::Shuriken => Self::Exotic,
        }
    }
}
