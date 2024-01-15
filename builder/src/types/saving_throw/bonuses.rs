use crate::{
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
