use crate::{
    attribute_subtype,
    build::{
        attribute::Attribute,
        bonus::{condition::Condition, source::Source, types::BonusType, Bonus},
    },
};

use super::{Flag, SavingThrow, Skill};

attribute_subtype!(Ability, (Strength "Strength"), (Dexterity "Dexterity"), (Constitution "Constitution"), (Wisdom "Wisdom"), (Intelligence "Intelligence"), (Charisma "Charisma"));

impl Ability {
    pub fn get_modifier_bonuses(&self, value: f32, source: Source) -> Vec<Bonus> {
        let mut bonuses = match self {
            Ability::Strength => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Jump),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Swim),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
            ],
            Ability::Dexterity => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Balance),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Hide),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::MoveSilently),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::OpenLock),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Tumble),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
            ],
            Ability::Constitution => vec![Bonus::new(
                Attribute::Skill(Skill::Concentration),
                BonusType::AbilityModifier,
                value,
                source,
                None,
            )],
            Ability::Wisdom => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Heal),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Listen),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Spot),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
            ],
            Ability::Intelligence => vec![
                Bonus::new(
                    Attribute::Skill(Skill::DisableDevice),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Repair),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Search),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::SpellCraft),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
            ],
            Ability::Charisma => vec![
                Bonus::new(
                    Attribute::Skill(Skill::Bluff),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Diplomacy),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Haggle),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Intimidate),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::Perform),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
                Bonus::new(
                    Attribute::Skill(Skill::UseMagicalDevice),
                    BonusType::AbilityModifier,
                    value,
                    source,
                    None,
                ),
            ],
        };

        bonuses.push(Bonus::new(
            Attribute::Attack(),
            BonusType::AbilityModifier,
            value,
            source,
            Some(vec![Condition::Flag(Flag::AbilityToAttack(*self))]),
        ));
        bonuses.push(Bonus::new(
            Attribute::Damage(),
            BonusType::AbilityModifier,
            value,
            source,
            Some(vec![Condition::Flag(Flag::AbilityToDamage(*self))]),
        ));
        for saving_throw in [
            SavingThrow::Reflex,
            SavingThrow::Fortitude,
            SavingThrow::Will,
        ] {
            bonuses.push(Bonus::new(
                Attribute::SavingThrow(saving_throw),
                BonusType::AbilityModifier,
                value,
                source,
                Some(vec![Condition::Flag(Flag::AbilityToSavingThrow(
                    *self,
                    saving_throw,
                ))]),
            ));
        }

        bonuses
    }
}
