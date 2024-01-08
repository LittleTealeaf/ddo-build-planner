use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::{Bonus, BonusType, Condition},
    feat::{Feat, Proficiency, RacialFeat},
    types::{
        ability::Ability,
        alignment::Alignment,
        damage_type::DamageType,
        immunity::Immunity,
        item::WeaponType,
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
            self,
            None,
        )
    }

    fn bonus_feat(self, feat: impl Into<Feat>) -> Bonus {
        Bonus::new(feat.into(), BonusType::Stacking, 1, self, None)
    }
}

impl GetBonuses for Race {
    fn get_bonuses(&self, value: Decimal) -> Option<Vec<Bonus>> {
        (value > Decimal::ZERO).then(|| match self {
            Self::Aasimar => Some(vec![
                self.ability_modifier(Ability::Wisdom, 2),
                Bonus::new(Skill::Heal, BonusType::Racial, 2, Self::Aasimar, None),
                Bonus::new(Skill::Listen, BonusType::Racial, 2, Self::Aasimar, None),
                Bonus::new(Skill::Spot, BonusType::Racial, 2, Self::Aasimar, None),
                Bonus::new(
                    Attribute::Resistance(DamageType::Cold),
                    BonusType::Stacking,
                    5,
                    Self::Aasimar,
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Acid),
                    BonusType::Stacking,
                    5,
                    Self::Aasimar,
                    None,
                ),
                Bonus::new(
                    Attribute::Resistance(DamageType::Electric),
                    BonusType::Stacking,
                    5,
                    Self::Aasimar,
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
                Bonus::feat(
                    Proficiency::from(WeaponType::DwarvenWarAxe),
                    Self::Dwarf,
                    Condition::has(Proficiency::MartialWeaponProficiency),
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
                Bonus::new(Skill::Listen, BonusType::Racial, 1, Self::HalfElf, None),
                Bonus::new(Skill::Search, BonusType::Racial, 1, Self::HalfElf, None),
                Bonus::new(Skill::Spot, BonusType::Racial, 1, Self::HalfElf, None),
                Bonus::flag(Immunity::Sleep, Self::HalfElf, None),
                Bonus::new(Skill::Diplomacy, BonusType::Racial, 2, Self::HalfElf, None),
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
                Bonus::new(Skill::Balance, BonusType::Racial, 2, Self::Tiefling, None),
                Bonus::new(
                    SavingThrow::Spell,
                    BonusType::Racial,
                    2,
                    Self::Tiefling,
                    None,
                ),
                Bonus::toggle(
                    AttackingTarget::Alignment(Alignment::Lawful),
                    Self::Tiefling,
                    None,
                ),
                Bonus::toggle(
                    AttackingTarget::Alignment(Alignment::Good),
                    Self::Tiefling,
                    None,
                ),
                Bonus::toggle(
                    AttackingTarget::MonsterType(MonsterType::Outsiders),
                    Self::Tiefling,
                    None,
                ),
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Stacking,
                    2,
                    Self::Tiefling,
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                Bonus::new(
                    (WeaponHand::Both, WeaponStat::Damage),
                    BonusType::Stacking,
                    2,
                    Self::Tiefling,
                    (Condition::toggled(AttackingTarget::Alignment(Alignment::Lawful))
                        | Condition::toggled(AttackingTarget::Alignment(Alignment::Good)))
                        & Condition::toggled(AttackingTarget::MonsterType(MonsterType::Outsiders)),
                ),
                Bonus::flag(Immunity::Fear, Self::Tiefling, None),
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
