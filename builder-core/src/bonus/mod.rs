//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod bonus_type;
mod condition;
mod source;
mod value;

use crate::attribute::Attribute;

pub use bonus_type::*;
pub use condition::*;
pub use source::*;
pub use value::*;

pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: BonusValue,
    source: BonusSource,
    condition: Option<Condition>,
}

impl Bonus {
    pub fn new(
        attribute: Attribute,
        bonus_type: BonusType,
        value: BonusValue,
        source: BonusSource,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            attribute,
            bonus_type,
            value,
            source,
            condition,
        }
    }
}
