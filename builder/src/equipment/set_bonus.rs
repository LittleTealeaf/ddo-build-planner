//! Set Bonuses are stored in the `data` crate, using the following structure to load. Then, they
//! are imported into the breakdown object

use im::OrdMap;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, ToValue},
};

/// Describes a set bonus with it's name and bonuses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetBonus {
    name: String,
    bonuses: OrdMap<i32, Vec<BonusTemplate>>,
}

impl SetBonus {
    /// Returns a dynamic bonus entry for [`Breakdowns::import_dynamic_bonuses`]
    ///
    /// [`Breakdowns::import_dynamic_bonuses`]:
    /// crate::breakdowns::Breakdowns::import_dynamic_bonuses
    #[must_use]
    pub fn to_dynamic_bonus(self) -> (Attribute, Vec<BonusTemplate>) {
        let attribute = Attribute::SetBonus(self.name);

        (
            attribute.clone(),
            self.bonuses
                .into_iter()
                .flat_map(move |(count, bonuses)| {
                    let condition = attribute
                        .clone()
                        .to_value()
                        .greater_or_equal_to(count.to_value());
                    bonuses.into_iter().map(move |mut bonus| {
                        bonus.set_condition({
                            bonus
                                .condition()
                                .clone()
                                .map_or_else(|| condition.clone(), |cond| cond & condition.clone())
                        });
                        bonus
                    })
                })
                .collect(),
        )
    }

    /// Creates a new bonus
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            bonuses: OrdMap::new(),
        }
    }

    /// Returns a reference to the name of this [`SetBonus`].
    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    /// Sets the name of this [`SetBonus`].
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Returns a reference to the bonuses of this [`SetBonus`].
    #[must_use]
    pub const fn bonuses(&self) -> &OrdMap<i32, Vec<BonusTemplate>> {
        &self.bonuses
    }

    /// Returns a mutable reference to the bonuses of this [`SetBonus`].
    pub fn bonuses_mut(&mut self) -> &mut OrdMap<i32, Vec<BonusTemplate>> {
        &mut self.bonuses
    }
}
