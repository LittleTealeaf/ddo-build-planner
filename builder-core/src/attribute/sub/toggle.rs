use crate::{
    attribute::{Attribute, GetBonuses},
    bonus::Bonus,
    simple_enum,
};

simple_enum!(
    Toggle, "", (
        Blocking() String::from("Blocking"),
        InReaper() String::from("In Reaper"),
        AttackingTrippedTarget() String::from("Attacking Tripped Targets")
    )
);

impl GetBonuses for Toggle {
    fn get_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        None
    }
}

impl From<Toggle> for Attribute {
    #[inline(always)]
    fn from(value: Toggle) -> Attribute {
        Attribute::Toggle(value)
    }
}
