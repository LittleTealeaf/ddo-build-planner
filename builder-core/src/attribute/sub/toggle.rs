use crate::{bonus::Bonus, simple_enum};

simple_enum!(
    Toggle, (
        Blocking() String::from("Blocking"),
        InReaper() String::from("In Reaper"),
        AttackingTrippedTarget() String::from("Attacking Tripped Targets")
    )
);

impl Toggle {
    pub fn get_toggled_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}
