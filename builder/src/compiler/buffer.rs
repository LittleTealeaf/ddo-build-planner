use im::{OrdMap, OrdSet};
use utils::ord::IntoOrdGroupMap;

use crate::{
    attribute::{Attribute, TrackAttribute},
    bonus::{Bonus, BonusSource, CloneBonus},
};

#[derive(Default)]
pub struct Buffer {
    bonuses: OrdMap<BonusSource, Vec<Bonus>>,
    attributes: OrdSet<Attribute>,
    forced: OrdSet<Attribute>,
}

// PERF: Use the "children" design like the actual compiler does

impl Buffer {
    /// Inserts attributes into the queue. All attributes are forced as no bonuses are included
    pub fn insert_attributes<T>(&mut self, attributes: T)
    where
        T: IntoIterator<Item = Attribute>,
    {
        for attribute in attributes {
            self.attributes.insert(attribute);
            self.forced.insert(attribute);
        }
    }

    pub fn insert_bonuses<T>(&mut self, bonuses: T, forced: bool)
    where
        T: IntoIterator<Item = Bonus>,
    {
        let by_source = bonuses
            .into_iter()
            .flat_map(|bonus| {
                [
                    bonus
                        .get_attribute()
                        .clone_bonus(&bonus)
                        .unwrap_or_default(),
                    vec![bonus],
                ]
            })
            .flatten()
            .filter_map(|bonus| {
                bonus
                    .get_attribute()
                    .is_tracked()
                    .then_some((bonus.get_source(), bonus))
            })
            .into_grouped_ord_map();

        for (source, bonuses) in by_source {
            for bonus in &bonuses {
                let attribute = bonus.get_attribute();
                self.attributes.insert(attribute);
                if forced {
                    self.forced.insert(attribute);
                }
            }

            self.bonuses.insert(source, bonuses);
        }
    }

    pub fn pop(&mut self) -> Option<(Attribute, Vec<Bonus>, bool)> {
        loop {
            let attribute = self.attributes.remove_min()?;
            let forced = self.forced.remove(&attribute).is_some();
            let bonuses = self
                .bonuses
                .iter()
                .flat_map(|(_, bonuses)| {
                    bonuses
                        .iter()
                        .filter(|bonus| bonus.get_attribute().eq(&attribute))
                        .cloned()
                })
                .collect::<Vec<Bonus>>();

            if forced || !bonuses.is_empty() {
                return Some((attribute, bonuses, forced));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bonus::BonusType;

    use super::*;

    #[test]
    fn inserting_attribute_is_forced() {
        let mut buffer = Buffer::default();

        buffer.insert_attributes([Attribute::Debug(0)]);
        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, _, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(0));
        assert!(forced);
    }

    #[test]
    fn empty_buffer_pops_none() {
        let mut buffer = Buffer::default();
        assert!(buffer.pop().is_none());
        buffer.insert_attributes([Attribute::Debug(1)]);
        assert!(buffer.pop().is_some());
        assert!(buffer.pop().is_none());
    }

    #[test]
    fn inserting_bonus_pops_bonus() {
        let mut buffer = Buffer::default();
        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(0),
                None,
            )],
            false,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 1);
        assert!(!forced);

        buffer.insert_bonuses(
            [
                Bonus::new(
                    Attribute::Debug(3),
                    BonusType::Stacking,
                    10f32.into(),
                    BonusSource::Debug(0),
                    None,
                ),
                Bonus::new(
                    Attribute::Debug(3),
                    BonusType::Stacking,
                    10f32.into(),
                    BonusSource::Debug(1),
                    None,
                ),
            ],
            true,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 2);
        assert!(forced);
    }

    #[test]
    fn inserting_forced_overwrites() {
        let mut buffer: Buffer = Buffer::default();
        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(2),
                None,
            )],
            false,
        );

        buffer.insert_bonuses(
            [Bonus::new(
                Attribute::Debug(3),
                BonusType::Stacking,
                10f32.into(),
                BonusSource::Debug(0),
                None,
            )],
            true,
        );

        let value = buffer.pop();
        assert!(value.is_some());
        let (attribute, bonuses, forced) = value.unwrap();
        assert_eq!(attribute, Attribute::Debug(3));
        assert_eq!(bonuses.len(), 2);
        assert!(forced);
    }
}
