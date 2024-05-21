//! Application Starting Point

use builder::{
    bonus::{Bonus, BonusType},
    breakdowns::Breakdowns,
    debug::DebugValue,
    feat::EpicPastLife,
    types::{
        flag::MainHandType,
        item_type::WeaponType,
        toggle::Toggle,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

fn main() {
    let mut breakdowns = Breakdowns::new();
    breakdowns.insert_bonus(Bonus::new(
        EpicPastLife::AncientPower,
        BonusType::Stacking,
        1,
        DebugValue(0),
    ));

    breakdowns.insert_bonus(Bonus::new(
        Toggle::EpicPastLife(EpicPastLife::AncientPower),
        BonusType::Stacking,
        1,
        DebugValue(1),
    ));

    breakdowns.insert_bonus(Bonus::new(
        MainHandType::Weapon(WeaponType::Club),
        BonusType::Stacking,
        1,
        DebugValue(2),
    ));

    breakdowns.evaluate_attribute_from((WeaponHand::Main, WeaponStat::Damage));
}
