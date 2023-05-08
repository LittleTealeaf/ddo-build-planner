use super::Bonus;

// TODO: Move this to somewhere more thoughtful
pub trait Bonuses {
    fn get_bonuses(&self) -> Vec<Bonus>;
}
