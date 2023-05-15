use crate::simple_enum;

simple_enum!(HealAmp, (Positive "Positive", Negative "Negative", Repair "Repair", All "All"));

impl HealAmp {
    pub fn get_cloned_attributes(&self) -> Option<Vec<HealAmp>> {
        match self {
            Self::All => Some(vec![Self::Positive, Self::Negative, Self::Repair]),
            _ => None,
        }
    }
}
