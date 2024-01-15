//! Application Starting Point

use builder::{
    attribute::Attribute,
    bonus::{BonusTemplate, BonusType},
    equipment::set_bonus::SetBonus,
    types::{
        ability::Ability, armor_class::ArmorClass, heal_amp::HealingAmplification,
        sheltering::Sheltering, spell_selector::SpellSelector,
    },
};
use ron::ser::PrettyConfig;

fn main() {
    let bonuses = vec![
        {
            let mut bonus = SetBonus::new("Might of the Abashai".to_string());

            bonus.bonuses_mut().insert(
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

            bonus.bonuses_mut().insert(
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

            bonus
        },
        {
            let mut bonus = SetBonus::new("Epic Might of the Abashai".to_string());

            bonus.bonuses_mut().insert(
                3,
                vec![
                    BonusTemplate::new(Sheltering::Both, BonusType::Profane, 10, None),
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

            bonus.bonuses_mut().insert(
                5,
                vec![
                    BonusTemplate::new(Sheltering::Both, BonusType::Profane, 15, None),
                    BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 8, None),
                    BonusTemplate::new(Ability::All, BonusType::Profane, 2, None),
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSelector::All),
                        BonusType::Profane,
                        2,
                        None,
                    ),
                ],
            );

            bonus
        },
        {
            let mut bonus = SetBonus::new("Legendary Might of the Abashai".to_string());

            bonus.bonuses_mut().insert(
                3,
                vec![
                    BonusTemplate::new(Sheltering::Both, BonusType::Profane, 20, None),
                    BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 10, None),
                    BonusTemplate::new(Ability::All, BonusType::Profane, 2, None),
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSelector::All),
                        BonusType::Profane,
                        2,
                        None,
                    ),
                    BonusTemplate::new(HealingAmplification::All, BonusType::Profane, 10, None),
                ],
            );

            bonus.bonuses_mut().insert(
                5,
                vec![
                    BonusTemplate::new(Sheltering::Both, BonusType::Profane, 20, None),
                    BonusTemplate::new(ArmorClass::NaturalArmor, BonusType::Profane, 10, None),
                    BonusTemplate::new(Ability::All, BonusType::Profane, 3, None),
                    BonusTemplate::new(
                        Attribute::SpellDC(SpellSelector::All),
                        BonusType::Profane,
                        3,
                        None,
                    ),
                    BonusTemplate::new(HealingAmplification::All, BonusType::Profane, 30, None),
                ],
            );

            bonus
        },
    ];

    println!(
        "{}",
        ron::ser::to_string_pretty(&bonuses, PrettyConfig::new()).unwrap()
    );
}
