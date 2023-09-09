use builder::{
    attribute::{flags::OffHandType, Attribute},
    bonus::{Bonus, BonusSource, BonusType},
    compiler::Compiler,
    equipment::item::types::ShieldType,
    types::{Ability, ArmorClass, PlayerClass, Race, WeaponHand, WeaponStat},
};
use rust_decimal_macros::dec;

fn main() {
    let mut compiler = Compiler::default();

    println!("Adding Bonuses");

    compiler.add_bonuses(vec![Bonus::new(
        Ability::All.into(),
        BonusType::Stacking,
        dec!(10).into(),
        BonusSource::Custom(10),
        None,
    )]);

    compiler.add_bonuses(vec![
        Bonus::new(
            PlayerClass::FavoredSoul.into(),
            BonusType::Stacking,
            dec!(10).into(),
            0.into(),
            None,
        ),
        Bonus::flag(OffHandType::from(ShieldType::TowerShield).into(), 0.into()),
        Bonus::flag(Race::Gnome.into(), 0.into()),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ShieldMaxDexBonus),
            BonusType::Stacking,
            dec!(5).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Attribute::ArmorClass(ArmorClass::ArmorMaxDexBonus),
            BonusType::Stacking,
            dec!(10).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::All.into(),
            BonusType::Stacking,
            dec!(8).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Dexterity.into(),
            BonusType::Stacking,
            dec!(20).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Intelligence.into(),
            BonusType::Stacking,
            dec!(20).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Enhancement,
            dec!(20).into(),
            1.into(),
            None,
        ),
        Bonus::new(
            Ability::Wisdom.into(),
            BonusType::Insightful,
            dec!(10).into(),
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
