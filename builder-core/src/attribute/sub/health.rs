use crate::simple_enum;


simple_enum!(
    Health,
    (Base "Base Hit Points", BaseScalar "Base Hit Point Scalar", BonusPoints "Bonus Hit Points", HitPointScalar "Hit Point Scalar")
);
