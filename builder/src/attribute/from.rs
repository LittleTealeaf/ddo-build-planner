use crate::{
    feat::{Feat, Proficiency},
    player_class::PlayerClass,
    race::Race,
    types::{Ability, Skill, SavingThrow},
};

use super::{
    bonuses::{ArmorClass, Immunity, Sheltering, WeaponAttribute},
    flags::Flag,
    toggles::Toggle,
    Attribute,
};

impl From<&Self> for Attribute {
    fn from(value: &Self) -> Self {
        *value
    }
}

impl From<Flag> for Attribute {
    fn from(value: Flag) -> Self {
        Self::Flag(value)
    }
}

impl From<PlayerClass> for Attribute {
    fn from(value: PlayerClass) -> Self {
        Self::ClassLevel(value)
    }
}

impl From<Toggle> for Attribute {
    fn from(value: Toggle) -> Self {
        Self::Toggle(value)
    }
}

impl From<Ability> for Attribute {
    fn from(value: Ability) -> Self {
        Self::Ability(value)
    }
}

impl From<ArmorClass> for Attribute {
    fn from(value: ArmorClass) -> Self {
        Self::ArmorClass(value)
    }
}

impl From<Immunity> for Attribute {
    fn from(value: Immunity) -> Self {
        Flag::from(value).into()
    }
}

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Self::SavingThrow(value)
    }
}

impl From<Sheltering> for Attribute {
    fn from(value: Sheltering) -> Self {
        Self::Sheltering(value)
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Self::Skill(value)
    }
}

impl From<WeaponAttribute> for Attribute {
    fn from(value: WeaponAttribute) -> Self {
        Self::Weapon(value)
    }
}

impl From<Proficiency> for Attribute {
    fn from(value: Proficiency) -> Self {
        Feat::from(value).into()
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Self::Feat(value)
    }
}

impl From<Race> for Attribute {
    fn from(value: Race) -> Self {
        Flag::from(value).into()
    }
}
