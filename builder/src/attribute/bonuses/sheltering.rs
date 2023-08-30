use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::Sheltering,
};

impl CloneBonus for Sheltering {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::Both).then(|| {
            [Self::Physical, Self::Magical]
                .map(|sheltering| {
                    Bonus::new(
                        sheltering.into(),
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

impl TrackAttribute for Sheltering {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::Both)
    }
}
