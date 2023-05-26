//! A Bonus is an individual bonus to an attribute, increasing or decreasing it by a certain amount.
mod condition;
mod bonus_type;
mod value;
mod source;

use crate::attribute::Attribute;

pub use condition::*;
pub use bonus_type::*;
pub use value::*;
pub use source::*;

pub struct Bonus {
    attribute: Attribute,
    bonus_type: BonusType,
    value: BonusValue,
    source: BonusSource,
    condition: Option<Condition>,
}

