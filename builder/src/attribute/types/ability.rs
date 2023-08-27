use crate::{
    attribute::{Attribute, DefaultBonuses, TrackAttribute},
    bonus::{Bonus, BonusSource, BonusType, CloneBonus, Value},
    types::Ability,
};

impl Ability {
    /// Represents the 6 different values that [`Ability`] can be (without [`Ability::All`])
    pub const VALUES: [Self; 6] = [
        Self::Strength,
        Self::Dexterity,
        Self::Constitution,
        Self::Intelligence,
        Self::Wisdom,
        Self::Charisma,
    ];

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

impl DefaultBonuses for Ability {
    type Iterator = std::iter::Flatten<std::array::IntoIter<[Bonus; 2], 6>>;

    fn get_default_bonuses() -> Self::Iterator {
        Self::VALUES
            .map(|ability| {
                [
                    Bonus::new(
                        Attribute::Ability(ability),
                        BonusType::Stacking,
                        8f32.into(),
                        BonusSource::Base,
                        None,
                    ),
                    Bonus::new(
                        Attribute::AbilityModifier(ability),
                        BonusType::Stacking,
                        Value::Floor(Box::new(Value::Product(vec![
                            Value::Sum(vec![Attribute::Ability(ability).into(), (-10f32).into()]),
                            0.5f32.into(),
                        ]))),
                        BonusSource::Base,
                        None,
                    ),
                ]
            })
            .into_iter()
            .flatten()
    }
}

impl CloneBonus for Ability {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::VALUES
                .map(|ability| {
                    Bonus::new(
                        ability.into(),
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

impl TrackAttribute for Ability {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_default_bonuses;

    use super::*;

    test_default_bonuses!(Ability);

    #[test]
    fn all_is_not_tracked() {
        assert!(!Ability::All.is_tracked());
        assert!(!Attribute::Ability(Ability::All).is_tracked());
        assert!(!Attribute::AbilityModifier(Ability::All).is_tracked());
    }

    #[test]
    fn abilities_are_tracked() {
        for ability in Ability::VALUES {
            assert!(ability.is_tracked());
            assert!(Attribute::Ability(ability).is_tracked());
            assert!(Attribute::AbilityModifier(ability).is_tracked());
        }
    }
}
