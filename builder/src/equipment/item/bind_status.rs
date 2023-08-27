use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Default)]
pub enum ItemBindStatus {
    #[default]
    Account,
    Character,
    CharacterOnEquip,
    AccountOnEquip,
}

impl Display for ItemBindStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemBindStatus::Character => write!(f, "Bound to Character"),
            ItemBindStatus::Account => write!(f, "Bound to Account"),
            ItemBindStatus::CharacterOnEquip => write!(f, "Bound to Character on Equip"),
            ItemBindStatus::AccountOnEquip => write!(f, "Bound to Account on Equip"),
        }
    }
}
