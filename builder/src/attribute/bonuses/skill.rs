use rust_decimal::Decimal;

use crate::{
    attribute::{Attribute, DefaultBonuses, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
    types::{Skill, SpellPower},
};

impl Skill {
    fn spell_power_bonus(self, sp: SpellPower, value: Decimal) -> Bonus {
        Bonus::new(
            Attribute::SpellPower(sp),
            BonusType::Stacking,
            value.into(),
            Attribute::Skill(self).into(),
            None,
        )
    }
}

impl CloneBonus for Skill {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::SKILLS
                .map(|skill| {
                    Bonus::new(
                        skill.into(),
                        bonus.get_type(),
                        bonus.get_value(),
                        bonus.get_source(),
                        bonus.get_condition(),
                    )
                })
                .to_vec()
        })
    }
}

macro_rules! skill_ability_bonus {
    ($ability: ident, $skill: ident) => {
        Bonus::new(
            $crate::attribute::Attribute::Skill(Skill::$skill).into(),
            $crate::bonus::BonusType::AbilityModifier,
            $crate::attribute::Attribute::AbilityModifier($crate::types::Ability::$ability).into(),
            $crate::bonus::BonusSource::Base,
            None,
        )
    };
}

impl DefaultBonuses for Skill {
    type Iterator = [Bonus; 21];

    fn get_default_bonuses() -> Self::Iterator {
        [
            skill_ability_bonus!(Dexterity, Balance),
            skill_ability_bonus!(Charisma, Bluff),
            skill_ability_bonus!(Constitution, Concentration),
            skill_ability_bonus!(Charisma, Diplomacy),
            skill_ability_bonus!(Intelligence, DisableDevice),
            skill_ability_bonus!(Charisma, Haggle),
            skill_ability_bonus!(Wisdom, Heal),
            skill_ability_bonus!(Dexterity, Hide),
            skill_ability_bonus!(Charisma, Intimidate),
            skill_ability_bonus!(Strength, Jump),
            skill_ability_bonus!(Wisdom, Listen),
            skill_ability_bonus!(Dexterity, MoveSilently),
            skill_ability_bonus!(Dexterity, OpenLock),
            skill_ability_bonus!(Charisma, Perform),
            skill_ability_bonus!(Intelligence, Repair),
            skill_ability_bonus!(Intelligence, Search),
            skill_ability_bonus!(Intelligence, Spellcraft),
            skill_ability_bonus!(Wisdom, Spot),
            skill_ability_bonus!(Strength, Swim),
            skill_ability_bonus!(Dexterity, Tumble),
            skill_ability_bonus!(Charisma, UseMagicalDevice),
        ]
    }
}

impl TrackAttribute for Skill {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(Skill);

    #[test]
    fn all_is_not_tracked() {
        assert!(!Skill::All.is_tracked());
        assert!(!Attribute::from(Skill::All).is_tracked());
    }

    #[test]
    fn skills_are_tracked() {
        for skill in Skill::SKILLS {
            assert!(skill.is_tracked());
            assert!(Attribute::from(skill).is_tracked());
        }
    }
}
