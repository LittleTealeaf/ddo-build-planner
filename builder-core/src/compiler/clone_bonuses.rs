use crate::{bonus::Bonus, utils::EnumBinaryMap};

#[inline(always)]
pub fn clone_bonuses(bonuses: &mut Vec<Bonus>) {
    let mut queue = EnumBinaryMap::from(
        bonuses
            .drain(..)
            .map(|bonus| (bonus.get_attribute(), bonus)),
    );

    while let Some((attribute, mut attr_bonuses)) = queue.pop() {
        if let Some(clone_attributes) = attribute.get_attribute_clones() {
            for clone_attr in clone_attributes {
                queue
                    .get_mut_or_default(&clone_attr)
                    .extend(attr_bonuses.iter().map(|bonus| {
                        Bonus::new(
                            clone_attr,
                            bonus.get_bonus_type(),
                            bonus.get_value(),
                            bonus.get_source(),
                            bonus.get_conditions(),
                        )
                    }));
            }
        }
        bonuses.append(&mut attr_bonuses);
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{attribute::{sub::Skill, Attribute}, bonus::BonusType};

    use super::*;

    #[test]
    fn clones_cloned_bonuses() {
        let attributes: Vec<Attribute> = vec![Skill::All.into()];
        let mut bonuses = attributes.into_iter().map(|attribute| Bonus::new(attribute, BonusType::Stacking, 1f32, 5.into(), None)).collect_vec();

        clone_bonuses(&mut bonuses);

        let attributes = bonuses.into_iter().map(|bonus| bonus.get_attribute()).collect_vec();

        for skill in Skill::VALUES {
            assert!(attributes.contains(&skill.into()));
        }

        assert!(attributes.contains(&Skill::All.into()));

    }
}
