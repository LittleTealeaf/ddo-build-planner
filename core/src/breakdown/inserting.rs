use crate::{bonus::Bonus, breakdown::Breakdown};

impl Breakdown {
    pub fn insert_bonuses<I>(&mut self, bonuses: I)
    where
        I: IntoIterator<Item = Bonus>,
    {
        todo!()
    }
}
