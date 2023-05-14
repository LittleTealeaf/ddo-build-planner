use crate::{attribute::Attribute, simple_enum};

use super::DamageReduction;

simple_enum!(
    WeaponStat, (
        Attack() String::from("Attack"),
        Damage() String::from("Damage"),
        CriticalAttack() String::from("Critical Attack"),
        CriticalDamage() String::from("Critical Damage"),
        CriticalMultiplier() String::from("Critical Multiplier"),
        CriticalMultiplier1920() String::from("Critical Multiplier (19-20)"),
        DamageReductionBypass(damagereduction: DamageReduction) format!("{} Bypass", damagereduction.to_string())
    )
);

impl WeaponStat {
    pub fn custom_to_string(&self, hand: &WeaponHand) -> String {
        match hand {
            WeaponHand::Both => self.to_string(),
            WeaponHand::Main => format!("Main Hand {}", self.to_string()),
            WeaponHand::Off => format!("Off Hand {}", self.to_string()),
        }
    }

    pub fn get_cloned_attributes(&self, hand: &WeaponHand) -> Option<Vec<Attribute>> {
        Some(
            match hand {
                WeaponHand::Both => Some(vec![WeaponHand::Main, WeaponHand::Off]),
                _ => None,
            }?
            .into_iter()
            .map(|item| Attribute::WeaponStat(item, *self))
            .collect(),
        )
    }
}

simple_enum!(WeaponHand, (Main "Main Hand", Off "Off Hand", Both "Both Hand"));
