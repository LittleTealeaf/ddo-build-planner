//! Application Starting Point

use builder::{
    attribute::Attribute,
    bonus::{BonusTemplate, BonusType},
    equipment::set_bonus::SetBonus,
    types::{ability::Ability, armor_class::ArmorClass, spell_selector::SpellSelector},
};
use ron::ser::PrettyConfig;

fn main() {
    let mut setbonus = SetBonus::new("Might of the Abashai".to_string());

    setbonus.bonuses_mut().insert(
        3,
        vec![
            BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 3, None),
            BonusTemplate::new(Ability::All, BonusType::Profane, 1, None),
            BonusTemplate::new(
                Attribute::SpellDC(SpellSelector::All),
                BonusType::Profane,
                1,
                None,
            ),
        ],
    );

    setbonus.bonuses_mut().insert(
        5,
        vec![
            BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 5, None),
            BonusTemplate::new(Ability::All, BonusType::Profane, 2, None),
            BonusTemplate::new(
                Attribute::SpellDC(SpellSelector::All),
                BonusType::Profane,
                2,
                None,
            ),
        ],
    );

    let bonuses = vec![setbonus];

    println!(
        "{}",
        ron::ser::to_string_pretty(&bonuses, PrettyConfig::new()).unwrap()
    );

    //
    // let mut breakdowns = Breakdowns::new();
    //
    // breakdowns.import_set_bonuses();
    //
    // breakdowns.insert_bonus(Bonus::new(
    //     Attribute::SetBonus("Might of the Abashai".to_string()),
    //     BonusType::Stacking,
    //     5,
    //     BonusSource::Custom(0),
    //     None,
    // ));
    // // clean up
    //
    // breakdowns.insert_bonus(Bonus::new(
    //     Ability::All,
    //     BonusType::Stacking,
    //     50,
    //     BonusSource::Custom(1),
    //     None,
    // ));
    //
    // println!(
    //     "{}",
    //     ron::ser::to_string_pretty(&breakdowns, PrettyConfig::new()).unwrap()
    // );
}
