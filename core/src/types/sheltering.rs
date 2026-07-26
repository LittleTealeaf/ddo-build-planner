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
