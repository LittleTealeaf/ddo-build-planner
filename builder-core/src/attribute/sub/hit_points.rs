use crate::{simple_enum, attribute::Attribute};

simple_enum!(HitPoint, "", (Base "Base Hit Points", BaseScalar "Base Scalar Hit Points", Bonus "Hit Points", Scalar "Scalar Hit Points"));

impl From<HitPoint> for Attribute {
    #[inline(always)]
    fn from(value: HitPoint) -> Self {
        Attribute::HitPoints(value)
    }
}
