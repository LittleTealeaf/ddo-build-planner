use std::fmt::Display;

use enum_map::Enum;

use crate::{item::types::WeaponType, attribute::Attribute};

use super::Feat;



#[derive(Clone, Copy, PartialEq, Eq, Debug, Enum)]
pub enum Proficiency {
    WeaponProficiency(WeaponType),
}

impl Display for Proficiency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Proficiency::WeaponProficiency(weapon) => write!(f, "{} Proficiency", weapon),
        }
    }
}

impl From<WeaponType> for Proficiency {
    fn from(value: WeaponType) -> Self {
        Self::WeaponProficiency(value)
    }
}


impl From<Proficiency> for Attribute {
    fn from(value: Proficiency) -> Self {
        Feat::from(value).into()
    }
}
