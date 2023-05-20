use std::collections::HashMap;

use itertools::Itertools;

use crate::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, Condition},
};

use super::partial_bonus::PartialBonus;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BonusSet(HashMap<Attribute, Vec<PartialBonus>>);

impl BonusSet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, attribute: &Attribute) -> Option<&Vec<PartialBonus>> {
        let Self(map) = self;
        map.get(attribute)
    }

    pub fn add(&mut self, bonus: Bonus) {
        let Self(map) = self;
        let attribute = bonus.get_attribute();
        let partial = bonus.into();

        if let Some(set) = map.get_mut(&attribute) {
            set.push(partial);
        } else {
            map.insert(attribute, vec![partial]);
        }
    }

    pub fn insert(&mut self, attribute: Attribute, bonuses: Vec<Bonus>) {
        let Self(map) = self;

        let mut partial_bonuses = bonuses
            .into_iter()
            .filter(|bonus| bonus.get_value() != 0f32)
            .map(PartialBonus::from)
            .collect();

        if let Some(set) = map.get_mut(&attribute) {
            set.append(&mut partial_bonuses);
        } else {
            map.insert(attribute, partial_bonuses);
        }
    }

    #[deprecated = "Try to use remove_sources_from"]
    pub fn remove_sources(&mut self, sources: Vec<BonusSource>) {
        let Self(map) = self;

        let entries = map
            .drain()
            .map(|(key, set)| {
                (
                    key,
                    set.into_iter()
                        .filter(|bonus| !sources.contains(&bonus.source))
                        .collect_vec(),
                )
            })
            .collect_vec();

        map.extend(entries.into_iter());
    }

    pub fn remove_source_from(&mut self, source: BonusSource, children: Vec<Attribute>) {
        let Self(map) = self;
        children.into_iter().for_each(|child| {
            if let Some(set) = map.get_mut(&child) {
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
            }
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
                            | Condition::Min(attr, _) => attribute.eq(&attr),
                        })
                    } else {
                        false
                    }
                })
            })
            .map(|(key, _)| *key)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod remove_source_from {
        use crate::bonus::{BonusType};

        use super::*;

        #[test]
        fn remove_sources_from_children() {
            let mut bonuses = BonusSet::new();

            bonuses.add(Bonus::dummy(BonusSource::Unique(0)));
            bonuses.add(Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, BonusSource::Unique(0), None));

            bonuses.remove_source_from(BonusSource::Unique(0), vec![Attribute::Dummy(), Attribute::Dodge()]);

            assert_eq!(0, bonuses.0.get(&Attribute::Dummy()).unwrap_or(&Vec::new()).len());
            assert_eq!(0, bonuses.0.get(&Attribute::Dodge()).unwrap_or(&Vec::new()).len());
        }

        #[test]
        fn does_not_remove_from_non_children() {
            let mut bonuses = BonusSet::new();

            bonuses.add(Bonus::dummy(BonusSource::Unique(0)));
            bonuses.add(Bonus::new(Attribute::Dodge(), BonusType::Stacking, 1f32, BonusSource::Unique(0), None));

            bonuses.remove_source_from(BonusSource::Unique(0), vec![Attribute::Dummy()]);

            assert_eq!(0, bonuses.0.get(&Attribute::Dummy()).unwrap_or(&Vec::new()).len());
            assert_eq!(1, bonuses.0.get(&Attribute::Dodge()).unwrap_or(&Vec::new()).len());
        }

        #[test]
        fn does_not_remove_non_sources() {
            let mut bonuses = BonusSet::new();

            bonuses.add(Bonus::dummy(BonusSource::Unique(1)));

            bonuses.remove_source_from(BonusSource::Unique(0), vec![Attribute::Dummy()]);

            assert_eq!(1, bonuses.0.get(&Attribute::Dummy()).unwrap_or(&Vec::new()).len());
        }
    }

    mod get_all_references {
        use crate::bonus::BonusType;

        use super::*;

        #[test]
        fn does_not_return_non_references() {
            let mut bonuses = BonusSet::new();
            bonuses.add(Bonus::new(
                Attribute::Dummy(),
                BonusType::Stacking,
                1f32,
                BonusSource::Unique(0),
                None,
            ));

            assert_eq!(
                Vec::<Attribute>::new(),
                bonuses.get_all_references(&Attribute::Dodge())
            );
        }

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
