use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::ability::Ability,
};

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ABILITIES
                .map(|ability| bonus.clone_into_attribute(ability))
                .to_vec()
        })
    }
}

impl TrackAttribute for Ability {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        attribute::Attribute,
        bonus::{BonusSource, BonusType},
    };

    use super::*;

    #[test]
    fn all_is_not_tracked() {
        assert!(!Ability::All.is_tracked());
        assert!(!Attribute::Ability(Ability::All).is_tracked());
        assert!(!Attribute::AbilityModifier(Ability::All).is_tracked());
    }

    #[test]
    fn abilities_are_tracked() {
        for ability in Ability::ABILITIES {
            assert!(ability.is_tracked());
            assert!(Attribute::Ability(ability).is_tracked());
            assert!(Attribute::AbilityModifier(ability).is_tracked());
        }
    }

    #[test]
    fn clone_bonus_return_none_for_ability() {
        for ability in Ability::ABILITIES {
            let bonus = ability.clone_bonus(&Bonus::new(
                Attribute::Ability(Ability::Wisdom),
                BonusType::Stacking,
                1,
                BonusSource::Debug(0),
                None,
            ));
            assert!(bonus.is_none());
        }
    }

    #[test]
    fn clone_bonus_returns_all_bonuses() {
        let bonus = Bonus::new(
            Ability::All,
            BonusType::Stacking,
            1,
            BonusSource::Debug(0),
            None,
        );

        let bonuses = Ability::All
            .clone_bonus(&bonus)
            .expect("Expected clone_bonus to return Some(_)");

        let attributes = bonuses
            .into_iter()
            .map(|bonus| *bonus.get_attribute())
            .collect::<Vec<_>>();

        for ability in Ability::ABILITIES {
            assert!(attributes.contains(&Attribute::Ability(ability)));
        }
    }
}
