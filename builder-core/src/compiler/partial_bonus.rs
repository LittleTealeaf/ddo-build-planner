use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType, Condition},
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PartialBonus {
    pub value: f32,
    pub bonus_type: BonusType,
    pub source: BonusSource,
    pub conditions: Option<Vec<Condition>>,
}

impl PartialBonus {
    pub fn is_from_source(&self, source: &BonusSource) -> bool {
        self.source.eq(source)
    }

    pub fn into_bonus(self, attribute: Attribute) -> Bonus {
        Bonus::new(
            attribute,
            self.bonus_type,
            self.value,
            self.source,
            self.conditions,
        )
    }
}

impl From<Bonus> for PartialBonus {
    fn from(bonus: Bonus) -> Self {
        Self {
            value: bonus.get_value(),
            bonus_type: bonus.get_bonus_type(),
            source: bonus.get_source(),
            conditions: bonus.get_conditions(),
        }
    }
}
