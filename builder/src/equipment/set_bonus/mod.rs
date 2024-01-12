//! Set Bonuses are stored in the `data` crate, using the following structure to load. Then, they
//! are imported into the breakdown object

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusTemplate, Condition, ToValue},
};

/// Describes a set bonus with it's name and bonuses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetBonus {
    name: String,
    bonuses: HashMap<i32, Vec<BonusTemplate>>,
}

// TODO: Convert all the "get bonus" fields to return BonusTemplate instead of Bonus

impl SetBonus {
    /// Creates an iterator of bonuses pulled from this set bonus
    pub fn to_bonuses(self) -> impl Iterator<Item = Bonus> {
        let attribute = Attribute::SetBonus(self.name);

        self.bonuses.into_iter().flat_map(move |(count, bonuses)| {
            let attribute = attribute.clone();
            bonuses.into_iter().map(move |bonus| {
                Bonus::new(
                    bonus.attribute().clone(),
                    *bonus.bonus_type(),
                    bonus.value().clone(),
                    attribute.clone(),
                    bonus.condition().clone().unwrap_or(Condition::TRUE)
                        & (attribute.clone().value().greater_than(count.value())
                            | attribute.clone().value().equal_to(count.value())),
                )
            })
        })
    }

    /// Converts this set bonus to a dynamic bonus
    #[must_use]
    pub fn to_dynamic_bonus(self) -> (Attribute, Vec<Bonus>) {
        (
            Attribute::SetBonus(self.name.clone()),
            self.to_bonuses().collect(),
        )
    }

    /// Creates a new bonus
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            bonuses: HashMap::new(),
        }
    }

    /// Returns a mutable reference to the bonuses of this [`SetBonus`].
    pub fn bonuses_mut(&mut self) -> &mut HashMap<i32, Vec<BonusTemplate>> {
        &mut self.bonuses
    }
}
