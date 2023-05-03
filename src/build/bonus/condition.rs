use crate::build::attribute::Attribute;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Condition {
    Has(Attribute)
}
