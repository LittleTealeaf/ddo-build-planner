use crate::bonus::BonusValue;

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
