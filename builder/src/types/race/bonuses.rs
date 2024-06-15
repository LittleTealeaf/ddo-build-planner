use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusTemplate, BonusType, Condition, Value},
    feat::{Feat, Proficiency, RacialFeat},
    types::{
        ability::Ability,
        alignment::Alignment,
        damage_type::DamageType,
        immunity::Immunity,
        item_type::WeaponType,
        monster_type::MonsterType,
        race::Race,
        saving_throw::SavingThrow,
        skill::Skill,
        toggle::{AttackingTarget, SeasonalAffinity},
        weapon_attribute::{WeaponHand, WeaponStat},
    },
    val,
};

impl Race {
    fn ability_modifier(self, ability: Ability, value: i16) -> Bonus {
        Bonus::new(
            Attribute::Ability(ability),
            BonusType::Stacking,
            value,
            self,
        )
    }

    fn bonus_feat<F>(self, feat: F) -> Bonus
    where
        F: Into<Feat>,
    {
        Bonus::new(feat.into(), BonusType::Stacking, Value::ONE, self)
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Aasimar => Some(vec![
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Heal, BonusType::Racial, Value::TWO),
                BonusTemplate::new(Skill::Listen, BonusType::Racial, Value::TWO),
                BonusTemplate::new(Skill::Spot, BonusType::Racial, Value::TWO),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Cold),
                    BonusType::Stacking,
                    val!(5),
                ),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Acid),
                    BonusType::Stacking,
                    val!(5),
                ),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Electric),
                    BonusType::Stacking,
                    val!(5),
                ),
            ]),
            Self::Scourge => Some(vec![BonusTemplate::new(
                Ability::Wisdom,
                BonusType::Stacking,
                Value::TWO,
            )]),
            Self::Bladeforged => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, val!(-2)),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, val!(-2)),
            ]),
            Self::DeepGnome => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, val!(-2)),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus),
                BonusTemplate::feat(RacialFeat::GnomishProficiencies),
                BonusTemplate::feat(RacialFeat::RacialSpellResistance),
                BonusTemplate::feat(Proficiency::from(WeaponType::LightHammer)),
                BonusTemplate::feat(Proficiency::from(WeaponType::ThrowingHammer)),
                BonusTemplate::feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Dragonborn => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, val!(-2)),
            ]),
            Self::Drow => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::RacialSpellResistance),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
                BonusTemplate::feat(Proficiency::from(WeaponType::Shuriken)),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier)),
                BonusTemplate::feat(Proficiency::from(WeaponType::ShortSword)),
                // TODO: Shuriken Expertise
            ]),
            Self::Dwarf => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::GiantEvasion),
                BonusTemplate::feat(RacialFeat::OrcAndGoblinBonus),
                BonusTemplate::feat(RacialFeat::PoisonSaveBonus),
                BonusTemplate::feat(RacialFeat::DwarvenStonecunning),
                BonusTemplate::feat(RacialFeat::DwarvenStability),
                BonusTemplate::feat(RacialFeat::SpellSaveBonus),
                BonusTemplate::feat(Proficiency::from(WeaponType::DwarvenWarAxe))
                    .with_condition(Condition::has(Proficiency::MartialWeaponProficiency)),
            ]),
            Self::Elf => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier)),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword)),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongBow)),
                BonusTemplate::feat(Proficiency::from(WeaponType::ShortBow)),
            ]),
            Self::Gnome => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus),
                BonusTemplate::feat(RacialFeat::GnomishProficiencies),
                BonusTemplate::feat(Proficiency::from(WeaponType::LightHammer)),
                BonusTemplate::feat(Proficiency::from(WeaponType::ThrowingHammer)),
                BonusTemplate::feat(Proficiency::from(WeaponType::WarHammer)),
            ]),
            Self::Halfling => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::HalflingAgility),
                BonusTemplate::feat(RacialFeat::HalflingBravery),
                BonusTemplate::feat(RacialFeat::HalflingKeenEars),
                BonusTemplate::feat(RacialFeat::HalflingLuck),
                BonusTemplate::feat(RacialFeat::HalflingThrownWeaponFocus),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus),
            ]),
            Self::HalfElf => Some(vec![
                BonusTemplate::new(Skill::Listen, BonusType::Racial, 1),
                BonusTemplate::new(Skill::Search, BonusType::Racial, 1),
                BonusTemplate::new(Skill::Spot, BonusType::Racial, 1),
                BonusTemplate::new(Skill::Diplomacy, BonusType::Racial, Value::TWO),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
            ]),
            Self::HalfOrc => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, val!(-2)),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
            ]),
            Self::Morninglord => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, val!(-2)),
            ]),
            Self::PurpleDragonKnight | Self::Human => None,
            Self::Razorclaw => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, val!(-2)),
            ]),
            Self::Shadarkai => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
            ]),
            Self::Shifter => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, val!(-2)),
            ]),
            Self::Tabaxi | Self::Trailblazer => Some(vec![BonusTemplate::new(
                Ability::Dexterity,
                BonusType::Stacking,
                Value::TWO,
            )]),
            Self::Tiefling => Some(vec![
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Skill::Balance, BonusType::Racial, Value::TWO),
                BonusTemplate::new(SavingThrow::Spell, BonusType::Racial, Value::TWO),
                BonusTemplate::toggle(AttackingTarget::Alignment(Alignment::Lawful)),
                BonusTemplate::toggle(AttackingTarget::Alignment(Alignment::Good)),
                BonusTemplate::toggle(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Stacking,
                    Value::TWO,
                )
                .with_condition(
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Damage),
                    BonusType::Stacking,
                    Value::TWO,
                )
                .with_condition(
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                BonusTemplate::flag(Immunity::Fear),
            ]),
            Self::Scoundrel => Some(vec![BonusTemplate::new(
                Ability::Charisma,
                BonusType::Stacking,
                Value::TWO,
            )]),
            Self::Warforged => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, val!(-2)),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, val!(-2)),
            ]),
            Self::WoodElf => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, val!(-2)),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier)),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword)),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword)),
            ]),
            Self::Eladrin => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, Value::TWO),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus),
                BonusTemplate::toggle(SeasonalAffinity::Spring),
                BonusTemplate::toggle(SeasonalAffinity::Summer),
                BonusTemplate::toggle(SeasonalAffinity::Winter),
                BonusTemplate::toggle(SeasonalAffinity::Autumn),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Spring))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Spring)),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Summer))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Summer)),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Autumn))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Autumn)),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Winter))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Winter)),
            ]),
            Self::Chaosmancer => Some(vec![
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::TWO),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus),
                BonusTemplate::toggle(SeasonalAffinity::Spring),
                BonusTemplate::toggle(SeasonalAffinity::Summer),
                BonusTemplate::toggle(SeasonalAffinity::Winter),
                BonusTemplate::toggle(SeasonalAffinity::Autumn),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Spring))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Spring)),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Summer))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Summer)),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Autumn))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Autumn)),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, Value::ONE)
                    .with_condition(Condition::toggled(SeasonalAffinity::Winter))
                    .with_display_source(Attribute::toggle(SeasonalAffinity::Winter)),
            ]),
        })?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_returns_nothing() {
        let races = Race::ALL;

        for race in races {
            assert!(race.get_bonuses(Decimal::ZERO).is_none());
        }
    }
}
