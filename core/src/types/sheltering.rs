use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusSource, BonusType,
    },
    traits::IterValues,
    val,
};

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Clone,
    Copy,
    derive_more::Display,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum Sheltering {
    #[display("Physical Sheltering")]
    Physical,
    #[display("Magical Sheltering")]
    Magical,
    #[display("Magical Sheltering Cap")]
    MagicalCap,
    #[display("Magical Sheltering Uncapped Bonus")]
    MagicalUncapped,
    #[display("PRR Damage Reduction")]
    PhysicalReduction,
    #[display("MRR Damage Reduction")]
    MagicalReduction,
}

impl Sheltering {
    pub const VALUES: [Self; 6] = [
        Self::Physical,
        Self::Magical,
        Self::MagicalCap,
        Self::MagicalUncapped,
        Self::PhysicalReduction,
        Self::MagicalReduction,
    ];

    #[must_use]
    pub fn core_bonuses() -> impl IntoIterator<Item = Bonus> {
        [
            Bonus::new(
                Self::MagicalReduction,
                {
                    let magical = Self::Magical.attribute().to_value();
                    let magical_cap = Self::MagicalCap.attribute().to_value();
                    let magical_uncapped = Self::MagicalUncapped.attribute().to_value();
                    let capped_magical = magical.min(magical_cap);
                    let total_magical = capped_magical + magical_uncapped;

                    val!(1) - (val!(100) / (val!(100) + total_magical))
                },
                BonusType::Stacking,
                BonusSource::Custom("Sheltering".to_owned()),
            ),
            Bonus::new(
                Self::PhysicalReduction,
                val!(1) - (val!(100) / (val!(100) + Self::Physical.attribute().to_value())),
                BonusType::Stacking,
                BonusSource::Custom("Sheltering".to_owned()),
            ),
        ]
    }
}

impl IterValues for Sheltering {
    fn values() -> impl Iterator<Item = Self> {
        Self::VALUES.into_iter()
    }
}
