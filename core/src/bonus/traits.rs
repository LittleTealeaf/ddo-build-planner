use crate::{attribute::Attribute, bonus::BonusValue};

pub trait ToValue {
    fn to_value(self) -> BonusValue;
}

impl<T> ToValue for T
where
    T: Into<BonusValue>,
{
    fn to_value(self) -> BonusValue {
        self.into()
    }
}

pub trait ToAttribute {
    fn attribute(self) -> Attribute;
}

impl<T> ToAttribute for T
where
    T: Into<Attribute>,
{
    fn attribute(self) -> Attribute {
        self.into()
    }
}

pub trait ContainsAttribute {
    fn contains_attribute(&self, attribute: &Attribute) -> bool;
}
