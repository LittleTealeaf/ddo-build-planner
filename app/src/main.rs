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
        None,
        DebugValue(0),
    ));

    breakdowns.insert_bonus(Bonus::new(
        Toggle::EpicPastLife(EpicPastLife::AncientPower),
        BonusType::Stacking,
        1,
        None,
        DebugValue(1),
    ));

    breakdowns.insert_bonus(Bonus::new(
        MainHandType::Weapon(WeaponType::Club),
        BonusType::Stacking,
        1,
        None,
        DebugValue(2),
    ));


    breakdowns.get_attribute((WeaponHand::Main, WeaponStat::Damage));
}
