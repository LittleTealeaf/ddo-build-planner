use crate::{attribute::Attribute, simple_enum};

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
    #[inline(always)]
    fn from(value: Tactics) -> Self {
        Attribute::Tactics(value)
    }
}
