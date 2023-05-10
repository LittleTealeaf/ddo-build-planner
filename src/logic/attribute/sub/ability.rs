use itertools::Itertools;

use crate::{
    logic::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource, BonusType, Condition},
    },
    simple_attribute_enum,
};

use super::{Flag, Skill, WeaponHand, WeaponStat};

simple_attribute_enum!(Ability, (Strength "Strength", Dexterity "Dexterity", Constitution "Constitution", Intelligence "Intelligence", Wisdom "Wisdom", Charisma "Charisma"));

macro_rules! modifier_skill {
    ($modifier: ident, $skill: ident, $value: expr) => {
        Bonus::new(
            Attribute::Skill(Skill::$skill),
            BonusType::Stacking,
            $value,
            BonusSource::Attribute(Attribute::AbilityModifier(Ability::$modifier)),
            None,
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
            ],
            Ability::Dexterity => vec![
                modifier_skill!(Dexterity, Balance, value),
                modifier_skill!(Dexterity, Hide, value),
                modifier_skill!(Dexterity, MoveSilently, value),
                modifier_skill!(Dexterity, OpenLock, value),
                modifier_skill!(Dexterity, Tumble, value),
            ],
            Ability::Constitution => vec![modifier_skill!(Constitution, Concentration, value)],
            Ability::Intelligence => vec![
                modifier_skill!(Intelligence, DisableDevice, value),
                modifier_skill!(Intelligence, Repair, value),
                modifier_skill!(Intelligence, Search, value),
                modifier_skill!(Intelligence, SpellCraft, value),
            ],
            Ability::Wisdom => vec![
                modifier_skill!(Wisdom, Heal, value),
                modifier_skill!(Wisdom, Listen, value),
                modifier_skill!(Wisdom, Spot, value),
            ],
            Ability::Charisma => vec![
                modifier_skill!(Charisma, Bluff, value),
                modifier_skill!(Charisma, Diplomacy, value),
                modifier_skill!(Charisma, Haggle, value),
                modifier_skill!(Charisma, Intimidate, value),
                modifier_skill!(Charisma, Perform, value),
                modifier_skill!(Charisma, UseMagicalDevice, value),
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
