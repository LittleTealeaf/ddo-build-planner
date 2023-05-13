use crate::{
    attributes,
    bonus::{Bonus, BonusType},
};

use super::{bonus::BonusSource, feat::Feat};

mod macros;
mod sub;
pub use macros::*;
use serde::{Deserialize, Serialize};
pub use sub::*;

attributes!(
    Attribute,
    val,
    Dummy() => (
        String::from("Dummy"),
        None,
        None
    )
    Feat(feat: Feat) => (
        feat.to_string(),
        feat.get_attribute_bonuses(val),
        None
    )
    Flag(flag: Flag) => (
        flag.to_string(),
        flag.get_attribute_bonuses(val),
        None
    )
    Toggle(toggle: Toggle) => (
        toggle.to_string(),
        None,
        None
    )
    Ability(ability: Ability) => (
        ability.to_string(),
        Some(vec![Bonus::new(Attribute::AbilityModifier(*ability), BonusType::Stacking, ((val - 10f32) / 2f32).floor(), BonusSource::Attribute(Attribute::Ability(*ability)), None)]),
        Some(ability.get_cloned_abilities()?.into_iter().map(Attribute::Ability).collect())
    )
    AbilityModifier(ability: Ability) => (
        format!("{} Modifier", ability.to_string()),
        ability.get_modifier_bonuses(val),
        Some(ability.get_cloned_abilities()?.into_iter().map(Attribute::AbilityModifier).collect())
    )
    Skill(skill: Skill) => (
        skill.to_string(),
        skill.get_attribute_bonuses(val),
        Some(skill.get_cloned_skills()?.into_iter().map(Attribute::Skill).collect())
    )
    SavingThrow(savingthrow: SavingThrow) => (
        savingthrow.to_string(),
        savingthrow.get_attribute_bonuses(val),
        Some(savingthrow.get_cloned_values()?.into_iter().map(Attribute::SavingThrow).collect())
    )
    SpellPower(spellpower: SpellPower) => (
        format!("{} Spell Power", spellpower.to_string()),
        None,
        Some(spellpower.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellPower).collect())
    )
    SpellCriticalChance(spellpower: SpellPower) => (
        format!("{} Spell Critical Chance", spellpower.to_string()),
        None,
        Some(spellpower.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellCriticalChance).collect())
    )
    SpellCriticalDamage(spellpower: SpellPower) => (
        format!("{} Spell Critical Damage", spellpower.to_string()),
        None,
        Some(spellpower.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellCriticalDamage).collect())
    )
    PhysicalSheltering() => (
        String::from("Physical Sheltering"),
        None,
        None
    )
    MagicalSheltering() => (
        String::from("Magical Sheltering"),
        None,
        None
    )
    Sheltering() => (
        String::from("Sheltering"),
        None,
        Some(vec![Attribute::PhysicalSheltering(), Attribute::MagicalSheltering()])
    )
    WeaponStat(weaponhand: WeaponHand, weaponstat: WeaponStat) => (
        weaponstat.custom_to_string(weaponhand),
        None,
        weaponstat.get_cloned_attributes(weaponhand)
    )
    OffHandAttackChance() => (String::from("Off Hand Attack Chance"), None, None)
    Doublestrike() => ( String::from("Doublestrike"), None, None)
    Doubleshot() => (String::from("Doubleshot"), None, None)
    ImbueDice() => (String::from("Imbue Dice"), None, None)
    SneakAttackDice() => (String::from("Sneak Attack Dice"), None, None)
    SneakAttackDamage() => (String::from("Sneak Attack Bonus"), None, None)
    MeleePower() => (String::from("Melee Power"), None, None)
    RangedPower() => (String::from("Ranged Power"), None, None)
    SecondaryShieldBash() => (String::from("Secondary Shield Bash Chance"), None, None)
    DodgeBypass() => (String::from("Dodge Bypass"), None, None)
    FortificationBypass() => (String::from("Fortification Bypass"), None, None)
    MissileDeflection() => (String::from("Missile Deflection"), None, None)
    MissileDeflectionBypass() => (String::from("Missile Deflection Bypass"), None, None)
    Strikethrough() => (String::from("Strikethrough"), None, None)
    HelplessDamage() => (String::from("Helpless Damage"), None, None)
    ThreatGeneration(threat_type: ThreatType) => (
        format!("{} Threat Generation", threat_type.to_string()), 
        None, 
        Some(threat_type.get_cloned_types()?.into_iter().map(Attribute::ThreatGeneration).collect())
    )
    ThreatReduction(threat_type: ThreatType) => (
        format!("{} Threat Reduction", threat_type.to_string()), 
        None, 
        Some(threat_type.get_cloned_types()?.into_iter().map(Attribute::ThreatGeneration).collect())
    )
);

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        BonusSource::Attribute(value)
    }
}
