use crate::{attribute::GetCloned, simple_enum};

simple_enum!(ThreatType, "", (Ranged "Ranged", Melee "Melee", Spell "Spell", All "All"));

impl GetCloned<ThreatType> for ThreatType {
    fn get_cloned(&self) -> Option<Vec<ThreatType>> {
        if matches!(self, Self::All) {
            Some(vec![Self::Ranged, Self::Melee, Self::Spell])
        } else {
            None
        }
    }
}
