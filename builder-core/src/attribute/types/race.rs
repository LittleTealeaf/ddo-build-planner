use crate::{
    attribute::{
        flags::Flag,
        toggles::{AttackingTarget, Toggle},
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusSource, BonusType, Condition},
    race::Race,
};

use super::{
    Ability, ArmorClass, Immunity, MonsterType, SavingThrow, Skill, WeaponHand, WeaponStat,
};

impl GetBonuses for Race {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            Race::Aasimar => Some(vec![ability_modifier(Race::Aasimar, Ability::Wisdom, 2f32)]),
            Race::Scourge => Some(vec![ability_modifier(Race::Scourge, Ability::Wisdom, 2f32)]),
            Race::Bladeforged => Some(vec![
                ability_modifier(Race::Bladeforged, Ability::Constitution, 2f32),
                ability_modifier(Race::Bladeforged, Ability::Dexterity, -2f32),
                ability_modifier(Race::Bladeforged, Ability::Wisdom, -2f32),
            ]),
            Race::DeepGnome => Some(vec![
                ability_modifier(Race::DeepGnome, Ability::Intelligence, 2f32),
                ability_modifier(Race::DeepGnome, Ability::Wisdom, 2f32),
                ability_modifier(Race::DeepGnome, Ability::Strength, -2f32),
                ability_modifier(Race::DeepGnome, Ability::Charisma, -2f32),
            ]),
            Race::Dragonborn => Some(vec![
                ability_modifier(Race::Dragonborn, Ability::Strength, 2f32),
                ability_modifier(Race::Dragonborn, Ability::Charisma, 2f32),
                ability_modifier(Race::Dragonborn, Ability::Dexterity, -2f32),
            ]),
            Race::Drow => Some(vec![
                ability_modifier(Race::Drow, Ability::Dexterity, 2f32),
                ability_modifier(Race::Drow, Ability::Intelligence, 2f32),
                ability_modifier(Race::Drow, Ability::Charisma, 2f32),
                ability_modifier(Race::Drow, Ability::Constitution, -2f32),
                Bonus::flag(Immunity::Sleep.into(), Attribute::from(Race::Drow).into()),
                Bonus::new(
                    SavingThrow::Enchantment.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Drow).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Drow).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Drow).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Drow).into(),
                    None,
                ),
                // TODO: Proficiency with Rapiers, Shortswords, Shurikens
                // TODO: +6 Spell Resistance
            ]),
            Race::Dwarf => Some(vec![
                ability_modifier(Race::Dwarf, Ability::Constitution, 2f32),
                ability_modifier(Race::Dwarf, Ability::Charisma, -2f32),
                // TODO: Proficiency with Dwarven Axe if proficiency with Martial Weapons
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Racial,
                    4f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Poison.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    None,
                ),
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc)).into(),
                    Attribute::from(Race::Dwarf).into(),
                ),
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid)).into(),
                    Attribute::from(Race::Dwarf).into(),
                ),
                Bonus::flag(
                    Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                    Attribute::from(Race::Dwarf).into(),
                ),
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Attack).into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    Some(Condition::Any(vec![
                        Condition::Has(
                            Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Orc))
                                .into(),
                        ),
                        Condition::Has(
                            Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Goblinoid))
                                .into(),
                        ),
                    ])),
                ),
                Bonus::new(
                    ArmorClass::Bonus.into(),
                    BonusType::Dodge,
                    4f32.into(),
                    Attribute::from(Race::Dwarf).into(),
                    Some(Condition::Has(
                        Toggle::Attacking(AttackingTarget::MonsterType(MonsterType::Giant)).into(),
                    )),
                ),
            ]),
            Race::Elf => Some(vec![
                ability_modifier(Race::Elf, Ability::Dexterity, 2f32),
                ability_modifier(Race::Elf, Ability::Constitution, -2f32),
                Bonus::flag(Immunity::Sleep.into(), Attribute::from(Race::Elf).into()),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Elf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Elf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Elf).into(),
                    None,
                ),
                // TODO: Proficiencies: longsword, rapier, longbow, composite longbow, shortbow,
                // composite shortbow
            ]),
            Race::Gnome => Some(vec![
                ability_modifier(Race::Gnome, Ability::Intelligence, 2f32),
                ability_modifier(Race::Gnome, Ability::Charisma, -2f32),
                Bonus::new(
                    ArmorClass::Bonus.into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Attack).into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Hide.into(),
                    BonusType::Size,
                    4f32.into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Intimidate.into(),
                    BonusType::Size,
                    (-4f32).into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Haggle.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                Bonus::new(
                    Skill::UseMagicalDevice.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Gnome).into(),
                    None,
                ),
                // TODO: Proficiencies: Light Hammer, Throwing Hammer, War Hammer
            ]),
            Race::Halfling => Some(vec![
                ability_modifier(Race::Halfling, Ability::Dexterity, 2f32),
                ability_modifier(Race::Halfling, Ability::Strength, -2f32),
                Bonus::new(
                    Skill::Intimidate.into(),
                    BonusType::Size,
                    (-4f32).into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    ArmorClass::Bonus.into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Attack).into(),
                    BonusType::Size,
                    1f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Hide.into(),
                    BonusType::Size,
                    4f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Jump.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    Skill::MoveSilently.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::All.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Fear.into(),
                    BonusType::Morale,
                    2f32.into(),
                    Attribute::from(Race::Halfling).into(),
                    None,
                ),
                // +1 to Attack with Thrown Weapons
            ]),
            Race::HalfElf => Some(vec![
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Race::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Race::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Race::HalfElf).into(),
                    None,
                ),
                Bonus::flag(
                    Immunity::Sleep.into(),
                    Attribute::from(Race::HalfElf).into(),
                ),
                Bonus::new(
                    Skill::Diplomacy.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::HalfElf).into(),
                    None,
                ),
            ]),
            Race::HalfOrc => Some(vec![
                ability_modifier(Race::HalfOrc, Ability::Strength, 2f32),
                ability_modifier(Race::HalfOrc, Ability::Intelligence, -2f32),
                ability_modifier(Race::HalfOrc, Ability::Charisma, -2f32),
            ]),
            Race::Human => None,
            Race::Morninglord => Some(vec![
                ability_modifier(Race::Morninglord, Ability::Intelligence, 2f32),
                ability_modifier(Race::Morninglord, Ability::Constitution, -2f32),
            ]),
            Race::PurpleDragonKnight => None,
            Race::Razorclaw => Some(vec![
                ability_modifier(Race::Razorclaw, Ability::Strength, 2f32),
                ability_modifier(Race::Razorclaw, Ability::Intelligence, -2f32),
            ]),
            Race::Shadarkai => Some(vec![
                ability_modifier(Race::Shadarkai, Ability::Dexterity, 2f32),
                ability_modifier(Race::Shadarkai, Ability::Charisma, -2f32),
            ]),
            Race::Shifter => Some(vec![
                ability_modifier(Race::Shifter, Ability::Dexterity, 2f32),
                ability_modifier(Race::Shifter, Ability::Intelligence, -2f32),
            ]),
            Race::Tabaxi => Some(vec![ability_modifier(
                Race::Tabaxi,
                Ability::Dexterity,
                2f32,
            )]),
            Race::Trailblazer => Some(vec![ability_modifier(
                Race::Trailblazer,
                Ability::Dexterity,
                2f32,
            )]),
            Race::Tiefling => Some(vec![
                ability_modifier(Race::Tiefling, Ability::Charisma, 2f32),
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Tiefling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Tiefling).into(),
                    None,
                ),
                // TODO: +2 to hit and damage against lawful outsiders and good outsiders
                // TODO: Fear Immunity
            ]),
            Race::Scoundrel => Some(vec![ability_modifier(
                Race::Scoundrel,
                Ability::Charisma,
                2f32,
            )]),
            Race::Warforged => Some(vec![
                ability_modifier(Race::Warforged, Ability::Constitution, 2f32),
                ability_modifier(Race::Warforged, Ability::Wisdom, -2f32),
                ability_modifier(Race::Warforged, Ability::Charisma, -2f32),
            ]),
            Race::WoodElf => Some(vec![
                ability_modifier(Race::WoodElf, Ability::Dexterity, 2f32),
                ability_modifier(Race::WoodElf, Ability::Intelligence, -2f32),
            ]),
        })?
    }
}
fn ability_modifier(race: Race, ability: Ability, value: f32) -> Bonus {
    Bonus::new(
        ability.into(),
        BonusType::Racial,
        value.into(),
        Attribute::from(Flag::from(race)).into(),
        None,
    )
}

impl From<Race> for Attribute {
    fn from(value: Race) -> Self {
        Flag::from(value).into()
    }
}
