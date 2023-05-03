use crate::build::attribute::Attribute;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Condition {
    Has(Attribute)
}
