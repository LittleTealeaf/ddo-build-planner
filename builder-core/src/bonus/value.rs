

pub enum BonusValue {
    Value(f32)
}

impl From<f32> for BonusValue {
    fn from(value: f32) -> BonusValue {
        BonusValue::Value(value)
    }
}
