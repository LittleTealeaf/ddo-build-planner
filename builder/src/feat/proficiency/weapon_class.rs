use enum_map::Enum;

use crate::item::types::WeaponType;

/// Proficiency Class for weapons
#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum WeaponProficiencyClass {
    /// Simple Weapons
    Simple,
    /// Martial Weapons
    Martial,
    /// Exotic WEapons
    Exotic,
}

impl WeaponProficiencyClass {
    // TODO: Make this more efficient by reversing the process?
    fn get_weapon_types(&self) -> Vec<WeaponType> {
        (0..WeaponType::LENGTH)
            .map(WeaponType::from_usize)
            .filter(|wt| WeaponProficiencyClass::from(*wt).eq(self))
            .collect()
    }
}

impl From<WeaponType> for WeaponProficiencyClass {
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

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn fully_describes_weapon_types() {
        let classes = (0..WeaponProficiencyClass::LENGTH)
            .map(WeaponProficiencyClass::from_usize)
            .collect_vec();

        let mut set = Vec::new();

        for class in classes {
            for weapon_type in class.get_weapon_types() {
                assert!(!set.contains(&weapon_type));
                set.push(weapon_type);
            }
        }
    }
}