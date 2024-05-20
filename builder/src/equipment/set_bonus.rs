//! Item Sets are stored in the `data` crate, using the following structure to load. Then, they
//! are imported into the breakdown object

use im::OrdMap;
use serde::{Deserialize, Serialize};

use crate::{
    attribute::Attribute,
    bonus::{BonusTemplate, ToValue},
};

/// Describes an item set with it's name and bonuses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemSet {
    #[serde(rename = "n", alias = "name")]
    name: String,
    #[serde(rename = "b", alias = "bonuses")]
    bonuses: OrdMap<i32, Vec<BonusTemplate>>,
}

impl ItemSet {
    /// Returns a dynamic bonus entry for [`Breakdowns::import_dynamic_bonuses`]
    ///
    /// [`Breakdowns::import_dynamic_bonuses`]:
    /// crate::breakdowns::Breakdowns::import_dynamic_bonuses
    pub fn to_dynamic_bonus(self) -> (Attribute, impl Iterator<Item = BonusTemplate>) {
        let attribute = Attribute::ItemSet(self.name);

        (
            attribute.clone(),
            self.bonuses.into_iter().flat_map(move |(count, bonuses)| {
                let condition = attribute
                    .clone()
                    .to_value()
                    .greater_or_equal_to(count.to_value());
                bonuses
                    .into_iter()
                    .map(move |bonus| bonus.with_condition_and(condition.clone()))
            }),
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
