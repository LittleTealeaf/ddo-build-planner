#![allow(unused_variables)]
use crate::{
    bonus::{Bonus, BonusType},
    player_class::PlayerClass,
};

use super::{bonus::BonusSource, feat::Feat};

#[macro_use]
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
        format!("Flag: {}", flag.to_string()),
        flag.get_attribute_bonuses(val),
        None
    )
    Toggle(toggle: Toggle) => (
        format!("Toggle: {}", toggle.to_string()),
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
    SpellPower(spell_power: SpellPower) => (
        format!("{} Spell Power", spell_power.to_string()),
        None,
        Some(spell_power.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellPower).collect())
    )
    SpellCriticalChance(spell_power: SpellPower) => (
        format!("{} Spell Critical Chance", spell_power.to_string()),
        None,
        Some(spell_power.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellCriticalChance).collect())
    )
    SpellCriticalDamage(spell_power: SpellPower) => (
        format!("{} Spell Critical Damage", spell_power.to_string()),
        None,
        Some(spell_power.get_cloned_spellpowers()?.into_iter().map(Attribute::SpellCriticalDamage).collect())
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
    MagicalShelteringCap() => (String::from("Magical Sheltering Cap"), None, None)
    Sheltering() => (
        String::from("Sheltering"),
        None,
        Some(vec![Attribute::PhysicalSheltering(), Attribute::MagicalSheltering()])
    )
    WeaponStat(weapon_hand: WeaponHand, weapon_stat: WeaponStat) => (
        weapon_stat.custom_to_string(weapon_hand),
        None,
        weapon_stat.get_cloned_attributes(weapon_hand)
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
    Fortification() => (String::from("Fortification"), None, None)
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
    ElementalResistance(element: ElementalType) => (
        format!("{} Resistance", element.to_string()),
        None,
        None
    )
    ElementalAbsorption(element: ElementalType) => (
        format!("{} Absorption", element.to_string()),
        None,
        None
    )
    SpellFocus(spellschool: SpellSchool) => (
        format!("{} Spell Focus", spellschool.to_string()),
        None,
        Some(spellschool.get_cloned_schools()?.into_iter().map(Attribute::SpellFocus).collect())
    )
    SpellPoints(spellpoint: SpellPoint) => (spellpoint.to_string(), None, None)
    HitPoints(hitpoint: HitPoint) => (hitpoint.to_string(), None, None)
    Vitality() => (
        String::from("Vitality"),
        Some(vec![Bonus::new(Attribute::HitPoints(HitPoint::Bonus), BonusType::Stacking, val, BonusSource::Attribute(Attribute::Vitality()), None)]),
        None
    )
    UnconsciousRange() => (String::from("Unconscious Range"), None, None)
    HealAmp(healamp: HealAmp) => (format!("{} Amplification", healamp.to_string()), None, Some(healamp.get_cloned_attributes()?.into_iter().map(Attribute::HealAmp).collect()))
    ClassLore(lore: ClassLore) => (format!("{} Lore", lore.to_string()), lore.get_attribute_bonuses(val), None)
    ClassLevel(player_class: PlayerClass) => (format!("{} Levels", player_class.to_string()), player_class.get_attribute_bonuses(val), None)
    MovementSpeed() => (String::from("Movement Speed"), None, None)
    PactDice() => (String::from("Pact Dice"), None, None)
    EldritchBlastDice() => (String::from("Eldritch Blast Dice"), None, None)
    SpellCostReduction() => (String::from("Spell Cost Reduction"), None, None)
    SpellResistance() => (String::from("Spell Resistance"), None, None)
    SpellPenetration() => (String::from("Spell Penetation"), None, None)
    NaturalArmor() => (String::from("Natural Armor"), None, None)
    FiligreeSet(set: FiligreeSet) => (set.to_string(), set.get_attribute_bonuses(val), None)
    Dodge() => (String::from("Dodge"), None, None)
    MaxDodge() => (String::from("Maximum Dodge"), None, None)
    Tactics(tactics: Tactics) => (format!("{} DC", tactics.to_string()), None, None)
    BonusActionBoosts() => (String::from("Bonus Action Boosts"), None, None)
    CasterLevel(casterlevel: CasterLevel) => (casterlevel.to_string(), None, Some(casterlevel.get_cloned_attributes()?.into_iter().map(Attribute::CasterLevel).collect()))
);

impl From<Attribute> for BonusSource {
    fn from(value: Attribute) -> Self {
        BonusSource::Attribute(value)
    }
}
