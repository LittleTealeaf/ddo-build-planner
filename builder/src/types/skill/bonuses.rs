use crate::{
    attribute::TrackAttribute,
    bonus::{Bonus, CloneBonus},
    types::skill::Skill,
};

impl CloneBonus for Skill {
    fn clone_bonus(&self, bonus: &Bonus) -> Option<Vec<Bonus>> {
        matches!(self, Self::All).then(|| {
            Self::SKILLS
                .map(|skill| bonus.clone_into_attribute(skill))
                .to_vec()
        })
    }
}

impl TrackAttribute for Skill {
    fn is_tracked(&self) -> bool {
        !matches!(self, Self::All)
    }
}

#[cfg(test)]
mod tests {

    use crate::attribute::Attribute;

    use super::*;

    #[test]
    fn all_is_not_tracked() {
        assert!(!Skill::All.is_tracked());
        assert!(!Attribute::from(Skill::All).is_tracked());
    }

    #[test]
    fn skills_are_tracked() {
        for skill in Skill::SKILLS {
            assert!(skill.is_tracked());
            assert!(Attribute::from(skill).is_tracked());
        }
    }
}
