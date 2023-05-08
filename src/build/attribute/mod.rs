use crate::attributes;

use super::bonus::{Source, BonusType, Bonus};

pub(crate) mod macros;
mod sub;
pub use sub::*;

fn no_children(_: f32, _: Source) -> Vec<Bonus> {
    Vec::new()
}

attributes!(
    Attribute,
    Ability(ability: Ability) => (
        ability.to_string(),
        |value: f32, source| vec![
            Bonus::new(Self::AbilityModifier(*ability), BonusType::AbilityModifier, ((value - 10f32) / 2f32).floor(), source, None)
        ]
    ),
    AbilityModifier(ability: Ability) => (
        format!("{} Modifier", ability.to_string()),
        |value, source|  ability.get_modifier_bonuses(value, source)   ),
    Skill(skill: Skill) => (
        skill.to_string(),
        |value, source| skill.get_bonuses(value, source)
    ),
    Flag(flag: Flag) => (
        flag.to_string(),
        no_children
    ),
    ClassLore(class_lore: ClassLore) => (
        format!("{} Lore", class_lore.to_string()),
        no_children
    ),
    SpellFocus(school: SpellSchool) => (
        format!("Spell Focus: {}", school.to_string()),
        no_children
    ),
    SpellPower(spell_type: SpellDamageType) => (
        format!("{} Spell Power", spell_type.to_string()),
        no_children
    ),
    SpellCriticalChance(spell_type: SpellDamageType) => (
        format!("{} Spell Critical Chance", spell_type.to_string()),
        no_children
    ),
    SpellCriticalDamage(spell_type: SpellDamageType) => (
        format!("{} Spell Critical Damage", spell_type.to_string()),
        no_children
    ),
    SavingThrow(saving_throw: SavingThrow) => (
        format!("{} Saving Throw", saving_throw.to_string()),
        no_children
    ),
    ElementalResistance(element: ElementalType) => (
        format!("{} Resistance", element.to_string()),
        no_children
    ),
    ElementalAbsorption(element: ElementalType) => (
        format!("{} Resistance", element.to_string()),
        no_children
    ),
    Attack() => (
        String::from("Attack"),
        no_children
    ),
    Damage() => (
        String::from("Damage"),
        no_children
    ),
    Toggle(toggle: Toggle) => (
        format!("Toggle: {}", toggle.to_string()),
        no_children
    )
);
