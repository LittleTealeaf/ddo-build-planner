use crate::simple_enum;


simple_enum!(
    Health,
    (Base "Base Hit Points", BaseScalar "Base Hit Point Scalar", Bonus "Bonus Hit Points", Scalar "Hit Point Scalar")
);
