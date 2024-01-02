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
    Condition::has(Flag::ArmorType(ArmorType::Light).into())
        | Condition::has(Flag::ArmorType(ArmorType::Medium).into())
        | Condition::has(Flag::ArmorType(ArmorType::Heavy).into())
}

fn is_wielding_tower_shield() -> Condition {
    Condition::has(Attribute::from(Flag::OffHandType(OffHandType::Shield(
        ShieldType::TowerShield,
    ))))
}

fn main() {
    let mut breakdowns = Breakdowns::new();

    breakdowns.insert_bonuses([Bonus::new(
        Ability::All.into(),
        BonusType::Stacking,
        10.into(),
        BonusSource::Custom(10),
        None,
    )]);

    breakdowns.insert_bonuses([
        Bonus::new(
            PlayerClass::FavoredSoul.into(),
            BonusType::Stacking,
            10.into(),
            0.into(),
            None,
        ),
        // Bonus::flag(OffHandType::from(ShieldType::TowerShield).into(), 0.into()),
        Bonus::flag(Race::Gnome.into(), 0.into()),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ShieldMaxDex),
            BonusType::Stacking,
            5.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ArmorMaxDex),
            BonusType::Stacking,
            10.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::All.into(),
            BonusType::Stacking,
            8.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Dexterity.into(),
            BonusType::Stacking,
            20.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Intelligence.into(),
            BonusType::Stacking,
            20.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Enhancement,
            20.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Insightful,
            10.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack).into(),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Intelligence).into(),
            2.into(),
            None,
        ),
        Bonus::new(
            (WeaponHand::Main, WeaponStat::Attack).into(),
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(Ability::Strength).into(),
            2.into(),
            None,
        ),
    ]);
}
