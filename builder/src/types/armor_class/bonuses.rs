use crate::{
    attribute::Attribute,
    bonus::Condition,
    equipment::item::types::{ArmorType, ShieldType},
    types::flag::{Flag, OffHandType},
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
