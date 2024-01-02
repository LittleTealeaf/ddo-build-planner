use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::ability::Ability,
};

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::ABILITIES
                .map(|ability| {
                    Bonus::new(
                        ability,
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
    use crate::attribute::Attribute;

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
