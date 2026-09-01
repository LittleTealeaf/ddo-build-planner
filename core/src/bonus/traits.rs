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
    fn any_attribute<'a, F>(&'a self, fun: &F) -> bool
    where
        F: Fn(&'a Attribute) -> bool;

    fn contains_attribute(&self, attribute: &Attribute) -> bool {
        self.any_attribute(&|attr| attr == attribute)
    }
}
