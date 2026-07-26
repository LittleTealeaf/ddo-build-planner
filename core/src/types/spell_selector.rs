use itertools::chain;

use crate::{
    attribute::Attribute,
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
    #[display("{_0} School")]
    School(SpellSchool),
    #[display("{_0} Spells")]
    Power(SpellPower),
}

impl SpellSelector {
    #[must_use]
    pub const fn caster_level(self) -> Attribute {
        Attribute::CasterLevel(self)
    }

    #[must_use]
    pub const fn spell_dc(self) -> Attribute {
        Attribute::SpellDC(self)
    }
}

impl IterValues for SpellSelector {
    fn values() -> impl Iterator<Item = Self> {
        chain!(
            SpellSchool::values().map(Self::School),
            SpellPower::values().map(Self::Power),
        )
    }
}
