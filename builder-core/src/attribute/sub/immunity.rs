use crate::{attribute::Attribute, simple_enum};

simple_enum!(
    Immunity,
    "Indicates that the player is immune to a certain thing",
    (
        MagicMissile() String::from("Magic Missile"),
        EnergyDrain() String::from("Energy Drain"),
        MummyRot() String::from("Mummy Rot"),
        NaturalDisease() String::from("Natural Disease"),
        SlipperySurfaces() String::from("Slippery Surfaces"),
        Knockdown() String::from("Knockdown"),
        Quell() String::from("Quell"),
        Petrification() String::from("Petrification"),
        MostSlowForms() String::from("Most Slow Forms")
    )
);

impl From<Immunity> for Attribute {
    #[inline(always)]
    fn from(value: Immunity) -> Self {
        Attribute::Flag(value.into())
    }
}
