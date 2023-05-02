use self::heroic::HeroicFeat;

pub mod heroic;

#[derive(Clone, Copy)]
pub enum Feat {
    Heroic(HeroicFeat),
}

