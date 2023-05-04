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
    ($ability:expr, $attribute:expr, $value:expr) => {
        Bonus::new(
            $attribute,
            BonusType::AbilityModifier,
            $value,
            Source::Attribute(Attribute::AbilityModifier($ability)),
            None,
        )
    };
}

macro_rules! mod_attr {
    ($attribute:expr) => {
        Bonus::new(
            $attribute,
            BonusType::AbilityModifier,
            value,
            Source::Attribute(Attribute::AbilityModifier(ability)),
            None,
        )
    };
}

pub fn get_ability_modifier_updates(ability: Ability, value: f32) -> Vec<Bonus> {
    let mut bonuses = match ability {
        Ability::Strength => get_strength_modifier_bonuses(value),
        Ability::Dexterity => get_dexterity_modifier_bonuses(value),
        Ability::Constitution => get_constitution_modifier_bonuses(value),
        Ability::Intelligence => get_intelligence_modifier_bonuses(value),
        Ability::Wisdom => get_wisdom_modifier_bonuses(value),
        Ability::Charisma => get_charisma_modifier_bonuses(value),
    };

    bonuses.push(Bonus::new(
        Attribute::Flag(Flag::AbilityForAttack(ability)),
        BonusType::AbilityModifier,
        value,
        Source::Attribute(Attribute::AbilityModifier(ability)),
        Some(Condition::HasFlag(Flag::AbilityForAttack(ability)).into_vec()),
    ));

    bonuses.push(Bonus::new(
        Attribute::Flag(Flag::AbilityForDamage(ability)),
        BonusType::AbilityModifier,
        value,
        Source::Attribute(Attribute::AbilityModifier(ability)),
        Some(Condition::HasFlag(Flag::AbilityForDamage(ability)).into_vec()),
    ));

    bonuses
}

fn get_strength_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(Ability::Strength, Attribute::Skill(Skill::Jump), value),
        mod_bonus!(Ability::Strength, Attribute::Skill(Skill::Swim), value),
    ]
}

fn get_dexterity_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(Ability::Dexterity, Attribute::Skill(Skill::Balance), value),
        mod_bonus!(Ability::Dexterity, Attribute::Skill(Skill::Hide), value),
        mod_bonus!(
            Ability::Dexterity,
            Attribute::Skill(Skill::MoveSilently),
            value
        ),
        mod_bonus!(Ability::Dexterity, Attribute::Skill(Skill::OpenLock), value),
        mod_bonus!(Ability::Dexterity, Attribute::Skill(Skill::Tumble), value),
    ]
}

fn get_constitution_modifier_bonuses(value: f32) -> Vec<Bonus> {
    mod_bonus!(
        Ability::Dexterity,
        Attribute::Skill(Skill::Concentration),
        value
    )
    .into_vec()
}

fn get_intelligence_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(
            Ability::Intelligence,
            Attribute::Skill(Skill::DisableDevice),
            value
        ),
        mod_bonus!(
            Ability::Intelligence,
            Attribute::Skill(Skill::Repair),
            value
        ),
        mod_bonus!(
            Ability::Intelligence,
            Attribute::Skill(Skill::Search),
            value
        ),
        mod_bonus!(
            Ability::Intelligence,
            Attribute::Skill(Skill::Spellcraft),
            value
        ),
    ]
}

fn get_wisdom_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Heal), value),
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Listen), value),
        mod_bonus!(Ability::Wisdom, Attribute::Skill(Skill::Spot), value),
        mod_bonus!(
            Ability::Wisdom,
            Attribute::SavingThrow(SavingThrow::Will),
            value
        ),
    ]
}

fn get_charisma_modifier_bonuses(value: f32) -> Vec<Bonus> {
    vec![
        mod_bonus!(Ability::Charisma, Attribute::Skill(Skill::Bluff), value),
        mod_bonus!(Ability::Charisma, Attribute::Skill(Skill::Diplomacy), value),
        mod_bonus!(Ability::Charisma, Attribute::Skill(Skill::Haggle), value),
        mod_bonus!(
            Ability::Charisma,
            Attribute::Skill(Skill::Intimidate),
            value
        ),
        mod_bonus!(Ability::Charisma, Attribute::Skill(Skill::Perform), value),
        mod_bonus!(
            Ability::Charisma,
            Attribute::Skill(Skill::UseMagicalDevice),
            value
        ),
    ]
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
