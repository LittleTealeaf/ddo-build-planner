use self::{heroic::HeroicFeat, epic::EpicFeat};

pub mod heroic;
pub mod scion;
pub mod epic;
pub mod destiny;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Feat {
    Heroic(HeroicFeat),
    Epic(EpicFeat),
}

