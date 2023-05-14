use crate::{bonus::Bonus, simple_enum};

simple_enum!(
    Toggle, (
        Blocking() String::from("Blocking")
    )
);

impl Toggle {
    pub fn get_toggled_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            _ => None,
        }
    }
}
