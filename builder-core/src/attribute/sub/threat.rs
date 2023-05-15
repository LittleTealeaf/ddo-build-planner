use crate::simple_enum;

simple_enum!(ThreatType, "", (Ranged "Ranged", Melee "Melee", Spell "Spell", All "All"));

impl ThreatType {
    pub fn get_cloned_types(&self) -> Option<Vec<ThreatType>> {
        if let Self::All = self {
            Some(vec![Self::Ranged, Self::Melee, Self::Spell])
        } else {
            None
        }
    }
}
