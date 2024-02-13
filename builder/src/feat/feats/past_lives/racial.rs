use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, BonusType, Condition},
    types::{
        ability::Ability,
        damage_type::DamageType,
        dodge::Dodge,
        flag::ToFlag,
        race::Race,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        spell_power::SpellPower,
        spell_school::SpellSchool,
        spell_selector::SpellSelector,
        toggle::{IconicPastLife, ToToggle},
        weapon_attribute::{WeaponAttribute, WeaponHand, WeaponStat},
    },
};

pub fn racial_past_lives(race: Race, value: Decimal) -> Option<Vec<BonusTemplate>> {
    (value > Decimal::ZERO).then(|| {
        let value = value.max(dec!(3));

        match race {
            Race::Dragonborn | Race::Tiefling => {
                heroic_past_lives(Skill::Spellcraft, Ability::Charisma, value)
            }
            Race::Drow => heroic_past_lives(Skill::Search, Ability::Intelligence, value),
            Race::Dwarf => heroic_past_lives(Skill::Balance, Ability::Constitution, value),
            Race::Gnome => heroic_past_lives(Skill::UseMagicalDevice, Ability::Intelligence, value),
            Race::Halfling => heroic_past_lives(Skill::MoveSilently, Ability::Dexterity, value),
            Race::HalfElf => heroic_past_lives(Skill::Diplomacy, Ability::Charisma, value),
            Race::HalfOrc => heroic_past_lives(Skill::Intimidate, Ability::Strength, value),
            Race::Human => heroic_past_lives(Skill::Haggle, Ability::Wisdom, value),
            Race::Warforged => heroic_past_lives(Skill::Repair, Ability::Constitution, value),
            Race::Aasimar => heroic_past_lives(Skill::Heal, Ability::Wisdom, value),
            Race::Tabaxi => heroic_past_lives(Skill::Tumble, Ability::Dexterity, value),
            Race::Shifter | Race::WoodElf | Race::Elf => {
                heroic_past_lives(Skill::Spot, Ability::Dexterity, value)
            }
            Race::Scourge => vec![
                BonusTemplate::new(SavingThrow::Fortitude, BonusType::Stacking, value, None),
                BonusTemplate::new(
                    IconicPastLife(Race::Scourge).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::DoubleStrike,
                    BonusType::Stacking,
                    value * Decimal::TWO,
                    Condition::has(IconicPastLife(Race::Scourge).to_toggle()),
                ),
            ],
            Race::Bladeforged => vec![
                BonusTemplate::new(
                    Attribute::Fortification,
                    BonusType::Stacking,
                    dec!(5) * value,
                    None,
                ),
                BonusTemplate::new(
                    IconicPastLife(Race::Bladeforged).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::SpellPower(SpellPower::Damage(DamageType::Repair)),
                    BonusType::Stacking,
                    Decimal::TEN * value,
                    Condition::has(IconicPastLife(Race::Bladeforged).to_toggle()),
                ),
            ],
            Race::DeepGnome => vec![
                BonusTemplate::new(
                    Sheltering::Magical,
                    BonusType::Stacking,
                    dec!(3) * value,
                    None,
                ),
                BonusTemplate::new(
                    IconicPastLife(Race::DeepGnome).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::SpellDC(SpellSelector::School(SpellSchool::Illusion)),
                    BonusType::Stacking,
                    value,
                    Condition::has(IconicPastLife(Race::DeepGnome).to_toggle()),
                ),
                BonusTemplate::new(
                    Attribute::SpellPower(SpellPower::Damage(DamageType::Acid)),
                    BonusType::Stacking,
                    value * dec!(5),
                    Condition::has(IconicPastLife(Race::DeepGnome).to_toggle()),
                ),
            ],
            Race::PurpleDragonKnight => vec![
                BonusTemplate::new(
                    Sheltering::Physical,
                    BonusType::Stacking,
                    value * dec!(3),
                    None,
                ),
                BonusTemplate::new(
                    IconicPastLife(Race::PurpleDragonKnight).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
            ],
            Race::Razorclaw => vec![
                BonusTemplate::new(SavingThrow::Will, BonusType::Stacking, value, None),
                BonusTemplate::new(
                    IconicPastLife(Race::Razorclaw).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                BonusTemplate::new(
                    WeaponAttribute(WeaponHand::Both, WeaponStat::Attack),
                    BonusType::Stacking,
                    value,
                    Condition::has(IconicPastLife(Race::Razorclaw).to_toggle()),
                ),
                BonusTemplate::new(
                    WeaponAttribute(WeaponHand::Both, WeaponStat::Damage),
                    BonusType::Stacking,
                    value,
                    Condition::has(IconicPastLife(Race::Razorclaw).to_toggle()),
                ),
            ],
            Race::Shadarkai => vec![
                BonusTemplate::new(Dodge::Bonus, BonusType::Stacking, value, None),
                BonusTemplate::new(
                    IconicPastLife(Race::Shadarkai).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
            ],
            Race::Morninglord => vec![
                BonusTemplate::new(
                    Attribute::SpellPower(SpellPower::Damage(DamageType::Positive)),
                    BonusType::Stacking,
                    value * dec!(3),
                    None,
                ),
                BonusTemplate::new(
                    IconicPastLife(Race::Morninglord).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                BonusTemplate::new(
                    Attribute::SpellPower(SpellPower::Damage(DamageType::Light)),
                    BonusType::Stacking,
                    Decimal::TEN * value,
                    Condition::has(IconicPastLife(Race::Morninglord).to_toggle()),
                ),
                BonusTemplate::new(
                    Attribute::SpellPower(SpellPower::Damage(DamageType::Alignment)),
                    BonusType::Stacking,
                    Decimal::TEN * value,
                    Condition::has(IconicPastLife(Race::Morninglord).to_toggle()),
                ),
            ],
            Race::Trailblazer => vec![
                BonusTemplate::new(SavingThrow::Traps, BonusType::Stacking, value, None),
                BonusTemplate::new(
                    IconicPastLife(Race::Trailblazer).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                // TODO: Add Attribute for Tactics
            ],
            Race::Scoundrel => vec![
                BonusTemplate::new(SavingThrow::Reflex, BonusType::Stacking, value, None),
                BonusTemplate::new(
                    IconicPastLife(Race::Scoundrel).to_flag(),
                    BonusType::Stacking,
                    1,
                    None,
                ),
                // TODO: Add Attribute for Movement Speed
            ],
        }
    })
}

fn heroic_past_lives(skill: Skill, ability: Ability, value: Decimal) -> Vec<BonusTemplate> {
    [
        (value >= Decimal::ONE)
            .then(|| BonusTemplate::new(skill, BonusType::Stacking, Decimal::ONE, None)),
        (value >= Decimal::TWO)
            .then(|| BonusTemplate::new(ability, BonusType::Stacking, Decimal::ONE, None)),
        // TODO: Bonus racial action point
    ]
    .into_iter()
    .flatten()
    .collect()
}
