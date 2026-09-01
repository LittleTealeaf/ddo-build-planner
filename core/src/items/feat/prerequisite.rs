use rust_decimal::Decimal;

use crate::types::ability::Ability;

#[derive(
    Debug,
    Clone,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
)]
pub enum Prerequisite {
    #[display("{_1} {_0} Score")]
    AbilityScore(Ability, Decimal),
}
