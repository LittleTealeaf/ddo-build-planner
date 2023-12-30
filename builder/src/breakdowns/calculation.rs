use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    bonus::{Condition, Value},
};

use super::Breakdowns;

impl Breakdowns {
    pub(super) fn evaluate_condition(&mut self, condition: &Condition) -> bool {
        todo!()
    }

    pub(super) fn evaluate_value(&mut self, value: &Value) -> Decimal {
        todo!()
    }

    /// Calculates and retuns the final value for a given [`Attribute`].
    pub fn get_attribute(&mut self, attribute: &Attribute) -> Decimal {
        todo!()
    }

    pub(crate) fn calculate_attribute(&mut self, attribute: Attribute) -> Option<Decimal> {
        todo!()
    }
}
