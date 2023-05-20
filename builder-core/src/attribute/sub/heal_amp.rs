use crate::{simple_enum, attribute::{GetCloned, Attribute}};

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
    fn from(value: HealAmp) -> Self {
        Attribute::HealAmp(value)
    }
}
