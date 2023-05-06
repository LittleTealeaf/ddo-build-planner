use crate::{attributes, no_children};

use self::sub::Ability;

use super::bonus::{types::BonusType, Bonus};

pub(crate) mod macros;
mod sub;
pub use sub::*;

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
        |value, source| match ability {
            Ability::Strength=> vec![
                Bonus::new(Self::Skill(Skill::Jump), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Swim), BonusType::AbilityModifier, value, source, None),
            ],
            Ability::Dexterity => vec![
                Bonus::new(Self::Skill(Skill::Balance), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Hide), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::MoveSilently), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::OpenLock), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Tumble), BonusType::AbilityModifier, value, source, None),
            ],
            Ability::Constitution => vec![
                Bonus::new(Self::Skill(Skill::Concentration), BonusType::AbilityModifier, value, source, None),
            ],
            Ability::Wisdom => vec![
                Bonus::new(Self::Skill(Skill::Heal), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Listen), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Spot), BonusType::AbilityModifier, value, source, None),
            ],
            Ability::Intelligence => vec![
                Bonus::new(Self::Skill(Skill::DisableDevice), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Repair), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Search), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::SpellCraft), BonusType::AbilityModifier, value, source, None),
            ],
            Ability::Charisma => vec![
                Bonus::new(Self::Skill(Skill::Bluff), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Diplomacy), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Haggle), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Intimidate), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::Perform), BonusType::AbilityModifier, value, source, None),
                Bonus::new(Self::Skill(Skill::UseMagicalDevice), BonusType::AbilityModifier, value, source, None),
            ],
        }
    ),
    Skill(skill: Skill) => (
        skill.to_string(),
        no_children!()
    ),
    Flag(flag: Flag) => (
        flag.to_string(),
        no_children!()
    ),
    ClassLore(class_lore: ClassLore) => (
        format!("{} Lore", class_lore.to_string()),
        no_children!()
    ),
    SpellFocus(school: SpellSchool) => (
        format!("Spell Focus: {}", school.to_string()),
        no_children!()
    ),
    SpellPower(spell_type: SpellDamageType) => (
        format!("{} Spell Power", spell_type.to_string()),
        no_children!()
    ),
    SpellCriticalChance(spell_type: SpellDamageType) => (
        format!("{} Spell Critical Chance", spell_type.to_string()),
        no_children!()
    ),
    SpellCriticalDamage(spell_type: SpellDamageType) => (
        format!("{} Spell Critical Damage", spell_type.to_string()),
        no_children!()
    ),
    SavingThrow(saving_throw: SavingThrow) => (
        format!("{} Saving Throw", saving_throw.to_string()),
        no_children!()
    )
);
