use crate::breakdown::bonus_store::BonusStore;

mod inserting;
mod bonus_store;


#[derive(Debug, Clone)]
pub struct Breakdown {
    bonuses: BonusStore
}


impl Breakdown {}
