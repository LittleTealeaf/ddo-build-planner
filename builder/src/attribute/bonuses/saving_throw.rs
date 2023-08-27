use crate::{
    attribute::{Attribute, DefaultBonuses, TrackAttribute},
    bonus::{Bonus, BonusSource, BonusType, CloneBonus},
    types::{Ability, SavingThrow},
};

impl CloneBonus for SavingThrow {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::CORE_SAVING_THROWS
                .map(|st| {
                    Bonus::new(
                        st.into(),
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

impl DefaultBonuses for SavingThrow {
    type Iterator = [Bonus; 3];

    fn get_default_bonuses() -> Self::Iterator {
        [
            Bonus::new(
                Self::Reflex.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Dexterity).into(),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                Self::Fortitude.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Constitution).into(),
                BonusSource::Base,
                None,
            ),
            Bonus::new(
                Self::Will.into(),
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(Ability::Wisdom).into(),
                BonusSource::Base,
                None,
            ),
        ]
    }
}

impl TrackAttribute for SavingThrow {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(SavingThrow);

    #[test]
    fn all_is_not_tracked() {
        assert!(!SavingThrow::All.is_tracked());
        assert!(!Attribute::from(SavingThrow::All).is_tracked());
    }

    #[test]
    fn saving_throws_are_tracked() {
        let saving_throws = [
            SavingThrow::Fortitude,
            SavingThrow::Poison,
            SavingThrow::Disease,
            SavingThrow::Reflex,
            SavingThrow::Traps,
            SavingThrow::Spell,
            SavingThrow::Magic,
            SavingThrow::Will,
            SavingThrow::Enchantment,
            SavingThrow::Illusion,
            SavingThrow::Fear,
            SavingThrow::Curse,
        ];

        for st in saving_throws {
            assert!(st.is_tracked());
            assert!(Attribute::from(st).is_tracked());
        }
    }
}
