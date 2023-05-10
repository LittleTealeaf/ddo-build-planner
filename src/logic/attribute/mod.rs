use crate::spell_power_universal_to_others;

use super::{
    bonus::{Bonus, BonusSource},
    feat::Feat,
};

mod sub;
pub use sub::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Attribute {
    Dummy,
    Feat(Feat),
    AbilityScore(Ability),
    AbilityModifier(Ability),
    Skill(Skill),
    SpellPower(SpellPower),
    SpellCriticalChance(SpellPower),
    SpellCriticalDamage(SpellPower),
    SpellFocus(SpellSchool),
    SavingThrow(SavingThrow),
    ElementalAbsortion(ElementalType),
    ElementalResistance(ElementalType),
    MagicalSheltering,
    PhysicalSheltering,
    MagicalShelteringCap,
    WeaponStat(WeaponHand, WeaponStat),
    Offensive(Offensive),
    SetBonus(SetBonus),
    SpellPoints(SpellPoints),
    HealingAmplification(HealingAmplification),
    Flag(Flag),
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        match self {
            Attribute::Dummy => String::from("Dummy"),
            Attribute::Feat(feat) => format!("Feat: {}", feat.to_string()),
            Attribute::AbilityScore(ability) => format!("{} Score", ability.to_string()),
            Attribute::AbilityModifier(ability) => format!("{} Modifier", ability.to_string()),
            Attribute::Skill(skill) => skill.to_string(),
            Attribute::SpellPower(spell_damage_type) => {
                format!("{} Spell Power", spell_damage_type.to_string())
            }
            Attribute::SpellCriticalChance(spell_damage_type) => {
                format!("{} Spell Critical Chance", spell_damage_type.to_string())
            }
            Attribute::SpellCriticalDamage(spell_damage_type) => {
                format!("{} Spell Critical Damage", spell_damage_type.to_string())
            }
            Attribute::SpellFocus(school) => format!("Spell Focus: {}", school.to_string()),
            Attribute::SavingThrow(saving_throw) => {
                format!("{} Saving Throw", saving_throw.to_string())
            }
            Attribute::ElementalAbsortion(element) => format!("{} Absorption", element.to_string()),
            Attribute::ElementalResistance(element) => {
                format!("{} Resistance", element.to_string())
            }
            Attribute::MagicalSheltering => String::from("Magical Sheltering"),
            Attribute::PhysicalSheltering => String::from("Physical Sheltering"),
            Attribute::MagicalShelteringCap => String::from("Magical Sheltering Cap"),
            Attribute::Offensive(offensive) => offensive.to_string(),
            Attribute::SetBonus(set_bonus) => set_bonus.to_string(),
            Attribute::SpellPoints(spell_points) => spell_points.to_string(),
            Attribute::HealingAmplification(amp_type) => {
                format!("{} Amplification", amp_type.to_string())
            }
            Attribute::WeaponStat(weapon_hand, weapon_stat) => {
                format!("{}{}", weapon_hand.to_string(), weapon_stat.to_string())
            }
            Attribute::Flag(flag) => flag.to_string(),
        }
    }
}

impl Attribute {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            Attribute::AbilityScore(ability) => Some(ability.get_score_bonuses(value)),
            Attribute::AbilityModifier(ability) => Some(ability.get_modifier_bonuses(value)),
            Attribute::Skill(skill) => skill.get_attribute_bonuses(value),
            Attribute::SpellPower(SpellPower::Universal) => {
                Some(spell_power_universal_to_others!(SpellPower, value))
            }
            Attribute::SpellCriticalChance(SpellPower::Universal) => {
                Some(spell_power_universal_to_others!(SpellCriticalChance, value))
            }
            Attribute::SpellCriticalDamage(SpellPower::Universal) => {
                Some(spell_power_universal_to_others!(SpellCriticalDamage, value))
            }
            Attribute::SavingThrow(saving_throw) => saving_throw.get_attribute_bonuses(value),
            _ => None,
        }
    }

    pub fn get_clone_attributes(&self) -> Option<Vec<Attribute>> {
        match self {
            Attribute::WeaponStat(WeaponHand::Both, weapon_stat) => Some(vec![
                Attribute::WeaponStat(WeaponHand::MainHand, *weapon_stat),
                Attribute::WeaponStat(WeaponHand::OffHand, *weapon_stat),
            ]),
            Attribute::SpellPower(SpellPower::Potency) => Some(
                POTENCY_CLONED_ATTRIBUTES
                    .map(|spell_power| Attribute::SpellPower(spell_power))
                    .to_vec(),
            ),
            _ => None,
        }
    }
}

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        BonusSource::Attribute(value)
    }
}
