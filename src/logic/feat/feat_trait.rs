use crate::logic::bonus::Bonus;

pub trait FeatTrait: ToString {
    fn get_feat_bonuses(&self, value: f32) -> Vec<Bonus>;
    fn get_name(&self) -> String {
        self.to_string()
    }
    fn get_description(&self) -> String;
}
