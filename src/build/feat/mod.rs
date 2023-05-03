use self::{heroic::HeroicFeat, epic::EpicFeat};

pub mod heroic;
pub mod scion;
pub mod epic;
pub mod destiny;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Feat {
    Heroic(HeroicFeat),
    Epic(EpicFeat),
}

