use crate::{
    attribute::{
        flags::Flag,
        toggles::{AttackingTarget, Toggle},
        types::{
            Ability, ArmorClass, EnergyResistance, Immunity, MonsterType, SavingThrow, Skill,
            WeaponHand, WeaponStat,
        },
        Attribute, GetBonuses,
    },
    bonus::{Bonus, BonusType, Condition},
    feat::{Feat, Proficiency},
    item::types::WeaponType,
    race::Race,
};

use super::RacialFeat;

impl Race {
    fn ability_modifier(&self, ability: Ability, value: f32) -> Bonus {
        Bonus::new(
            Attribute::Ability(ability),
            BonusType::Stacking,
            value.into(),
            Attribute::from(*self).into(),
            None,
        )
    }

    fn bonus_feat<T>(&self, feat: T) -> Bonus
    where
        Feat: From<T>,
    {
        Bonus::new(
            Feat::from(feat).into(),
            BonusType::Stacking,
            1f32.into(),
            Attribute::from(*self).into(),
            None,
        )
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            Race::Aasimar => Some(vec![
                self.ability_modifier(Ability::Wisdom, 2f32),
                Bonus::new(
                    Skill::Heal.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Cold),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Acid),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Electric),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Race::Aasimar).into(),
                    None,
                ),
            ]),
            Race::Scourge => Some(vec![self.ability_modifier(Ability::Wisdom, 2f32)]),
            Race::Bladeforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Dexterity, -2f32),
                self.ability_modifier(Ability::Wisdom, -2f32),
            ]),
            Race::DeepGnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Wisdom, 2f32),
                self.ability_modifier(Ability::Strength, -2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(RacialFeat::RacialSpellResistance),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Race::Dragonborn => Some(vec![
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Charisma, 2f32),
                self.ability_modifier(Ability::Dexterity, -2f32),
            ]),
            Race::Drow => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Charisma, 2f32),
                self.ability_modifier(Ability::Constitution, -2f32),
                self.bonus_feat(RacialFeat::RacialSpellResistance),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(Proficiency::from(WeaponType::Shuriken)),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::ShortSword)),
                // TODO: Shuriken Expertise
            ]),
            Race::Dwarf => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
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
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Constitution, -2f32),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
                self.bonus_feat(Proficiency::from(WeaponType::LongBow)),
                self.bonus_feat(Proficiency::from(WeaponType::ShortBow)),
            ]),
            Race::Gnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Race::Halfling => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Strength, -2f32),
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
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Race::Human => None,
            Race::Morninglord => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Constitution, -2f32),
            ]),
            Race::PurpleDragonKnight => None,
            Race::Razorclaw => Some(vec![
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
            ]),
            Race::Shadarkai => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Race::Shifter => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
            ]),
            Race::Tabaxi => Some(vec![self.ability_modifier(Ability::Dexterity, 2f32)]),
            Race::Trailblazer => Some(vec![self.ability_modifier(Ability::Dexterity, 2f32)]),
            Race::Tiefling => Some(vec![
                self.ability_modifier(Ability::Charisma, 2f32),
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
            Race::Scoundrel => Some(vec![self.ability_modifier(Ability::Charisma, 2f32)]),
            Race::Warforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Wisdom, -2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Race::WoodElf => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
                self.bonus_feat(Proficiency::from(WeaponType::ShortBow)),
            ]),
        })?
    }
}

impl From<Race> for Attribute {
    fn from(value: Race) -> Self {
        Flag::from(value).into()
    }
}
