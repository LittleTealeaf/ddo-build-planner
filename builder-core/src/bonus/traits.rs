use super::Bonus;

#[deprecated = "Replaced by GetBonuses"]
pub trait Bonuses {
    fn get_bonuses(&self) -> Vec<Bonus>;
    fn remove_bonuses(&self) -> Vec<Bonus>;
}
