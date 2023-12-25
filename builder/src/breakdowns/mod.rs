//! Pulls together all the bonuses and calculates the bonuses for each attribute

mod calculation;
pub use calculation::*;

mod inserting;
pub use inserting::*;

mod buffer;

use std::collections::HashMap;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource},
};


/// TODO: Documentation
pub struct Breakdowns {
    bonuses: HashMap<Attribute, Vec<Bonus>>,
    cache: HashMap<Attribute, f32>,
    children: HashMap<BonusSource, Vec<Attribute>>,
}
