use super::{Attributable, Attribute};

pub enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Attributable for Ability {
    fn into_attribute(self) -> Attribute {
        Attribute::Ability(self)
    }
}
