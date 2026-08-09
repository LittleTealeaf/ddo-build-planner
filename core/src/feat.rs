use crate::{bonus::Bonus, effect::Effect, feat::prerequisite::Prerequisite};

mod prerequisite;

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Feat {
    name: String,
    description: String,
    prerequisites: Vec<Prerequisite>,
    effects: Vec<Effect>,
}

impl Feat {
    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub const fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn into_bonuses(self) -> impl Iterator<Item = Bonus> {
        self.effects.into_iter().flat_map(Effect::into_bonuses)
    }
}
