use crate::attribute::Attribute;



pub enum BonusSource {
    Attribute(Attribute),
}

impl From<Attribute> for BonusSource {
   fn from(value: Attribute) -> Self {
       Self::Attribute(value)
   } 
}
