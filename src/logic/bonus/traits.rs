use super::Bonus;

pub trait Bonuses {
    fn get_bonuses(&self) -> Vec<Bonus>;
    fn remove_bonuses(&self) -> Vec<Bonus>;
}
