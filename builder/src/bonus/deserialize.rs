// Deserialized version of Bonus, used to rebuild Bonus, which
// re-populates the dependencies (without needing to serialize them)

use serde::{Deserialize, Serialize};

use crate::attribute::Attribute;

use super::{Bonus, BonusSource, BonusType, Condition, Value};

#[derive(Serialize, Deserialize)]
pub struct DeserializedBonus {
    #[serde(rename = "attr")]
    attribute: Attribute,
    #[serde(rename = "type")]
    bonus_type: BonusType,
    #[serde(rename = "val")]
    value: Value,
    #[serde(rename = "src")]
    source: BonusSource,
    #[serde(rename = "cond")]
    condition: Option<Condition>,
}

impl From<DeserializedBonus> for Bonus {
    fn from(value: DeserializedBonus) -> Self {
        Self::new(
            value.attribute,
            value.bonus_type,
            value.value,
            value.source,
            value.condition,
        )
    }
}
