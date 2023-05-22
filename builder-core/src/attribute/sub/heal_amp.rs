use crate::{
    attribute::{Attribute, GetCloned},
    simple_enum,
};

simple_enum!(HealAmp, "", (Positive "Positive", Negative "Negative", Repair "Repair", All "All"));

impl GetCloned<HealAmp> for HealAmp {
    fn get_cloned(&self) -> Option<Vec<HealAmp>> {
        match self {
            Self::All => Some(vec![Self::Positive, Self::Negative, Self::Repair]),
            _ => None,
        }
    }
}

impl From<HealAmp> for Attribute {
    #[inline(always)]
    fn from(value: HealAmp) -> Self {
        Attribute::HealAmp(value)
    }
}
