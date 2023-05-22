#![allow(unused_variables)]
use crate::{
    bonus::{Bonus, BonusType},
    player_class::PlayerClass,
    utils::AsString,
};

use super::{bonus::BonusSource, feat::Feat};

#[macro_use]
mod macros;
/// Contains lists of sub-attributes that allow attributes to be generally expressed.
pub mod sub;
pub use macros::*;
use serde::{Deserialize, Serialize};
use sub::*;
mod traits;
pub use traits::*;

attributes!(
    Attribute,
    val,
    Dummy() => (
        String::from("Dummy"),
        "A dummy attrirbute.\n\nOften used to indicate that a data source should be removed from the Brerakdowns.",
        None,
        None
    )
    Feat(feat: Feat) => (
        feat.to_string(),
        "Indicates that the character has a certain feat, and should receive its bonuses.",
        feat.get_attribute_bonuses(val),
        None
    )
    Flag(flag: Flag) => (
        format!("Flag: {}", flag.to_string()),
        "Represents any flags that the character has.",
        flag.get_bonuses(val),
        Some(flag.get_cloned()?.into_iter().map(Attribute::Flag).collect())
    )
    Toggle(toggle: Toggle) => (
        format!("Toggle: {}", toggle.to_string()),
        "Represents any toggles that should be visible to the user.",
        None,
        None
    )
    Ability(ability: Ability) => (
        ability.to_string(),
        "The score for each of the character's 6 abilities.",
        GetBonuses::<_AbilityScore>::get_bonuses(ability, val),
        Some(ability.get_cloned()?.into_iter().map(Attribute::Ability).collect())
    )
    AbilityModifier(ability: Ability) => (
        format!("{} Modifier", ability.to_string()),
        "The modifier, derived from the Ability Score, for each of the 6 abilities.",
        GetBonuses::<_AbilityModifier>::get_bonuses(ability, val),
        Some(ability.get_cloned()?.into_iter().map(Attribute::AbilityModifier).collect())
    )
    Skill(skill: Skill) => (
        skill.to_string(),
        "Skills that provide additional attributes and abilities for the character.",
        skill.get_bonuses(val),
        Some(skill.get_cloned()?.into_iter().map(Attribute::Skill).collect())
    )
    SavingThrow(savingthrow: SavingThrow) => (
        savingthrow.to_string(),
        "Represents the three main saving throws: [Reflex](SavingThrow::Reflex) ([Dexterity](Ability::Dexterity)), [Fortitude](SavingThrow::Fortitude) ([Constitution](Ability::Constitution)), and [Will](SavingThrow::Will) ([Wisdom](Ability::Wisdom)). Also represents additional specific saving throws.",
        None,
        Some(savingthrow.get_cloned()?.into_iter().map(Attribute::SavingThrow).collect())
    )
    SpellPower(spell_power: SpellPower) => (
        format!("{} Spell Power", spell_power.to_string()),
        "For each point in a spell power, spells of that type gain 1% more damage. [SpellPower::All] will automatically split off to other spell powers.",
        None,
        Some(spell_power.get_cloned()?.into_iter().map(Attribute::SpellPower).collect())
    )
    SpellCriticalChance(spell_power: SpellPower) => (
        format!("{} Spell Critical Chance", spell_power.to_string()),
        "The chance that spells of a given type will critically hit. [SpellPower::All] will automatically split off to other spell powers.",
        None,
        Some(spell_power.get_cloned()?.into_iter().map(Attribute::SpellCriticalChance).collect())
    )
    SpellCriticalDamage(spell_power: SpellPower) => (
        format!("{} Spell Critical Damage", spell_power.to_string()),
        "The % bonus damage that critical hits deal with spells of a certain type. [SpellPower::All] will automatically split off to other spell powers.",
        None,
        Some(spell_power.get_cloned()?.into_iter().map(Attribute::SpellCriticalDamage).collect())
    )
    PhysicalSheltering() => (
        String::from("Physical Sheltering"),
        "Physical Resistance Rating, which decreases the amount of physical damage taken.",
        None,
        None
    )
    MagicalSheltering() => (
        String::from("Magical Sheltering"),
        "Magical Resistance Rating, which decreases the amount of magical damage taken",
        None,
        None
    )
    MagicalShelteringCap() => (
        String::from("Magical Sheltering Cap"),
        "Magical Resistance Rating Cap, which increases the maximum that you Magical Resistance Rating can be when wearing light or cloth armor",
         None,
         None
    )
    Sheltering() => (
        String::from("Sheltering"),
        "Adds bonuses to both [Magical Sheltering](Attribute::MagicalSheltering) and [Physical Sheltering](Self::PhysicalSheltering)",
        None,
        Some(vec![Attribute::PhysicalSheltering(), Attribute::MagicalSheltering()])
    )
    WeaponStat(weapon_hand: WeaponHand, weapon_stat: WeaponStat) => (
        (weapon_hand, weapon_stat).as_string(),
        "Any specific stats that might only pertain to a specific weapon. Using [WeaponHand::Both] can be used for overall bonuses",
        None,
        Some((*weapon_hand, *weapon_stat).get_cloned()?.into_iter().map(Attribute::from).collect())
    )
    OffHandAttackChance() => (
        String::from("Off Hand Attack Chance"),
        "The chance that the off-hand weapon will roll to attack",
        None,
        None
    )
    Doublestrike() => (
        String::from("Doublestrike"),
        "Chance that melee attacks will gain a x2 multiplier to implement hitting twice",
        None,
        None
    )
    Doubleshot() => (
        String::from("Doubleshot"),
        "Chance that ranged attacks will gain a x2 multiplier to implement shooting twice",
        None,
        None
    )
    ImbueDice() => (
        String::from("Imbue Dice"),
        "Bonus dice to imbue damage",
        None,
        None
    )
    SneakAttackDice() => (
        String::from("Sneak Attack Dice"),
        "Bonus dice for Sneak Attacks",
        None,
        None
    )
    SneakAttackDamage() => (
        String::from("Sneak Attack Bonus"),
        "Bonus to attack for sneak attacks",
        None,
        None
    )
    MeleePower() => (
        String::from("Melee Power"),
        "Bonus to melee power",
        None,
        None
    )
    RangedPower() => (String::from("Ranged Power"), "", None, None)
    SecondaryShieldBash() => (String::from("Secondary Shield Bash Chance"), "", None, None)
    DodgeBypass() => (String::from("Dodge Bypass"), "", None, None)
    FortificationBypass() => (String::from("Fortification Bypass"), "", None, None)
    Fortification() => (String::from("Fortification"), "", None, None)
    MissileDeflection() => (String::from("Missile Deflection"), "", None, None)
    MissileDeflectionBypass() => (String::from("Missile Deflection Bypass"), "", None, None)
    Strikethrough() => (String::from("Strikethrough"), "", None, None)
    HelplessDamage() => (String::from("Helpless Damage"), "", None, None)
    ThreatGeneration(threat_type: ThreatType) => (
        format!("{} Threat Generation", threat_type.to_string()),
        "",
        None,
        Some(threat_type.get_cloned()?.into_iter().map(Attribute::ThreatGeneration).collect())
    )
    ThreatReduction(threat_type: ThreatType) => (
        format!("{} Threat Reduction", threat_type.to_string()),
        "",
        None,
        Some(threat_type.get_cloned()?.into_iter().map(Attribute::ThreatGeneration).collect())
    )
    ElementalResistance(element: ElementalType) => (
        format!("{} Resistance", element.to_string()),
        "",
        None,
        None
    )
    ElementalAbsorption(element: ElementalType) => (
        format!("{} Absorption", element.to_string()),
        "",
        None,
        None
    )
    SpellFocus(spellschool: SpellSchool) => (
        format!("{} Spell Focus", spellschool.to_string()),
        "",
        None,
        Some(spellschool.get_cloned()?.into_iter().map(Attribute::SpellFocus).collect())
    )
    SpellPoints(spellpoint: SpellPoint) => (spellpoint.to_string(), "", None, None)
    HitPoints(hitpoint: HitPoint) => (hitpoint.to_string(), "", None, None)
    Vitality() => (
        String::from("Vitality"),
        "",
        Some(vec![Bonus::new(Attribute::HitPoints(HitPoint::Bonus), BonusType::Stacking, val, BonusSource::Attribute(Attribute::Vitality()), None)]),
        None
    )
    UnconsciousRange() => (String::from("Unconscious Range"), "", None, None)
    HealAmp(healamp: HealAmp) => (format!("{} Amplification", healamp.to_string()), "", None, Some(healamp.get_cloned()?.into_iter().map(Attribute::HealAmp).collect()))
    ClassLore(lore: ClassLore) => (format!("{} Lore", lore.to_string()), "", lore.get_bonuses(val), None)
    ClassLevel(player_class: PlayerClass) => (format!("{} Levels", player_class.to_string()), "", player_class.get_bonuses(val), None)
    MovementSpeed() => (String::from("Movement Speed"), "", None, None)
    PactDice() => (String::from("Pact Dice"), "", None, None)
    EldritchBlastDice() => (String::from("Eldritch Blast Dice"), "", None, None)
    SpellCostReduction() => (String::from("Spell Cost Reduction"), "", None, None)
    SpellResistance() => (String::from("Spell Resistance"), "", None, None)
    SpellPenetration() => (String::from("Spell Penetation"), "", None, None)
    NaturalArmor() => (String::from("Natural Armor"), "", None, None)
    FiligreeSet(set: FiligreeSet) => (set.to_string(), "", set.get_bonuses(val), None)
    Dodge() => (String::from("Dodge"), "", None, None)
    MaxDodge() => (String::from("Maximum Dodge"), "", None, None)
    Tactics(tactics: Tactics) => (format!("{} DC", tactics.to_string()), "", None, None)
    BonusActionBoosts() => (String::from("Bonus Action Boosts"), "", None, None)
    CasterLevel(casterlevel: CasterLevel) => (
        casterlevel.to_string(),
        "",
        None,
        Some(casterlevel.get_cloned()?.into_iter().map(Attribute::CasterLevel).collect())
    )
    Immunity(immunity: Immunity) => (
        format!("Immunity from {}", immunity.to_string()),
        "Immunity from something",
        None,
        None
    )
);

impl Attribute {
    /// Converts any type that implements [`Into<Attribute>`] to a [`BonusSource`]
    pub fn to_source<T: Into<Attribute>>(source: T) -> BonusSource {
        BonusSource::Attribute(source.into())
    }
}

impl Default for Attribute {
    #[inline]
    fn default() -> Self {
        Self::Dummy()
    }
}

impl From<Attribute> for BonusSource {
    #[inline]
    fn from(value: Attribute) -> Self {
        BonusSource::Attribute(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_dummy() {
        assert_eq!(Attribute::Dummy(), Attribute::default());
    }
}
