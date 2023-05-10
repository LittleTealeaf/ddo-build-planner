use crate::logic::bonus::{Bonus, Bonuses};

pub trait FeatTrait: ToString + Bonuses {
    fn get_feat_bonuses(&self, value: f32) -> Vec<Bonus>;
    fn get_name(&self) -> String {
        self.to_string()
    }
    fn get_description(&self) -> String;
}
