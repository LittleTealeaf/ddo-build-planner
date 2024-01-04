use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::sheltering::Sheltering,
};

impl CloneBonus for Sheltering {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Both).then(|| {
            [Self::Physical, Self::Magical]
                .map(|sheltering| bonus.clone_into_attribute(sheltering))
                .to_vec()
        })
    }
}

impl TrackAttribute for Sheltering {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Both)
    }
}
