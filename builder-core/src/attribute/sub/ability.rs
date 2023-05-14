use crate::{attribute::Attribute, bonus::Bonus, simple_enum};

simple_enum!(Ability, (Strength "Strength", Dexterity "Dexterity", Constitution "Constitution", Intelligence "Intelligence", Wisdom "Wisdom", Charisma "Charisma", All "All"));

macro_rules! modifier_skill {
    ($modifier: ident, $skill: ident, $value: expr) => {
        Bonus::new(
            $crate::attribute::Attribute::Skill($crate::attribute::Skill::$skill),
            $crate::bonus::BonusType::AbilityModifier,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::AbilityModifier(
                Ability::$modifier,
            )),
            None,
        )
    };
}

macro_rules! modifier_saving_throw {
    ($modifier: ident, $saving_throw: ident, $value: expr, $def: expr) => {
        Bonus::new(
            $crate::attribute::Attribute::SavingThrow(
                $crate::attribute::SavingThrow::$saving_throw,
            ),
            $crate::bonus::BonusType::AbilityModifier,
            $value,
            $crate::bonus::BonusSource::Attribute($crate::attribute::Attribute::AbilityModifier(
                Ability::$modifier,
            )),
            if $def {
                None
            } else {
                Some(vec![$crate::bonus::Condition::Has(
                    $crate::attribute::Attribute::Flag(
                        $crate::attribute::Flag::AbilityToSavingThrow(
                            Ability::$modifier,
                            $crate::attribute::SavingThrow::$saving_throw,
                        ),
                    ),
                )])
            },
        )
    };
}

impl Ability {
    pub fn get_cloned_abilities(&self) -> Option<Vec<Ability>> {
        if let Self::All = self {
            Some(vec![
                Self::Strength,
                Self::Dexterity,
                Self::Constitution,
                Self::Intelligence,
                Self::Wisdom,
                Self::Charisma,
            ])
        } else {
            None
        }
    }

    pub fn get_modifier_bonuses(&self, value: f32) -> Option<Vec<Bonus>> {
        let mut values = vec![];

        values.append(&mut match self {
            Self::Strength => Some(vec![
                modifier_skill!(Strength, Jump, value),
                modifier_skill!(Strength, Swim, value),
                modifier_saving_throw!(Strength, Reflex, value, false),
                modifier_saving_throw!(Strength, Fortitude, value, false),
                modifier_saving_throw!(Strength, Will, value, false),
            ]),
            Self::Dexterity => Some(vec![
                modifier_skill!(Dexterity, Balance, value),
                modifier_skill!(Dexterity, Hide, value),
                modifier_skill!(Dexterity, MoveSilently, value),
                modifier_skill!(Dexterity, OpenLock, value),
                modifier_skill!(Dexterity, Tumble, value),
                modifier_saving_throw!(Dexterity, Reflex, value, true),
                modifier_saving_throw!(Dexterity, Fortitude, value, false),
                modifier_saving_throw!(Dexterity, Will, value, false),
            ]),
            Self::Constitution => Some(vec![
                modifier_skill!(Constitution, Concentration, value),
                modifier_saving_throw!(Constitution, Reflex, value, false),
                modifier_saving_throw!(Constitution, Fortitude, value, true),
                modifier_saving_throw!(Constitution, Will, value, false),
            ]),
            Self::Intelligence => Some(vec![
                modifier_skill!(Intelligence, DisableDevice, value),
                modifier_skill!(Intelligence, Repair, value),
                modifier_skill!(Intelligence, Search, value),
                modifier_skill!(Intelligence, SpellCraft, value),
                modifier_saving_throw!(Intelligence, Reflex, value, false),
                modifier_saving_throw!(Intelligence, Fortitude, value, false),
                modifier_saving_throw!(Intelligence, Will, value, false),
            ]),
            Self::Wisdom => Some(vec![
                modifier_skill!(Wisdom, Heal, value),
                modifier_skill!(Wisdom, Listen, value),
                modifier_skill!(Wisdom, Spot, value),
                modifier_saving_throw!(Wisdom, Reflex, value, false),
                modifier_saving_throw!(Wisdom, Fortitude, value, false),
                modifier_saving_throw!(Wisdom, Will, value, true),
            ]),
            Self::Charisma => Some(vec![
                modifier_skill!(Charisma, Bluff, value),
                modifier_skill!(Charisma, Diplomacy, value),
                modifier_skill!(Charisma, Haggle, value),
                modifier_skill!(Charisma, Intimidate, value),
                modifier_skill!(Charisma, Perform, value),
                modifier_skill!(Charisma, UseMagicalDevice, value),
                modifier_saving_throw!(Charisma, Reflex, value, false),
                modifier_saving_throw!(Charisma, Fortitude, value, false),
                modifier_saving_throw!(Charisma, Will, value, false),
            ]),
            Self::All => None,
        }?);

        Some(values)
    }
}
