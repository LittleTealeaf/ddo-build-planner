use crate::{
    attribute::{bonuses::EnergyResistance, Attribute, GetBonuses},
    bonus::{Bonus, BonusType, Condition},
    equipment::item::types::WeaponType,
    feat::{Feat, Proficiency},
    race::Race,
    types::{Ability, Immunity, SavingThrow, Skill},
};

use super::RacialFeat;

impl Race {
    fn ability_modifier(self, ability: Ability, value: f32) -> Bonus {
        Bonus::new(
            Attribute::Ability(ability),
            BonusType::Stacking,
            value.into(),
            Attribute::from(self).into(),
            None,
        )
    }

    fn bonus_feat<T>(self, feat: T) -> Bonus
    where
        Feat: From<T>,
    {
        Bonus::new(
            Feat::from(feat).into(),
            BonusType::Stacking,
            1f32.into(),
            Attribute::from(self).into(),
            None,
        )
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        (value > 0f32).then(|| match self {
            Self::Aasimar => Some(vec![
                self.ability_modifier(Ability::Wisdom, 2f32),
                Bonus::new(
                    Skill::Heal.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Cold),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Acid),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::EnergyResistance(EnergyResistance::Electric),
                    BonusType::Stacking,
                    5f32.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
            ]),
            Self::Scourge => Some(vec![self.ability_modifier(Ability::Wisdom, 2f32)]),
            Self::Bladeforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Dexterity, -2f32),
                self.ability_modifier(Ability::Wisdom, -2f32),
            ]),
            Self::DeepGnome => Some(vec![
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
            Self::Dragonborn => Some(vec![
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Charisma, 2f32),
                self.ability_modifier(Ability::Dexterity, -2f32),
            ]),
            Self::Drow => Some(vec![
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
            Self::Dwarf => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
                self.bonus_feat(RacialFeat::GiantEvasion),
                self.bonus_feat(RacialFeat::OrcAndGoblinBonus),
                self.bonus_feat(RacialFeat::PoisonSaveBonus),
                self.bonus_feat(RacialFeat::DwarvenStonecunning),
                self.bonus_feat(RacialFeat::DwarvenStability),
                self.bonus_feat(RacialFeat::SpellSaveBonus),
                Bonus::new(
                    Feat::from(Proficiency::from(WeaponType::DwarvenWarAxe)).into(),
                    BonusType::Stacking,
                    1f32.into(),
                    Attribute::from(Self::Dwarf).into(),
                    Some(Condition::has(Attribute::Feat(Feat::Proficiency(
                        Proficiency::MartialWeaponProficiency,
                    )))),
                ),
            ]),
            Self::Elf => Some(vec![
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
            Self::Gnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Halfling => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Strength, -2f32),
                self.bonus_feat(RacialFeat::HalflingAgility),
                self.bonus_feat(RacialFeat::HalflingBravery),
                self.bonus_feat(RacialFeat::HalflingKeenEars),
                self.bonus_feat(RacialFeat::HalflingLuck),
                self.bonus_feat(RacialFeat::HalflingThrownWeaponFocus),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
            ]),
            Self::HalfElf => Some(vec![
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    1f32.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::flag(
                    Immunity::Sleep.into(),
                    Attribute::from(Self::HalfElf).into(),
                ),
                Bonus::new(
                    Skill::Diplomacy.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
            ]),
            Self::HalfOrc => Some(vec![
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Self::Morninglord => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2f32),
                self.ability_modifier(Ability::Constitution, -2f32),
            ]),
            Self::PurpleDragonKnight | Self::Human => None,
            Self::Razorclaw => Some(vec![
                self.ability_modifier(Ability::Strength, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
            ]),
            Self::Shadarkai => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Self::Shifter => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
            ]),
            Self::Tabaxi | Self::Trailblazer => {
                Some(vec![self.ability_modifier(Ability::Dexterity, 2f32)])
            }
            Self::Tiefling => Some(vec![
                self.ability_modifier(Ability::Charisma, 2f32),
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Racial,
                    2f32.into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                // TODO: +2 to hit and damage against lawful outsiders and good outsiders
                // TODO: Fear Immunity
            ]),
            Self::Scoundrel => Some(vec![self.ability_modifier(Ability::Charisma, 2f32)]),
            Self::Warforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2f32),
                self.ability_modifier(Ability::Wisdom, -2f32),
                self.ability_modifier(Ability::Charisma, -2f32),
            ]),
            Self::WoodElf => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2f32),
                self.ability_modifier(Ability::Intelligence, -2f32),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
            ]),
        })?
    }
}
