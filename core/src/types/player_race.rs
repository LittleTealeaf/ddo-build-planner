use crate::traits::IterValues;



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
    strum::VariantArray
)]
pub enum PlayerRace {
    Human,
    Elf,
    #[display("Wood Elf")]
    WoodElf,
    Halfling,
    Dwarf,
    Dragonborn,
    #[display("Drow Elf")]
    DrowElf,
    #[display("Deep Gnome")]
    DeepGnome,
    Gnome,
    #[display("Half-Elf")]
    HalfElf,
    #[display("Half-Orc")]
    HalfOrc,
    Tiefling,
    Aasimar,
    Dhampir,
    Duergar,
    Eladrin,
    Shifter,
    Tabaxi,
    Warforged,
    #[display("Aasimar Scourge")]
    AasimarScourge,
    Bladeforged,
    Chaosmancer,
    #[display("Dark Bargainer")]
    DarkBargainer,
    #[display("Deep Gnome")]
    DeepGnome,
    #[display("Duergar Mindcleaver")]
    DuergarMindcleaver,
    #[display("Morninglord")]
    Morninglord,
    #[display("Purple Dragon Knight")]
    PurpleDragonKnight,
    #[display("Razorclaw Shifter")]
    RazorclawShifter,
    Scoundrel,
    ShadarKai,
    #[display("Tabaxi Trailblazer")]
    TabaxiTrailblazer
}

impl IterValues for PlayerRace {
    fn values() -> impl Iterator<Item = Self> {
        Self::VARIANTS.into_iter()
    }
}
