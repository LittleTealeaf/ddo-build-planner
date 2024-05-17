use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusTemplate, BonusType, Condition},
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
        toggle::AttackingTarget,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

impl Race {
    fn ability_modifier(self, ability: Ability, value: i16) -> Bonus {
        Bonus::new(
            Attribute::Ability(ability),
            BonusType::Stacking,
            value,
            None,
            self,
        )
    }

    fn bonus_feat<F>(self, feat: F) -> Bonus
    where
        F: Into<Feat>,
    {
        Bonus::new(feat.into(), BonusType::Stacking, 1, None, self)
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<BonusTemplate>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Aasimar => Some(vec![
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Heal, BonusType::Racial, 2, None),
                BonusTemplate::new(Skill::Listen, BonusType::Racial, 2, None),
                BonusTemplate::new(Skill::Spot, BonusType::Racial, 2, None),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Cold),
                    BonusType::Stacking,
                    5,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Acid),
                    BonusType::Stacking,
                    5,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::Resistance(DamageType::Electric),
                    BonusType::Stacking,
                    5,
                    None,
                ),
            ]),
            Self::Scourge => Some(vec![BonusTemplate::new(
                Ability::Wisdom,
                BonusType::Stacking,
                2,
                None,
            )]),
            Self::Bladeforged => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, -2, None),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, -2, None),
            ]),
            Self::DeepGnome => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, -2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus, None),
                BonusTemplate::feat(RacialFeat::GnomishProficiencies, None),
                BonusTemplate::feat(RacialFeat::RacialSpellResistance, None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LightHammer), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::ThrowingHammer), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::WarHammer), None),
            ]),
            Self::Dragonborn => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, -2, None),
            ]),
            Self::Drow => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::RacialSpellResistance, None),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses, None),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus, None),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep, None),
                BonusTemplate::feat(Proficiency::from(WeaponType::Shuriken), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::ShortSword), None),
                // TODO: Shuriken Expertise
            ]),
            Self::Dwarf => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::GiantEvasion, None),
                BonusTemplate::feat(RacialFeat::OrcAndGoblinBonus, None),
                BonusTemplate::feat(RacialFeat::PoisonSaveBonus, None),
                BonusTemplate::feat(RacialFeat::DwarvenStonecunning, None),
                BonusTemplate::feat(RacialFeat::DwarvenStability, None),
                BonusTemplate::feat(RacialFeat::SpellSaveBonus, None),
                BonusTemplate::feat(
                    Proficiency::from(WeaponType::DwarvenWarAxe),
                    Condition::has(Proficiency::MartialWeaponProficiency),
                ),
            ]),
            Self::Elf => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses, None),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep, None),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus, None),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongBow), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::ShortBow), None),
            ]),
            Self::Gnome => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus, None),
                BonusTemplate::feat(RacialFeat::GnomishProficiencies, None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LightHammer), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::ThrowingHammer), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::WarHammer), None),
            ]),
            Self::Halfling => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::HalflingAgility, None),
                BonusTemplate::feat(RacialFeat::HalflingBravery, None),
                BonusTemplate::feat(RacialFeat::HalflingKeenEars, None),
                BonusTemplate::feat(RacialFeat::HalflingLuck, None),
                BonusTemplate::feat(RacialFeat::HalflingThrownWeaponFocus, None),
                BonusTemplate::feat(RacialFeat::SmallSizeBonus, None),
            ]),
            Self::HalfElf => Some(vec![
                BonusTemplate::new(Skill::Listen, BonusType::Racial, 1, None),
                BonusTemplate::new(Skill::Search, BonusType::Racial, 1, None),
                BonusTemplate::new(Skill::Spot, BonusType::Racial, 1, None),
                BonusTemplate::new(Skill::Diplomacy, BonusType::Racial, 2, None),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep, None),
            ]),
            Self::HalfOrc => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, -2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
            ]),
            Self::Morninglord => Some(vec![
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, -2, None),
            ]),
            Self::PurpleDragonKnight | Self::Human => None,
            Self::Razorclaw => Some(vec![
                BonusTemplate::new(Ability::Strength, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, -2, None),
            ]),
            Self::Shadarkai => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
            ]),
            Self::Shifter => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, -2, None),
            ]),
            Self::Tabaxi | Self::Trailblazer => Some(vec![BonusTemplate::new(
                Ability::Dexterity,
                BonusType::Stacking,
                2,
                None,
            )]),
            Self::Tiefling => Some(vec![
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, 2, None),
                BonusTemplate::new(Skill::Balance, BonusType::Racial, 2, None),
                BonusTemplate::new(SavingThrow::Spell, BonusType::Racial, 2, None),
                BonusTemplate::toggle(AttackingTarget::Alignment(Alignment::Lawful), None),
                BonusTemplate::toggle(AttackingTarget::Alignment(Alignment::Good), None),
                BonusTemplate::toggle(AttackingTarget::MonsterType(MonsterType::Outsiders), None),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Stacking,
                    2,
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                BonusTemplate::new(
                    (WeaponHand::Both, WeaponStat::Damage),
                    BonusType::Stacking,
                    2,
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                BonusTemplate::flag(Immunity::Fear, None),
            ]),
            Self::Scoundrel => Some(vec![BonusTemplate::new(
                Ability::Charisma,
                BonusType::Stacking,
                2,
                None,
            )]),
            Self::Warforged => Some(vec![
                BonusTemplate::new(Ability::Constitution, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Wisdom, BonusType::Stacking, -2, None),
                BonusTemplate::new(Ability::Charisma, BonusType::Stacking, -2, None),
            ]),
            Self::WoodElf => Some(vec![
                BonusTemplate::new(Ability::Dexterity, BonusType::Stacking, 2, None),
                BonusTemplate::new(Ability::Intelligence, BonusType::Stacking, -2, None),
                BonusTemplate::feat(RacialFeat::ElvenKeenSenses, None),
                BonusTemplate::feat(RacialFeat::EnchantmentSaveBonus, None),
                BonusTemplate::feat(RacialFeat::ImmunityToSleep, None),
                BonusTemplate::feat(Proficiency::from(WeaponType::Rapier), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword), None),
                BonusTemplate::feat(Proficiency::from(WeaponType::LongSword), None),
            ]),
        })?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_returns_nothing() {
        let races = [
            Race::Dragonborn,
            Race::Drow,
            Race::Dwarf,
            Race::Elf,
            Race::Gnome,
            Race::Halfling,
            Race::HalfElf,
            Race::HalfOrc,
            Race::Human,
            Race::Tiefling,
            Race::Warforged,
            Race::WoodElf,
            Race::Aasimar,
            Race::Shifter,
            Race::Tabaxi,
            Race::Bladeforged,
            Race::DeepGnome,
            Race::Morninglord,
            Race::PurpleDragonKnight,
            Race::Razorclaw,
            Race::Scoundrel,
            Race::Scourge,
            Race::Shadarkai,
            Race::Trailblazer,
        ];

        for race in races {
            assert!(race.get_bonuses(Decimal::ZERO).is_none());
        }
    }
}
