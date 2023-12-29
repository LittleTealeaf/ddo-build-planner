use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType, Condition},
    feat::{Feat, Proficiency},
    types::{
        ability::Ability, damage_type::DamageType, immunity::Immunity, race::Race,
        saving_throw::SavingThrow, skill::Skill, item::WeaponType,
    }, race::RacialFeat,
};


impl Race {
    fn ability_modifier(self, ability: Ability, value: i16) -> Bonus {
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
            1.into(),
            Attribute::from(self).into(),
            None,
        )
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<Bonus>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Aasimar => Some(vec![
                self.ability_modifier(Ability::Wisdom, 2),
                Bonus::new(
                    Skill::Heal.into(),
                    BonusType::Racial,
                    2.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Listen.into(),
                    BonusType::Racial,
                    2.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    2.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Cold),
                    BonusType::Stacking,
                    5.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Acid),
                    BonusType::Stacking,
                    5.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Electric),
                    BonusType::Stacking,
                    5.into(),
                    Attribute::from(Self::Aasimar).into(),
                    None,
                ),
            ]),
            Self::Scourge => Some(vec![self.ability_modifier(Ability::Wisdom, 2)]),
            Self::Bladeforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2),
                self.ability_modifier(Ability::Dexterity, -2),
                self.ability_modifier(Ability::Wisdom, -2),
            ]),
            Self::DeepGnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2),
                self.ability_modifier(Ability::Wisdom, 2),
                self.ability_modifier(Ability::Strength, -2),
                self.ability_modifier(Ability::Charisma, -2),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(RacialFeat::RacialSpellResistance),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Dragonborn => Some(vec![
                self.ability_modifier(Ability::Strength, 2),
                self.ability_modifier(Ability::Charisma, 2),
                self.ability_modifier(Ability::Dexterity, -2),
            ]),
            Self::Drow => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Intelligence, 2),
                self.ability_modifier(Ability::Charisma, 2),
                self.ability_modifier(Ability::Constitution, -2),
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
                self.ability_modifier(Ability::Constitution, 2),
                self.ability_modifier(Ability::Charisma, -2),
                self.bonus_feat(RacialFeat::GiantEvasion),
                self.bonus_feat(RacialFeat::OrcAndGoblinBonus),
                self.bonus_feat(RacialFeat::PoisonSaveBonus),
                self.bonus_feat(RacialFeat::DwarvenStonecunning),
                self.bonus_feat(RacialFeat::DwarvenStability),
                self.bonus_feat(RacialFeat::SpellSaveBonus),
                Bonus::new(
                    Feat::from(Proficiency::from(WeaponType::DwarvenWarAxe)).into(),
                    BonusType::Stacking,
                    1.into(),
                    Attribute::from(Self::Dwarf).into(),
                    Some(Condition::has(Attribute::Feat(Feat::Proficiency(
                        Proficiency::MartialWeaponProficiency,
                    )))),
                ),
            ]),
            Self::Elf => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Constitution, -2),
                self.bonus_feat(RacialFeat::ElvenKeenSenses),
                self.bonus_feat(RacialFeat::ImmunityToSleep),
                self.bonus_feat(RacialFeat::EnchantmentSaveBonus),
                self.bonus_feat(Proficiency::from(WeaponType::Rapier)),
                self.bonus_feat(Proficiency::from(WeaponType::LongSword)),
                self.bonus_feat(Proficiency::from(WeaponType::LongBow)),
                self.bonus_feat(Proficiency::from(WeaponType::ShortBow)),
            ]),
            Self::Gnome => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2),
                self.ability_modifier(Ability::Charisma, -2),
                self.bonus_feat(RacialFeat::SmallSizeBonus),
                self.bonus_feat(RacialFeat::GnomishProficiencies),
                self.bonus_feat(Proficiency::from(WeaponType::LightHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::ThrowingHammer)),
                self.bonus_feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Halfling => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Strength, -2),
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
                    1.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Search.into(),
                    BonusType::Racial,
                    1.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
                Bonus::new(
                    Skill::Spot.into(),
                    BonusType::Racial,
                    1.into(),
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
                    2.into(),
                    Attribute::from(Self::HalfElf).into(),
                    None,
                ),
            ]),
            Self::HalfOrc => Some(vec![
                self.ability_modifier(Ability::Strength, 2),
                self.ability_modifier(Ability::Intelligence, -2),
                self.ability_modifier(Ability::Charisma, -2),
            ]),
            Self::Morninglord => Some(vec![
                self.ability_modifier(Ability::Intelligence, 2),
                self.ability_modifier(Ability::Constitution, -2),
            ]),
            Self::PurpleDragonKnight | Self::Human => None,
            Self::Razorclaw => Some(vec![
                self.ability_modifier(Ability::Strength, 2),
                self.ability_modifier(Ability::Intelligence, -2),
            ]),
            Self::Shadarkai => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Charisma, -2),
            ]),
            Self::Shifter => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Intelligence, -2),
            ]),
            Self::Tabaxi | Self::Trailblazer => {
                Some(vec![self.ability_modifier(Ability::Dexterity, 2)])
            }
            Self::Tiefling => Some(vec![
                self.ability_modifier(Ability::Charisma, 2),
                Bonus::new(
                    Skill::Balance.into(),
                    BonusType::Racial,
                    2.into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                Bonus::new(
                    SavingThrow::Spell.into(),
                    BonusType::Racial,
                    2.into(),
                    Attribute::from(Self::Tiefling).into(),
                    None,
                ),
                // TODO: +2 to hit and damage against lawful outsiders and good outsiders
                // TODO: Fear Immunity
            ]),
            Self::Scoundrel => Some(vec![self.ability_modifier(Ability::Charisma, 2)]),
            Self::Warforged => Some(vec![
                self.ability_modifier(Ability::Constitution, 2),
                self.ability_modifier(Ability::Wisdom, -2),
                self.ability_modifier(Ability::Charisma, -2),
            ]),
            Self::WoodElf => Some(vec![
                self.ability_modifier(Ability::Dexterity, 2),
                self.ability_modifier(Ability::Intelligence, -2),
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
