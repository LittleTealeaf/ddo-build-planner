use rust_decimal::Decimal;

use crate::types::ability::Ability;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Prerequisite {
    Feat(String),
    Ability(Ability, Decimal),
}
