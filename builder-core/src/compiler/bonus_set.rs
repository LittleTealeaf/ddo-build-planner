use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, Condition},
    utils::EnumBinaryMap,
};

use super::partial_bonus::PartialBonus;

pub struct BonusSet(EnumBinaryMap<Attribute, Vec<PartialBonus>>);

impl BonusSet {
    pub fn new() -> Self {
        Self(EnumBinaryMap::default())
    }

    pub fn get(&self, attribute: &Attribute) -> Option<&Vec<PartialBonus>> {
        let Self(map) = self;

        map.get(attribute)
    }

    pub fn add(&mut self, bonus: Bonus) {
        let Self(map) = self;
        let attribute = bonus.get_attribute();
        let partial = bonus.into();

        // map[attribute].push(partial);
        map.get_mut_or_default(&attribute).push(partial);
    }

    pub fn insert(&mut self, attribute: Attribute, bonuses: Vec<Bonus>) {
        let Self(map) = self;

        let mut partial_bonuses = bonuses
            .into_iter()
            .filter(|bonus| bonus.get_value() != 0f32)
            .map(PartialBonus::from);

        map.get_mut_or_default(&attribute)
            .extend(&mut partial_bonuses);
    }

    pub fn remove_source_from(&mut self, source: BonusSource, children: Vec<Attribute>) {
        let Self(map) = self;
        children.into_iter().for_each(|child| {
            let set = map.get_mut_or_default(&child);
            set.iter()
                .enumerate()
                .filter(|(_, item)| item.source.eq(&source))
                .map(|(i, _)| i)
                .rev()
                .collect_vec()
                .into_iter()
                .for_each(|i| {
                    set.swap_remove(i);
                });
        });
    }

    pub fn get_all_references(&mut self, attribute: &Attribute) -> Vec<Attribute> {
        let Self(map) = self;

        map.iter()
            .filter(|(_, set)| {
                set.iter().any(|bonus| {
                    if let Some(conditions) = &bonus.conditions {
                        conditions.iter().any(|condition| match condition {
                            Condition::Has(attr)
                            | Condition::NotHave(attr)
                            | Condition::NotEq(attr, _)
                            | Condition::Eq(attr, _)
                            | Condition::Max(attr, _)
                            | Condition::Min(attr, _) => attribute.eq(attr),
                        })
                    } else {
                        false
                    }
                })
            })
            .map(|(key, _)| key)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod remove_source_from {
        use crate::bonus::BonusType;

        use super::*;

        mod returns_reference {
            use super::*;

            #[test]
            fn has() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::Has(Attribute::Dodge())]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }

            #[test]
            fn not_have() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::NotHave(Attribute::Dodge())]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }

            #[test]
            fn max() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::Max(Attribute::Dodge(), 1f32)]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }

            #[test]
            fn min() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::Min(Attribute::Dodge(), 1f32)]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }

            #[test]
            fn eq() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::Eq(Attribute::Dodge(), 1f32)]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }
            #[test]
            fn not_eq() {
                let mut bonuses = BonusSet::new();
                bonuses.add(Bonus::new(
                    Attribute::Dummy(),
                    BonusType::Stacking,
                    1f32,
                    BonusSource::Unique(0),
                    Some(vec![Condition::NotEq(Attribute::Dodge(), 1f32)]),
                ));

                assert_eq!(
                    vec![Attribute::Dummy()],
                    bonuses.get_all_references(&Attribute::Dodge())
                );
            }
        }
    }
}
