use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct DamageDice {
    multiplier: Option<f32>,
    dice_count: f32,
    dice_size: f32,
    inner_bonus: Option<f32>,
    outer_bonus: Option<f32>,
}

impl DamageDice {
    pub fn new(
        multiplier: Option<f32>,
        dice_count: f32,
        dice_size: f32,
        inner_bonus: Option<f32>,
        outer_bonus: Option<f32>,
    ) -> Self {
        Self {
            multiplier,
            dice_count,
            dice_size,
            inner_bonus,
            outer_bonus,
        }
    }
}

impl Display for DamageDice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.multiplier {
            Some(multiplier) => match self.outer_bonus {
                Some(outer_bonus) => match self.inner_bonus {
                    Some(inner_bonus) => write!(
                        f,
                        "{}[{}d{} + {}] + {}",
                        multiplier, self.dice_count, self.dice_size, inner_bonus, outer_bonus
                    ),
                    None => write!(
                        f,
                        "{}[{}d{}] + {}",
                        multiplier, self.dice_count, self.dice_size, outer_bonus
                    ),
                },
                None => match self.inner_bonus {
                    Some(inner_bonus) => write!(
                        f,
                        "{}[{}d{} + {}]",
                        multiplier, self.dice_count, self.dice_size, inner_bonus
                    ),
                    None => write!(f, "{}[{}d{}]", multiplier, self.dice_count, self.dice_size),
                },
            },
            None => match self.outer_bonus {
                Some(outer_bonus) => match self.inner_bonus {
                    Some(inner_bonus) => write!(
                        f,
                        "{}d{} + {}",
                        self.dice_count,
                        self.dice_size,
                        inner_bonus + outer_bonus
                    ),
                    None => write!(
                        f,
                        "{}d{} + {}",
                        self.dice_count, self.dice_size, outer_bonus
                    ),
                },
                None => match self.inner_bonus {
                    Some(inner_bonus) => write!(
                        f,
                        "{}d{} + {}",
                        self.dice_count, self.dice_size, inner_bonus
                    ),
                    None => write!(f, "{}d{}", self.dice_count, self.dice_size),
                },
            },
        }
    }
}
