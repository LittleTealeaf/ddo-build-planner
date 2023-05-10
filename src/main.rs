#![allow(dead_code)]
use logic::{
    attribute::{Ability, Attribute, SpellPower},
    bonus::{Bonus, BonusSource, BonusType},
    breakdown::Breakdowns,
};

use crate::logic::{
    attribute::{Flag, WeaponHand}
};

mod logic;
mod utils;

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses(vec![
        Bonus::new(
            Attribute::SpellPower(SpellPower::Potency),
            BonusType::Equipment,
            112f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::SpellPower(SpellPower::Potency),
            BonusType::Insightful,
            30f32,
            BonusSource::Unique(0),
            None,
        ),
        Bonus::new(
            Attribute::SpellPower(SpellPower::Positive),
            BonusType::Enhancement,
            172f32,
            BonusSource::Unique(0),
            None,
        ),
    ]);

    breakdowns.insert_bonuses(vec![Bonus::new(
        Attribute::AbilityScore(Ability::Wisdom),
        BonusType::Enhancement,
        20f32,
        BonusSource::Unique(1),
        None,
    )]);

    breakdowns.insert_bonuses(vec![Bonus::new(
        Attribute::AbilityScore(Ability::Intelligence),
        BonusType::Enhancement,
        13f32,
        BonusSource::Unique(3),
        None,
    )]);

    breakdowns.insert_bonuses(vec![Bonus::new(
        Attribute::AbilityScore(Ability::Intelligence),
        BonusType::Insightful,
        5f32,
        BonusSource::Unique(4),
        None,
    )]);

    breakdowns.insert_bonuses(vec![Bonus::new(
        Attribute::Flag(Flag::AbilityToAttack(WeaponHand::Both, Ability::Wisdom)),
        BonusType::Stacking,
        1f32,
        BonusSource::Unique(2),
        None,
    )]);

    // for (key, value) in breakdowns.get_all_attributes().into_iter() {
    //     println!("{}: {}", key.to_string(), value);
    // }

    println!(
        "{}",
        ron::ser::to_string_pretty(&breakdowns, ron::ser::PrettyConfig::default()).unwrap()
    );
}
