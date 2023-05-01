use super::Bonus;

pub trait Effects {
    fn get_bonuses(&self) -> Vec<Bonus>;
}
