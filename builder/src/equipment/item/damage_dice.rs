use std::fmt::Display;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageDice {
    multiplier: f32,
    dice_count: f32,
    dice_size: f32,
    inner_bonus: f32,
    outer_bonus: f32,
}

impl Display for DamageDice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}[{}d{} + {}] + {}",
            self.multiplier, self.dice_count, self.dice_size, self.inner_bonus, self.outer_bonus
        )
    }
}
