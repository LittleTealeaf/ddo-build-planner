use crate::logic::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum SavingThrow {
    Fortitude,
    Poison,
    Disease,
    Reflex,
    Traps,
    Spell,
    Magic,
    Will,
    Enchantment,
    Illusion,
    Fear,
    Curse,
}

impl ToString for SavingThrow {
    fn to_string(&self) -> String {
        String::from(match self {
            SavingThrow::Fortitude => "Fortitude",
            SavingThrow::Poison => "Poison",
            SavingThrow::Disease => "Disease",
            SavingThrow::Reflex => "Reflex",
            SavingThrow::Traps => "Traps",
            SavingThrow::Spell => "Spells",
            SavingThrow::Magic => "Magic",
            SavingThrow::Will => "Will",
            SavingThrow::Enchantment => "Enchantment",
            SavingThrow::Illusion => "Illusion",
            SavingThrow::Fear => "Fear",
            SavingThrow::Curse => "Curse",
        })
    }
}

macro_rules! child_saving_throw {
    ($parent: ident, $child: ident, $value: expr) => {
        Bonus::new(
            Attribute::SavingThrow(SavingThrow::$child),
            BonusType::Stacking,
            $value,
            BonusSource::Attribute(Attribute::SavingThrow(SavingThrow::$parent)),
            None,
        )
    };
}

impl SavingThrow {
    pub fn get_attribute_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        match self {
            SavingThrow::Fortitude => Some(vec![
                child_saving_throw!(Fortitude, Poison, value),
                child_saving_throw!(Fortitude, Disease, value),
            ]),
            SavingThrow::Reflex => Some(vec![
                child_saving_throw!(Reflex, Traps, value),
                child_saving_throw!(Reflex, Spell, value),
                child_saving_throw!(Reflex, Magic, value),
            ]),
            SavingThrow::Will => Some(vec![
                child_saving_throw!(Will, Enchantment, value),
                child_saving_throw!(Will, Illusion, value),
                child_saving_throw!(Will, Fear, value),
                child_saving_throw!(Will, Curse, value),
            ]),
            _ => None,
        }
    }
}

impl From<SavingThrow> for Attribute {
    fn from(value: SavingThrow) -> Self {
        Attribute::SavingThrow(value)
    }
}
