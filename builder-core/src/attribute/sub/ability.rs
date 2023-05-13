use crate::{attribute::Attribute, simple_enum};

simple_enum!(Ability, (Strength "Strength", Dexterity "Dexterity", Constitution "Constitution", Intelligence "Intelligence", Wisdom "Wisdom", Charisma "Charisma", All "All"));

impl Ability {
    pub fn get_cloned_abilities(&self) -> Option<Vec<Ability>> {
        if let Self::All = self {
            Some(vec![
                Self::Strength,
                Self::Dexterity,
                Self::Constitution,
                Self::Intelligence,
                Self::Wisdom,
                Self::Charisma,
            ])
        } else {
            None
        }
    }
}
