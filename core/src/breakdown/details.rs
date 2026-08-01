use core::iter::once;

use im::HashMap;
use rust_decimal::Decimal;

use crate::{
    attribute::Attribute,
    breakdown::{calculate::BonusCalculator, Breakdown},
};

pub type DetailsCache = HashMap<Attribute, AttributeDetail>;

#[derive(Debug, Clone, Default)]
pub struct AttributeDetail {
    value: Decimal,
}

impl BonusCalculator<'_> {
    fn calculate_details(&mut self, attribute: Attribute) -> AttributeDetail {
        let value = self.evaluate_attribute(attribute, None);
        AttributeDetail { value }
    }
}
impl Breakdown {
    pub fn refresh_details(&mut self, attribute: &Attribute) -> bool {
        let Some(entry) = self.details.get_mut(attribute) else {
            return false;
        };
        let mut calc = BonusCalculator::from((&mut self.calc_cache, &self.bonuses));
        let details = calc.calculate_details(attribute.clone());
        *entry = details;
        true
    }

    pub fn store_detail(&mut self, attribute: Attribute) {
        self.store_details(once(attribute));
    }

    pub fn store_details<I>(&mut self, attributes: I)
    where
        I: IntoIterator<Item = Attribute>,
    {
        let mut calc = BonusCalculator::from((&mut self.calc_cache, &self.bonuses));
        for attribute in attributes {
            let detail = calc.calculate_details(attribute.clone());
            self.details.insert(attribute, detail);
        }
    }

    pub fn remove_detail(&mut self, attribute: &Attribute) {
        self.details.remove(attribute);
    }

    pub fn clear_details(&mut self) {
        self.details.clear();
    }

    #[must_use]
    pub fn get_detail(&self, attribute: &Attribute) -> Option<&AttributeDetail> {
        self.details.get(attribute)
    }
}
