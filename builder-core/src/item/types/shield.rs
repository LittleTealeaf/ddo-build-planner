use enum_map::Enum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize, Debug, Enum)]
pub enum ShieldType {
    Buckler,
    Small,
    Large,
    Tower,
    RuneArm
}

impl ToString for ShieldType {
    fn to_string(&self) -> String {
        match self {
            ShieldType::Buckler => String::from("Buckler"),
            ShieldType::Small => String::from("Small"),
            ShieldType::Large => String::from("Large"),
            ShieldType::Tower => String::from("Tower"),
            ShieldType::RuneArm => String::from("Rune Arm"),
        }
    }
}