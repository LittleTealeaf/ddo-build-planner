use crate::{simple_enum, attribute::Attribute};

simple_enum!(
    Tactics, "", (
        Assassinate "Assassinate",
        Stun "Stun",
        Sunder "Sunder",
        Trip "Trip",
        RuneArm "Rune Arm"
    )
);

impl From<Tactics> for Attribute {
    fn from(value: Tactics) -> Self {
        Attribute::Tactics(value)
    }
}
