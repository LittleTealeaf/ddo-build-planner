use crate::build::{
    attribute::{ability::Ability, flag::Flag, saving_throw::SavingThrow, skill::Skill, Attribute},
    bonus::{condition::Condition, source::Source, types::BonusType, Bonus},
};

pub fn get_ability_updates(ability: Ability, value: f32) -> Vec<Bonus> {
    vec![Bonus::new(
        Attribute::AbilityModifier(ability),
        BonusType::AbilityScore,
        (value - 10f32) / 2f32,
        Source::Attribute(Attribute::Ability(ability)),
        None,
    )]
}

macro_rules! mod_bonus {
    ($ability:ident, $attribute:expr, $value:ident) => {
        Bonus::new(
            $attribute,
            BonusType::AbilityModifier,
            $value,
            Source::Attribute(Attribute::AbilityModifier(Ability::$ability)),
            None,
        )
    };
}

macro_rules! mod_skill {
    ($ability: ident, $skill: ident, $value: ident) => {
        mod_bonus!($ability, Attribute::Skill(Skill::$skill), $value)
    };
}

pub fn get_ability_modifier_updates(ability: Ability, value: f32) -> Vec<Bonus> {
    let mut bonuses = match ability {
        Ability::Strength => vec![
            mod_skill!(Strength, Jump, value),
            mod_skill!(Strength, Swim, value),
        ],
        Ability::Dexterity => vec![
            mod_skill!(Dexterity, Balance, value),
            mod_skill!(Dexterity, Hide, value),
            mod_skill!(Dexterity, MoveSilently, value),
            mod_skill!(Dexterity, OpenLock, value),
            mod_skill!(Dexterity, Tumble, value),
            mod_bonus!(
                Dexterity,
                Attribute::SavingThrow(SavingThrow::Reflex),
                value
            ),
        ],
        Ability::Constitution => vec![
            mod_skill!(Constitution, Concentration, value),
            mod_bonus!(
                Constitution,
                Attribute::SavingThrow(SavingThrow::Fortitude),
                value
            ),
        ],
        Ability::Intelligence => vec![
            mod_skill!(Intelligence, DisableDevice, value),
            mod_skill!(Intelligence, Repair, value),
            mod_skill!(Intelligence, Search, value),
            mod_skill!(Intelligence, Spellcraft, value),
        ],
        Ability::Wisdom => vec![
            mod_skill!(Wisdom, Heal, value),
            mod_skill!(Wisdom, Listen, value),
            mod_skill!(Wisdom, Spot, value),
            mod_bonus!(Wisdom, Attribute::SavingThrow(SavingThrow::Will), value),
        ],
        Ability::Charisma => vec![
            mod_skill!(Charisma, Bluff, value),
            mod_skill!(Charisma, Diplomacy, value),
            mod_skill!(Charisma, Haggle, value),
            mod_skill!(Charisma, Intimidate, value),
            mod_skill!(Charisma, Perform, value),
            mod_skill!(Charisma, UseMagicalDevice, value),
        ],
    };

    bonuses.push(Bonus::new(
        Attribute::Attack,
        BonusType::AbilityModifier,
        value,
        Source::Attribute(Attribute::AbilityModifier(ability)),
        Some(Condition::Flag(Flag::AbilityForAttack(ability)).into_vec()),
    ));

    bonuses.push(Bonus::new(
        Attribute::Damage,
        BonusType::AbilityModifier,
        value,
        Source::Attribute(Attribute::AbilityModifier(ability)),
        Some(Condition::Flag(Flag::AbilityForDamage(ability)).into_vec()),
    ));

    bonuses
}

#[cfg(test)]
mod tests {
    use crate::build::{
        attribute::{ability::Ability, skill::Skill, Attribute},
        bonus::{source::Source, types::BonusType, Bonus},
        breakdowns::Breakdowns,
    };

    #[test]
    fn test_skills() {
        let mut breakdowns = Breakdowns::new();
        let abilities = [
            Ability::Strength,
            Ability::Dexterity,
            Ability::Constitution,
            Ability::Wisdom,
            Ability::Intelligence,
            Ability::Charisma,
        ];

        for ability in abilities {
            breakdowns.insert_attributes(
                Bonus::new(
                    Attribute::Ability(ability),
                    BonusType::Stacking,
                    20.0,
                    Source::Unique(0),
                    None,
                )
                .into_vec(),
            );
        }

        for skill in [
            Skill::Heal,
            Skill::Listen,
            Skill::Spot,
            Skill::Jump,
            Skill::Swim,
            Skill::DisableDevice,
            Skill::Repair,
            Skill::Search,
            Skill::Spellcraft,
            Skill::Balance,
            Skill::Hide,
            Skill::MoveSilently,
            Skill::OpenLock,
            Skill::Tumble,
            Skill::Concentration,
            Skill::Bluff,
            Skill::Diplomacy,
            Skill::Haggle,
            Skill::Intimidate,
            Skill::Perform,
            Skill::UseMagicalDevice,
        ] {
            assert!(breakdowns.get_attribute(&Attribute::Skill(skill)) > 0.0);
        }
    }
}
