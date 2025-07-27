use core::iter::once;
use itertools::chain;
use utils::{chain_tree, enums::StaticValues, hashmap::IntoGroupedHashMap};

use crate::{
    attribute::Attribute,
    bonus::{
        Bonus, BonusSource, BonusTemplate, BonusType, Condition, ConditionFold, ToValue, Value,
    },
    feat::{HeroicPastLife, PastLifeFeat, RacialPastLife},
    types::{
        ability::Ability,
        absorption::{Absorption, AbsorptionSource},
        armor_class::ArmorClass,
        damage_type::DamageType,
        dodge::Dodge,
        flag::{Flag, MainHandType, OffHandType},
        health::Health,
        item_type::{ArmorType, ShieldType, WeaponType},
        player_class::PlayerClass,
        saving_throw::SavingThrow,
        sheltering::Sheltering,
        skill::Skill,
        sneak_attack::SneakAttack,
        spell_points::SpellPoints,
        spell_power::SpellPower,
        toggle::Toggle,
        weapon_attribute::{WeaponHand, WeaponStat},
    },
    val,
};

/// Returns all base bonuses that are to be included by default.
pub fn get_base_bonuses() -> impl Iterator<Item = Bonus> {
    chain_tree!(
        ability_bonuses(),
        armor_class(),
        saving_throw(),
        secondary_saves(),
        spell_power_skills(),
        spell_power_universal(),
        skill(),
        health(),
        spell_points(),
        sheltering(),
        sheltering_reduction(),
        armor_check_penalties(),
        absorption(),
        completionist_feats(),
        melee_fighting_styles(),
        sneak_attack(),
        weapon_damage(),
        dodge(),
    )
    .map(|bonus| bonus.to_bonus(BonusSource::Base))
}

fn ability_bonuses() -> impl IntoIterator<Item = BonusTemplate> {
    Ability::VALUES
        .into_iter()
        .map(|ability| {
            BonusTemplate::new(
                Attribute::AbilityModifier(ability),
                BonusType::Stacking,
                ((Attribute::Ability(ability).to_value() - Value::TEN) / Value::TWO).floor(),
            )
            .with_display_source(Attribute::Ability(ability))
        })
        .chain(once(BonusTemplate::new(
            Attribute::Ability(Ability::All),
            BonusType::Stacking,
            val!(8),
        )))
}

fn saving_throw() -> impl IntoIterator<Item = BonusTemplate> {
    [
        (Ability::Dexterity, SavingThrow::Reflex),
        (Ability::Constitution, SavingThrow::Fortitude),
        (Ability::Wisdom, SavingThrow::Will),
    ]
    .into_iter()
    .map(|(ability, saving_throw)| {
        BonusTemplate::new(
            saving_throw,
            BonusType::AbilityModifier,
            Attribute::AbilityModifier(ability),
        )
        .with_display_source(Attribute::AbilityModifier(ability))
    })
}

fn secondary_saves() -> impl Iterator<Item = BonusTemplate> {
    SavingThrow::SECONDARY.into_iter().filter_map(|skill| {
        let parent = skill.get_parent()?;
        Some(BonusTemplate::new(skill, BonusType::Stacking, parent).with_display_source(parent))
    })
}

fn spell_power_skills() -> impl IntoIterator<Item = BonusTemplate> {
    [
        (Skill::Heal, DamageType::Positive),
        (Skill::Heal, DamageType::Negative),
        (Skill::Perform, DamageType::Sonic),
        (Skill::Spellcraft, DamageType::Acid),
        (Skill::Spellcraft, DamageType::Cold),
        (Skill::Spellcraft, DamageType::Electric),
        (Skill::Spellcraft, DamageType::Fire),
        (Skill::Spellcraft, DamageType::Force),
        (Skill::Spellcraft, DamageType::Light),
        (Skill::Spellcraft, DamageType::Poison),
    ]
    .into_iter()
    .map(|(skill, damage)| {
        BonusTemplate::new(Attribute::spell_power(damage), BonusType::Stacking, skill)
            .with_display_source(skill)
    })
}

fn skill() -> impl IntoIterator<Item = BonusTemplate> {
    Skill::SKILLS.into_iter().filter_map(|skill| {
        let ability = skill.get_ability()?;
        Some(
            BonusTemplate::new(
                skill,
                BonusType::AbilityModifier,
                Attribute::AbilityModifier(ability),
            )
            .with_display_source(Attribute::AbilityModifier(ability)),
        )
    })
}

fn armor_class() -> impl IntoIterator<Item = BonusTemplate> {
    [
        // Maximum Dexterity Bonus
        BonusTemplate::new(
            ArmorClass::MaxDexBonus,
            BonusType::AbilityModifier,
            Value::condition(
                [ArmorType::Light, ArmorType::Medium, ArmorType::Heavy]
                    .map(Condition::has)
                    .cond_any()
                    .unwrap(),
                ArmorClass::ArmorMaxDex,
                Value::MAX,
            )
            .min(Value::condition(
                Condition::has(OffHandType::Shield(ShieldType::TowerShield)),
                ArmorClass::ShieldMaxDex,
                Value::MAX,
            )),
        ),
        // Dexterity Bonus to Armor Class
        BonusTemplate::new(
            ArmorClass::Bonus,
            BonusType::AbilityModifier,
            Value::Attribute(Attribute::AbilityModifier(Ability::Dexterity))
                .min(ArmorClass::MaxDexBonus.to_value()),
        )
        .with_display_source(Attribute::AbilityModifier(Ability::Dexterity)),
        // Total Armor Class Bonus
        BonusTemplate::new(
            ArmorClass::Total,
            BonusType::Standard,
            Value::iter_sum([
                ArmorClass::Bonus.to_value(),
                ArmorClass::NaturalArmor.to_value(),
                ArmorClass::ShieldBonus.to_value() * ArmorClass::ShieldScalar.to_value().scalar(),
                ArmorClass::ArmorBonus.to_value() * ArmorClass::ArmorScalar.to_value().scalar(),
                Value::TEN,
            ]) * ArmorClass::TotalScalar.to_value().scalar(),
        ),
    ]
}

fn health() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            Health::Bonus,
            BonusType::Stacking,
            Health::Base.to_value() * Health::BaseModifier.to_value().scalar(),
        ),
        BonusTemplate::new(
            Health::Total,
            BonusType::Stacking,
            Health::Bonus.to_value() * Health::Modifier.to_value().scalar(),
        ),
    ]
}

fn spell_points() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            SpellPoints::Base,
            BonusType::Stacking,
            SpellPoints::Scaled.to_value()
                * (PlayerClass::FavoredSoul.to_value()
                    + PlayerClass::Sorcerer.to_value()
                    + val!(20))
                / val!(20),
        ),
        BonusTemplate::new(
            SpellPoints::Total,
            BonusType::Stacking,
            SpellPoints::Base.to_value() * SpellPoints::Modifier.to_value().scalar(),
        ),
    ]
}

fn spell_power_universal() -> impl IntoIterator<Item = BonusTemplate> {
    SpellPower::SPELL_POWERS.into_iter().flat_map(|sp| {
        [
            Attribute::SpellPower,
            Attribute::SpellCriticalChance,
            Attribute::SpellCriticalDamage,
        ]
        .into_iter()
        .map(move |attribute| {
            BonusTemplate::new(
                attribute(sp),
                BonusType::Stacking,
                attribute(SpellPower::Universal),
            )
            .with_display_source(attribute(SpellPower::Universal))
        })
    })
}

fn sheltering() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::new(
            Sheltering::MagicalCap,
            BonusType::Stacking,
            Value::condition(
                Condition::has(ArmorType::Medium) | Condition::has(ArmorType::Heavy),
                Sheltering::Magical,
                Value::condition(
                    Condition::has(ArmorType::Light),
                    Value::ONE_HUNDRED,
                    val!(50),
                ),
            ),
        ),
        BonusTemplate::new(
            Sheltering::MagicalTotal,
            BonusType::Stacking,
            Sheltering::Magical
                .to_value()
                .min(Sheltering::MagicalCap.to_value()),
        ),
        BonusTemplate::new(
            Sheltering::PhysicalTotal,
            BonusType::Stacking,
            Sheltering::Physical,
        ),
    ]
}

fn sheltering_reduction() -> impl IntoIterator<Item = BonusTemplate> {
    [
        (Sheltering::PhysicalTotal, Sheltering::PhysicalReduction),
        (Sheltering::MagicalTotal, Sheltering::MagicalReduction),
    ]
    .into_iter()
    .map(|(total, reduction)| {
        BonusTemplate::new(
            reduction,
            BonusType::Stacking,
            Value::ONE_HUNDRED
                * (Value::ONE - (Value::ONE_HUNDRED / (Value::ONE_HUNDRED + total.to_value()))),
        )
    })
}

fn armor_check_penalties() -> impl Iterator<Item = BonusTemplate> {
    [
        (Skill::Balance, val!(-1)),
        (Skill::Hide, val!(-1)),
        (Skill::Jump, val!(-1)),
        (Skill::MoveSilently, val!(-1)),
        (Skill::Swim, val!(-2)),
        (Skill::Tumble, val!(-1)),
    ]
    .into_iter()
    .map(|(skill, scale)| {
        BonusTemplate::new(
            skill,
            BonusType::Stacking,
            scale * Attribute::ArmorCheckPenalty.to_value(),
        )
        .with_condition(Condition::has(Attribute::ArmorCheckPenalty))
    })
}

fn absorption() -> impl Iterator<Item = BonusTemplate> {
    DamageType::values().map(|damage_type| {
        BonusTemplate::new(
            Absorption::Total(damage_type),
            BonusType::Stacking,
            Value::ONE
                - AbsorptionSource::values()
                    .map(|bonus_type| {
                        Value::ONE - Absorption::Bonus(damage_type, bonus_type).to_value()
                    })
                    .product(),
        )
    })
}

fn completionist_feats() -> impl IntoIterator<Item = BonusTemplate> {
    [
        {
            // HEROIC COMPLETIONIST
            let condition = PlayerClass::values()
                .map(|class| (class.get_parent_class().unwrap_or(class), class))
                .into_grouped_hash_map()
                .into_values()
                .map(|set| {
                    set.into_iter()
                        .map(|class| HeroicPastLife(class).to_value())
                        .sum::<Value>()
                        .greater_than(Value::ZERO)
                })
                .cond_all()
                .expect("Expected Condition");

            BonusTemplate::feat(PastLifeFeat::HeroicCompletionist).with_condition(condition)
        },
        {
            // RACIAL COMPLETIONIST
            let condition = RacialPastLife::RACES
                .map(|race| (race.get_base().unwrap_or(race), race))
                .into_grouped_hash_map()
                .into_values()
                .map(|set| {
                    set.into_iter()
                        .map(ToValue::to_value)
                        .sum::<Value>()
                        .greater_or_equal_to(val!(3))
                })
                .cond_all()
                .expect("Expected Condition");

            BonusTemplate::feat(PastLifeFeat::RacialCompletionist).with_condition(condition)
        },
    ]
}

fn sneak_attack() -> impl IntoIterator<Item = BonusTemplate> {
    [
        BonusTemplate::toggle(Toggle::SneakAttack),
        BonusTemplate::new(
            (WeaponHand::Both, WeaponStat::Attack),
            BonusType::Stacking,
            Attribute::from(SneakAttack::Attack),
        )
        .with_condition(Condition::toggled(Toggle::SneakAttack)),
        BonusTemplate::new(
            (WeaponHand::Both, WeaponStat::Damage),
            BonusType::Stacking,
            Attribute::from(SneakAttack::Damage),
        )
        .with_condition(Condition::toggled(Toggle::SneakAttack)),
        BonusTemplate::new(
            (WeaponHand::Both, WeaponStat::TotalDamage),
            BonusType::Stacking,
            Value::dice(SneakAttack::Dice.to_value(), val!(6)),
        )
        .with_condition(Condition::toggled(Toggle::SneakAttack)),
    ]
}

fn weapon_damage() -> impl Iterator<Item = BonusTemplate> {
    [WeaponHand::Main, WeaponHand::Off]
        .into_iter()
        .flat_map(|hand| {
            [BonusTemplate::new(
                (hand, WeaponStat::TotalDamage),
                BonusType::Stacking,
                Attribute::from((hand, WeaponStat::WeaponDamage)).to_value()
                    * (Value::ONE + Attribute::from((hand, WeaponStat::DiceMultiplier)).to_value()),
            )]
        })
}

fn melee_fighting_styles() -> impl IntoIterator<Item = BonusTemplate> {
    let one_hand_main_hand = WeaponType::ONE_HANDED_MELEE_WEAPONS
        .map(|weapon| Condition::has(MainHandType::Weapon(weapon)))
        .cond_any()
        .expect("Expected Condition");

    let one_hand_off_hand = WeaponType::ONE_HANDED_MELEE_WEAPONS
        .map(|weapon| Condition::has(OffHandType::Weapon(weapon)))
        .cond_any()
        .expect("Expected Condition");

    [
        BonusTemplate::flag(Flag::TwoHandedFighting).with_condition(
            WeaponType::TWO_HANDED_MELEE_WEAPONS
                .map(|weapon| Condition::has(MainHandType::Weapon(weapon)))
                .cond_any()
                .expect("Expected Condition"),
        ),
        BonusTemplate::flag(Flag::SingleWeaponFighting).with_condition(
            one_hand_main_hand.clone()
                & chain!(
                    [
                        one_hand_off_hand.clone(),
                        Condition::has(OffHandType::Shield(ShieldType::Buckler))
                            & !Condition::has(Flag::BucklerSingleWeaponFighting)
                    ],
                    [
                        ShieldType::LargeShield,
                        ShieldType::TowerShield,
                        ShieldType::SmallShield
                    ]
                    .map(|st| Condition::has(OffHandType::Shield(st))),
                )
                .cond_none()
                .expect("Expected Condition"),
        ),
        BonusTemplate::flag(Flag::TwoWeaponFighting)
            .with_condition(one_hand_main_hand & one_hand_off_hand),
    ]
}

fn dodge() -> impl Iterator<Item = BonusTemplate> {
    once(
        BonusTemplate::new(
            Dodge::Total,
            BonusType::Stacking,
            Value::min(
                Value::min(
                    Dodge::Bonus.to_value(),
                    Dodge::Cap.to_value() + ArmorClass::MaxDexBonus.to_value().min(val!(25)),
                ) + Dodge::Temporary.to_value(),
                val!(95),
            ),
        )
        .with_display_source(Dodge::Bonus),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_base_bonuses_have_base_source() {
        for bonus in get_base_bonuses() {
            assert_eq!(
                bonus.source(),
                &BonusSource::Base,
                "Does not have base bonus: {bonus:?}"
            );
        }
    }
}
