//! Application Starting Point
use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
    breakdowns::Breakdowns,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        flag::{Flag, OffHandType},
        item::{ArmorType, ShieldType},
        player_class::PlayerClass,
        race::Race,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

fn is_wearing_armor() -> Condition {
    Condition::has(Flag::ArmorType(ArmorType::Light))
        | Condition::has(Flag::ArmorType(ArmorType::Medium))
        | Condition::has(Flag::ArmorType(ArmorType::Heavy))
}

fn is_wielding_tower_shield() -> Condition {
    Condition::has(Attribute::from(Flag::OffHandType(OffHandType::Shield(
        ShieldType::TowerShield,
    ))))
}

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses([Bonus::new(
        Ability::All,
        BonusType::Stacking,
        10,
        BonusSource::Custom(10),
        None,
    )]);

    breakdowns.insert_bonuses([
        Bonus::new(PlayerClass::FavoredSoul, BonusType::Stacking, 10, 0, None),
        Bonus::flag(OffHandType::from(ShieldType::TowerShield), 0),
        Bonus::flag(Race::Gnome, 0),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ShieldMaxDex),
            BonusType::Stacking,
            5,
            1,
            None,
        ),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ArmorMaxDex),
            BonusType::Stacking,
            10,
            1,
            None,
        ),
        Bonus::new(Ability::All, BonusType::Stacking, 8, 1, None),
        Bonus::new(Ability::Dexterity, BonusType::Stacking, 20, 1, None),
        Bonus::new(Ability::Intelligence, BonusType::Stacking, 20, 1, None),
        Bonus::new(Ability::Wisdom, BonusType::Enhancement, 20, 1, None),
        Bonus::new(Ability::Wisdom, BonusType::Insightful, 10, 1, None),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Intelligence),
            2,
            None,
        ),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Strength),
            2,
            None,
        ),
    ]);
    for bonus in breakdowns.get_bonuses() {
        println!("{bonus}");
    }
}
