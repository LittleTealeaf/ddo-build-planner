use crate::build::attribute::Attribute;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Condition {
    Has(Attribute)
}
