use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, BonusType, CloneBonus},
    types::ability::Ability,
};

impl Ability {
    fn modifier_bonus<T>(self, attribute: T, value: f32) -> Bonus
    where
        Attribute: From<T>,
    {
        Bonus::new(
            attribute.into(),
            BonusType::AbilityModifier,
            value.into(),
            Attribute::AbilityModifier(self).into(),
            None,
        )
    }
}


impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ABILITIES
                .map(|ability| {
                    Bonus::new(
                        ability.into(),
                        *bonus.get_type(),
                        bonus.get_value().clone(),
                        *bonus.get_source(),
                        bonus.get_condition().cloned(),
                    )
                })
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
}
