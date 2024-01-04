use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::saving_throw::SavingThrow,
};

impl CloneBonus for SavingThrow {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::PRIMARY
                .map(|st| bonus.clone_into_attribute(st))
                .to_vec()
        })
    }
}

impl TrackAttribute for SavingThrow {
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
