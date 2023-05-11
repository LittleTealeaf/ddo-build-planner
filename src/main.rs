#![allow(dead_code)]
use logic::{
    attribute::{Ability, Attribute, SpellPower},
    bonus::{Bonus, BonusSource, BonusType},
    breakdown::Breakdowns,
};

use crate::logic::{
    attribute::{Flag, Skill, WeaponHand},
    bonus::Bonuses,
    feat::{Feat, SkillFocus, Tome},
};

mod logic;
mod utils;

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses(vec![
        Bonus::new(
            Attribute::AbilityScore(Ability::All),
            BonusType::Stacking,
            10f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::AbilityScore(Ability::All),
            BonusType::Stacking,
            8f32,
            BonusSource::Unique(1),
            None,
        ),
    ]);

    breakdowns.insert_bonuses(vec![
        Bonus::new(
            Attribute::Feat(Feat::Tome(Tome::Ability(Ability::Constitution))),
            BonusType::Stacking,
            8f32,
            BonusSource::Unique(2),
            None,
        ),
        Bonus::new(
            Attribute::Feat(Feat::Tome(Tome::Ability(Ability::Strength))),
            BonusType::Stacking,
            8f32,
            BonusSource::Unique(2),
            None,
        ),
    ]);

    for (i, item) in [
        (BonusType::Stacking, 2f32),
        (BonusType::Enhancement, 15f32),
        (BonusType::Insightful, 5f32),
        (BonusType::Quality, 3f32),
        (BonusType::Quality, 3f32),
        (BonusType::Festive, 2f32),
        (BonusType::Exceptional, 1f32),
        (BonusType::Stacking, 2f32),
        (BonusType::Stacking, 2f32),
        (BonusType::Stacking, 2f32),
        (BonusType::Stacking, 2f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 1f32),
        (BonusType::Stacking, 2f32),
        (BonusType::Sacred, 6f32),
    ]
    .iter()
    .enumerate()
    {
        breakdowns.insert_bonuses(vec![Bonus::new(
            Attribute::AbilityScore(Ability::Constitution),
            item.0,
            item.1,
            BonusSource::Unique(i + 3),
            None,
        )])
    }

    breakdowns.insert_bonuses(vec![
        Bonus::new(
            Attribute::SetBonus(logic::attribute::SetBonus::LegendaryVulkoorsChosen),
            BonusType::Stacking,
            3f32,
            BonusSource::Unique(1000),
            None,
        ),
        Bonus::new(
            Attribute::Flag(Flag::AbilityToAttack(WeaponHand::Both, Ability::Strength)),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1000),
            None,
        ),
        Bonus::new(
            Attribute::Flag(Flag::AbilityToDamage(WeaponHand::Both, Ability::Strength)),
            BonusType::Stacking,
            1f32,
            BonusSource::Unique(1000),
            None,
        ),
        Bonus::new(Attribute::Flag(Flag::AbilityToDamage(WeaponHand::Both, Ability::Constitution)), BonusType::Stacking, 1f32, BonusSource::Unique(1000), None),
        Bonus::new(Attribute::Flag(Flag::AbilityToAttack(WeaponHand::Both, Ability::Constitution)), BonusType::Stacking, 1f32, BonusSource::Unique(1000), None),
        Bonus::new(
            Attribute::SetBonus(logic::attribute::SetBonus::LegendaryDreadIsleCurse),
            BonusType::Stacking,
            5f32,
            BonusSource::Unique(999),
            None,
        ),
    ]);

    breakdowns.get_all_attributes();

    println!(
        "{}",
        ron::ser::to_string_pretty(&breakdowns, ron::ser::PrettyConfig::default()).unwrap()
    );
}
