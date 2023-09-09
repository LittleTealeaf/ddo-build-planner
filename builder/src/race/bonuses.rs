use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType, Condition},
    equipment::item::types::WeaponType,
    feat::{Feat, Proficiency},
    types::{Ability, DamageType, Immunity, Race, SavingThrow, Skill},
};

use super::RacialFeat;

impl Race {
    fn ability_modifier(self, ability: Ability, value: Decimal) -> Bonus {
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
            dec!(1).into(),
            Attribute::from(self).into(),
            None,
        )
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<Bonus>> {
        (value > dec!(0)).then(|| match self {
            Self::Aasimar => Some(vec![
                self.ability_modifier(Ability::Wisdom, dec!(2)),
                Bonus::new(
                    Skill::Heal.into(),
                    BonusType::Racial,
                    dec!(2).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    dec!(2).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    dec!(2).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Cold),
                    BonusType::Stacking,
                    dec!(5).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Acid),
                    BonusType::Stacking,
                    dec!(5).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Electric),
                    BonusType::Stacking,
                    dec!(5).into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
            ]),
            Self::Scourge => Some(vec![self.ability_modifier(Ability::Wisdom, dec!(2))]),
            Self::Bladeforged => Some(vec![
                self.ability_modifier(Ability::Constitution, dec!(2)),
                self.ability_modifier(Ability::Dexterity, dec!(-2)),
                self.ability_modifier(Ability::Wisdom, dec!(-2)),
            ]),
            Self::DeepGnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, dec!(2)),
                self.ability_modifier(Ability::Wisdom, dec!(2)),
                self.ability_modifier(Ability::Strength, dec!(-2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(RacialFeat::RacialSpellResistance),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Dragonborn => Some(vec![
                self.ability_modifier(Ability::Strength, dec!(2)),
                self.ability_modifier(Ability::Charisma, dec!(2)),
                self.ability_modifier(Ability::Dexterity, dec!(-2)),
            ]),
            Self::Drow => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Intelligence, dec!(2)),
                self.ability_modifier(Ability::Charisma, dec!(2)),
                self.ability_modifier(Ability::Constitution, dec!(-2)),
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
                self.ability_modifier(Ability::Constitution, dec!(2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
                self.bonus_feat(RacialFeat::GiantEvasion),
                self.bonus_feat(RacialFeat::OrcAndGoblinBonus),
                self.bonus_feat(RacialFeat::PoisonSaveBonus),
                self.bonus_feat(RacialFeat::DwarvenStonecunning),
                self.bonus_feat(RacialFeat::DwarvenStability),
                self.bonus_feat(RacialFeat::SpellSaveBonus),
                Bonus::new(
                    Feat::from(Proficiency::from(WeaponType::DwarvenWarAxe)).into(),
                    BonusType::Stacking,
                    dec!(1).into(),
                    Attribute::from(Self::Dwarf).into(),
                    Some(Condition::has(Attribute::Feat(Feat::Proficiency(
                        Proficiency::MartialWeaponProficiency,
                    )))),
                ),
            ]),
            Self::Elf => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Constitution, dec!(-2)),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
                self.bonus_feat(Proficiency::from(WeaponType::LongBow)),
                self.bonus_feat(Proficiency::from(WeaponType::ShortBow)),
            ]),
            Self::Gnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, dec!(2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Halfling => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Strength, dec!(-2)),
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
                    dec!(1).into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    dec!(1).into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    dec!(1).into(),
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
                    dec!(2).into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
            ]),
            Self::HalfOrc => Some(vec![
                self.ability_modifier(Ability::Strength, dec!(2)),
                self.ability_modifier(Ability::Intelligence, dec!(-2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
            ]),
            Self::Morninglord => Some(vec![
                self.ability_modifier(Ability::Intelligence, dec!(2)),
                self.ability_modifier(Ability::Constitution, dec!(-2)),
            ]),
            Self::PurpleDragonKnight | Self::Human => None,
            Self::Razorclaw => Some(vec![
                self.ability_modifier(Ability::Strength, dec!(2)),
                self.ability_modifier(Ability::Intelligence, dec!(-2)),
            ]),
            Self::Shadarkai => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
            ]),
            Self::Shifter => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Intelligence, dec!(-2)),
            ]),
            Self::Tabaxi | Self::Trailblazer => {
                Some(vec![self.ability_modifier(Ability::Dexterity, dec!(2))])
            }
            Self::Tiefling => Some(vec![
                self.ability_modifier(Ability::Charisma, dec!(2)),
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Racial,
                    dec!(2).into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Racial,
                    dec!(2).into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                // TODO: +2 to hit and damage against lawful outsiders and good outsiders
                // TODO: Fear Immunity
            ]),
            Self::Scoundrel => Some(vec![self.ability_modifier(Ability::Charisma, dec!(2))]),
            Self::Warforged => Some(vec![
                self.ability_modifier(Ability::Constitution, dec!(2)),
                self.ability_modifier(Ability::Wisdom, dec!(-2)),
                self.ability_modifier(Ability::Charisma, dec!(-2)),
            ]),
            Self::WoodElf => Some(vec![
                self.ability_modifier(Ability::Dexterity, dec!(2)),
                self.ability_modifier(Ability::Intelligence, dec!(-2)),
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
