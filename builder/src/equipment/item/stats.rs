use serde::{Deserialize, Serialize};

use crate::types::Ability;

use super::damage_dice::DamageDice;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
pub enum ItemStats {
    #[default]
    Equipment,
    Armor {
        armor_bonus: f32,
        max_dex_bonus: f32,
        armor_check_penalty: f32,
        arcane_spell_failure: f32,
    },
    MeleeWeapon {
        damage_dice: DamageDice,
        critical_range: f32,
        critical_multiplier: f32,
        attack_mod: Vec<Ability>,
        damage_mod: Vec<Ability>,
    },
}
