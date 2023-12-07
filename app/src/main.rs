//! Application Starting Point
use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    compiler::Compiler,
    equipment::item::types::ShieldType,
    types::{
        ability::Ability,
        armor_class::ArmorClass,
        flag::OffHandType,
        player_class::PlayerClass,
        race::Race,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
};

fn main() {
    let mut compiler = Compiler::default();

    println!("Adding Bonuses");

    compiler.add_bonuses(vec![Bonus::new(
        Ability::All.into(),
        BonusType::Stacking,
        10f32.into(),
        BonusSource::Custom(10),
        None,
    )]);

    compiler.add_bonuses(vec![
        Bonus::new(
            PlayerClass::FavoredSoul.into(),
            BonusType::Stacking,
            10f32.into(),
            0.into(),
            None,
        ),
        // Bonus::flag(OffHandType::from(ShieldType::TowerShield).into(), 0.into()),
        Bonus::flag(Race::Gnome.into(), 0.into()),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ShieldMaxDex),
            BonusType::Stacking,
            5f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ArmorMaxDex),
            BonusType::Stacking,
            10f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::All.into(),
            BonusType::Stacking,
            8f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Dexterity.into(),
            BonusType::Stacking,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Intelligence.into(),
            BonusType::Stacking,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Enhancement,
            20f32.into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Insightful,
            10f32.into(),
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
    ]);

    for (attr, val) in compiler.get_all_attributes() {
        println!("{attr}: {val}");
    }
}
