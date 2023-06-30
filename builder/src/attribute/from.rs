use crate::{
    feat::{Feat, Proficiency},
    player_class::PlayerClass,
    race::Race,
};

use super::{
    flags::Flag,
    toggles::Toggle,
    types::{Ability, ArmorClass, Immunity, SavingThrow, Sheltering, Skill, WeaponAttribute},
    Attribute,
};

impl From<&Attribute> for Attribute {
    fn from(value: &Attribute) -> Self {
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
        Attribute::Ability(value)
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
        Attribute::SavingThrow(value)
    }
}

impl From<Sheltering> for Attribute {
    fn from(value: Sheltering) -> Self {
        Self::Sheltering(value)
    }
}

impl From<Skill> for Attribute {
    fn from(value: Skill) -> Self {
        Attribute::Skill(value)
    }
}

impl From<WeaponAttribute> for Attribute {
    fn from(value: WeaponAttribute) -> Self {
        Attribute::Weapon(value)
    }
}

impl From<Proficiency> for Attribute {
    fn from(value: Proficiency) -> Self {
        Feat::from(value).into()
    }
}

impl From<Feat> for Attribute {
    fn from(value: Feat) -> Self {
        Attribute::Feat(value)
    }
}

impl From<Race> for Attribute {
    fn from(value: Race) -> Self {
        Flag::from(value).into()
    }
}
