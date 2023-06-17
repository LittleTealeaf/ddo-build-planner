use serde::{Deserialize, Serialize};

/// Describes the material of a parituclar item
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ItemMaterial {
    /// Adamantine
    Adamantine,
    /// Byeshk
    Byeshk,
    /// BlueDragonscale
    BlueDragonscale,
    /// BlackDragonscale
    BlackDragonscale,
    /// Blueshine
    Blueshine,
    /// Cloth
    Cloth,
    /// ColdIron
    ColdIron,
    /// Copper
    Copper,
    /// Crystal
    Crystal,
    /// Darkleaf
    Darkleaf,
    /// Darkweave
    Darkweave,
    /// Darkwood
    Darkwood,
    /// DwarvenIron
    DwarvenIron,
    /// Elven
    Elven,
    /// FlametouchedIron
    FlametouchedIron,
    /// Force
    Force,
    /// Gem
    Gem,
    /// Glass
    Glass,
    /// GreenSteel
    GreenSteel,
    /// GreenDragonscale
    GreenDragonscale,
    /// Ice
    Ice,
    /// Light
    Light,
    /// Metalline
    Metalline,
    /// Mithral
    Mithral,
    /// PlaneforgedStel
    PlaneforgedStel,
    /// Platinum
    Platinum,
    /// RedDragonscale
    RedDragonscale,
    /// Rust
    Rust,
    /// Silver
    Silver,
    /// SpiritcraftLeather
    SpiritcraftLeather,
    /// Steel
    Steel,
    /// Stone
    Stone,
    /// WhiteDragonscale
    WhiteDragonscale,
    /// Wood
    Wood,
    /// Unknown
    Unknown,
}
