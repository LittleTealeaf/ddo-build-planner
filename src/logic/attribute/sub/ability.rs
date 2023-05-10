use itertools::Itertools;

use crate::{
    logic::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition},
    },
    simple_attribute_enum,
};

use super::{Flag, SavingThrow, Skill, WeaponHand, WeaponStat};

simple_attribute_enum!(Ability, (Strength "Strength", Dexterity "Dexterity", Constitution "Constitution", Intelligence "Intelligence", Wisdom "Wisdom", Charisma "Charisma"));

macro_rules! modifier_skill {
    ($modifier: ident, $skill: ident, $value: expr) => {
        Bonus::new(
            Attribute::Skill(Skill::$skill),
            BonusType::AbilityModifier,
            $value,
            BonusSource::Attribute(Attribute::AbilityModifier(Ability::$modifier)),
            None,
        )
    };
}

macro_rules! modifier_saving_throw {
    ($modifier: ident, $saving_throw: ident, $value: expr, $def: expr) => {
        Bonus::new(
            Attribute::SavingThrow(SavingThrow::$saving_throw),
            BonusType::AbilityModifier,
            $value,
            BonusSource::Attribute(Attribute::AbilityModifier(Ability::$modifier)),
            if $def {
                None
            } else {
                Some(vec![Condition::Has(Attribute::Flag(
                    Flag::AbilityToSavingThrow(Ability::$modifier, SavingThrow::$saving_throw),
                ))])
            },
        )
    };
}

impl Ability {
    pub fn get_score_bonuses(&self, value: f32) -> Vec<Bonus> {
        vec![Bonus::new(
            Attribute::AbilityModifier(*self),
            BonusType::Stacking,
            ((value - 10f32) / 2f32).floor(),
            BonusSource::Attribute(Attribute::AbilityScore(*self)),
            None,
        )]
    }

    pub fn get_modifier_bonuses(&self, value: f32) -> Vec<Bonus> {
        let mut vec = match self {
            Ability::Strength => vec![
                modifier_skill!(Strength, Jump, value),
                modifier_skill!(Strength, Swim, value),
                modifier_saving_throw!(Strength, Reflex, value, false),
                modifier_saving_throw!(Strength, Fortitude, value, false),
                modifier_saving_throw!(Strength, Will, value, false),
            ],
            Ability::Dexterity => vec![
                modifier_skill!(Dexterity, Balance, value),
                modifier_skill!(Dexterity, Hide, value),
                modifier_skill!(Dexterity, MoveSilently, value),
                modifier_skill!(Dexterity, OpenLock, value),
                modifier_skill!(Dexterity, Tumble, value),
                modifier_saving_throw!(Dexterity, Reflex, value, true),
                modifier_saving_throw!(Dexterity, Fortitude, value, false),
                modifier_saving_throw!(Dexterity, Will, value, false),
            ],
            Ability::Constitution => vec![
                modifier_skill!(Constitution, Concentration, value),
                modifier_saving_throw!(Constitution, Reflex, value, false),
                modifier_saving_throw!(Constitution, Fortitude, value, true),
                modifier_saving_throw!(Constitution, Will, value, false),
            ],
            Ability::Intelligence => vec![
                modifier_skill!(Intelligence, DisableDevice, value),
                modifier_skill!(Intelligence, Repair, value),
                modifier_skill!(Intelligence, Search, value),
                modifier_skill!(Intelligence, SpellCraft, value),
                modifier_saving_throw!(Intelligence, Reflex, value, false),
                modifier_saving_throw!(Intelligence, Fortitude, value, false),
                modifier_saving_throw!(Intelligence, Will, value, false),
            ],
            Ability::Wisdom => vec![
                modifier_skill!(Wisdom, Heal, value),
                modifier_skill!(Wisdom, Listen, value),
                modifier_skill!(Wisdom, Spot, value),
                modifier_saving_throw!(Wisdom, Reflex, value, false),
                modifier_saving_throw!(Wisdom, Fortitude, value, false),
                modifier_saving_throw!(Wisdom, Will, value, true),
            ],
            Ability::Charisma => vec![
                modifier_skill!(Charisma, Bluff, value),
                modifier_skill!(Charisma, Diplomacy, value),
                modifier_skill!(Charisma, Haggle, value),
                modifier_skill!(Charisma, Intimidate, value),
                modifier_skill!(Charisma, Perform, value),
                modifier_skill!(Charisma, UseMagicalDevice, value),
                modifier_saving_throw!(Charisma, Reflex, value, false),
                modifier_saving_throw!(Charisma, Fortitude, value, false),
                modifier_saving_throw!(Charisma, Will, value, false),
            ],
        };

        vec.append(
            &mut [WeaponHand::Both, WeaponHand::OffHand, WeaponHand::MainHand]
                .into_iter()
                .map(|hand| {
                    Bonus::new(
                        Attribute::WeaponStat(hand, WeaponStat::Attack),
                        BonusType::AbilityModifier,
                        value,
                        BonusSource::Attribute(Attribute::AbilityModifier(*self)),
                        Some(vec![Condition::Has(Attribute::Flag(
                            Flag::AbilityToAttack(hand, *self),
                        ))]),
                    )
                })
                .collect_vec(),
        );

        vec.append(
            &mut [WeaponHand::Both, WeaponHand::OffHand, WeaponHand::MainHand]
                .into_iter()
                .map(|hand| {
                    Bonus::new(
                        Attribute::WeaponStat(hand, WeaponStat::Damage),
                        BonusType::AbilityModifier,
                        value,
                        BonusSource::Attribute(Attribute::AbilityModifier(*self)),
                        Some(vec![Condition::Has(Attribute::Flag(
                            Flag::AbilityToDamage(hand, *self),
                        ))]),
                    )
                })
                .collect_vec(),
        );

        vec
    }
}
