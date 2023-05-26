use std::fmt::Display;

use crate::bonus::Bonus;

pub trait AttributeTrait: Display {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>>;
}


pub trait GetBonuses<T = ()> {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>>;
}
