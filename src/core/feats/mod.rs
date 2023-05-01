use self::heroic::HeroicFeat;

use super::effects::Effects;

pub mod heroic;
pub mod tomes;

pub enum Feat {
    Heroic(HeroicFeat),
}
