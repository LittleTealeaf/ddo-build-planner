use crate::{
    bonus::{
        traits::{ToAttribute, ToValue},
        Bonus, BonusCondition, BonusSource, BonusType, BonusValue,
    },
    types::{ability::Ability, armor_type::ArmorType},
    val,
};

#[derive(
    Hash,
    PartialEq,
    Eq,
    Clone,
    Copy,
    Debug,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
    derive_more::From,
    strum::VariantArray,
)]
pub enum ArmorClass {
    ShieldBonus,
    ArmorBonus,
    NaturalArmor,
    ShieldMultiplier,
    ArmorMultiplier,
    ArmorMaxDex,
    ShieldMaxDex,
    MaxDexBonus,
    Bonus,
    BonusMultiplier,
    Total,
}

impl ArmorClass {
    pub fn core_bonuses() -> impl Iterator<Item = Bonus> {
        [
            Bonus::new(
                Self::MaxDexBonus,
                {
                    let has_armor = BonusCondition::any(
                        [ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
                            .map(|armor| armor.attribute().to_value().is_some()),
                    );

                    BonusValue::condition(
                        has_armor,
                        Self::ArmorMaxDex.attribute().to_value(),
                        BonusValue::MAX,
                    )
                },
                BonusType::Ability,
                BonusSource::Custom("Armor Max Dex Bonus".to_owned()),
            ),
            Bonus::new(
                Self::Bonus,
                Ability::Dexterity
                    .modifier()
                    .to_value()
                    .min(Self::MaxDexBonus.attribute().to_value()),
                BonusType::Ability,
                BonusSource::Attribute(Ability::Dexterity.modifier()),
            ),
            Bonus::new(
                Self::Total,
                BonusValue::iter_sum([
                    Self::Bonus.attribute().to_value(),
                    Self::NaturalArmor.attribute().to_value(),
                    Self::ShieldBonus.attribute().to_value()
                        * Self::ShieldMultiplier
                            .attribute()
                            .to_value()
                            .as_multiplier(),
                    Self::ArmorBonus.attribute().to_value()
                        * Self::ArmorMultiplier.attribute().to_value().as_multiplier(),
                    val!(10),
                ])
                .unwrap_or(val!(0))
                    * Self::BonusMultiplier.attribute().to_value().as_multiplier(),
                BonusType::Standard,
                BonusSource::Custom("Calculated".to_owned()),
            ),
        ]
        .into_iter()
    }
}
