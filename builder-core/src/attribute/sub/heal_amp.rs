use crate::{simple_enum, attribute::GetCloned};

simple_enum!(HealAmp, "", (Positive "Positive", Negative "Negative", Repair "Repair", All "All"));

impl GetCloned<HealAmp> for HealAmp {
    fn get_cloned(&self) -> Option<Vec<HealAmp>> {
        match self {
            Self::All => Some(vec![Self::Positive, Self::Negative, Self::Repair]),
            _ => None,
        }
    }
}
