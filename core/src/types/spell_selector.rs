use itertools::chain;

use crate::{
    traits::IterValues,
    types::{spell_power::SpellPower, spell_school::SpellSchool},
};

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::From,
)]
pub enum SpellSelector {
    School(SpellSchool),
    Power(SpellPower),
}

impl IterValues for SpellSelector {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            SpellSchool::values().map(Self::School),
            SpellPower::values().map(Self::Power),
        )
    }
}
