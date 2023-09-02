use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
/// Represents damage provided for a dice
pub struct DamageDice {
    multiplier: Option<f32>,
    dice_count: f32,
    dice_size: f32,
    inner_bonus: Option<f32>,
    outer_bonus: Option<f32>,
}


impl DamageDice {
    /// Creates a new dice
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

    /// Returns the multiplier of the inside roll
    pub fn get_multipler(&self) -> Option<f32> {
        self.multiplier
    }

    /// Returns the number of dice rolled
    pub fn get_dice_count(&self) -> f32 {
        self.dice_count
    }

    /// Returns the bonus added to the roll before applying the multiplier
    pub fn get_inner_bonus(&self) -> Option<f32> {
        self.inner_bonus
    }

    /// Returns the bonus added to the end
    pub fn get_outer_bonus(&self) -> Option<f32> {
        self.outer_bonus
    }

    /// Returns the number of faces on the dice
    pub fn get_dice_size(&self) -> f32 {
        self.dice_size
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
