use crate::{player_class::PlayerClass, simple_enum};

simple_enum!(
    SpellType,
    "",
    (
        Arcane "Arcane", Divine "Divine"
    )
);

impl SpellType {
    /// Converts the spell type to each of the player classes represented by that type.
    pub fn to_player_classes(&self) -> Option<Vec<PlayerClass>> {
        match self {
            Self::Arcane => Some(vec![
                PlayerClass::Wizard,
                PlayerClass::Sorcerer,
                PlayerClass::Ranger,
                PlayerClass::DarkHunter,
            ]),
            _ => None,
        }
    }
}
