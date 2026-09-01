use getset::{Getters, Setters, WithSetters};
use serde::{Deserialize, Serialize};

use crate::{bonus::Bonus, effect::Effect, items::feat::prerequisite::Prerequisite};

mod prerequisite;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Getters, Setters, WithSetters,
)]
#[getset(get = "pub", set = "pub", set_with = "pub")]
pub struct Feat {
    name: String,
    description: String,
    prerequisites: Vec<Prerequisite>,
    effects: Vec<Effect>,
}

impl Feat {
    pub fn into_bonuses(self) -> impl Iterator<Item = Bonus> {
        self.effects.into_iter().flat_map(Effect::into_bonuses)
    }
}
