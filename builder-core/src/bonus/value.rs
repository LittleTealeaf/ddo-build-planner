use crate::attribute::Attribute;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BonusValue {
    Value(f32),
    Indirect(Attribute),
    IndirectScaled(Attribute, f32),
}

impl BonusValue {
    pub fn get_dependencies(&self) -> Option<Vec<Attribute>> {
        match self {
            Self::IndirectScaled(attr, _) | Self::Indirect(attr) => Some(vec![*attr]),
            _ => None,
        }
    }
}

impl From<f32> for BonusValue {
    fn from(value: f32) -> BonusValue {
        BonusValue::Value(value)
    }
}

impl From<Attribute> for BonusValue {
    fn from(value: Attribute) -> Self {
        BonusValue::Indirect(value)
    }
}

impl From<(Attribute, f32)> for BonusValue {
    fn from((attribute, scale): (Attribute, f32)) -> Self {
        BonusValue::IndirectScaled(attribute, scale)
    }
}
