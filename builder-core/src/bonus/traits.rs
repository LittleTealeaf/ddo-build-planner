use super::Bonus;

pub trait CloneBonus {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>>;
}
