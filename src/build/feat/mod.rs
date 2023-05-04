use crate::feats;

use super::{
    attribute::{saving_throw::SavingThrow, skill::Skill, Attribute},
    bonus::{source::Source, types::BonusType, Bonus},
};

mod macros;

feats!(
    Feat,
    Feat,
    (
        SkillFocus => (
            |source| vec![Bonus::new(Attribute::Skill(skill.clone()), BonusType::Stacking, 3.0, source, None)]
        ),
        (Skill, skill)
    ),
    (
        Acrobatic => (
            |source| vec![
                Bonus::new(Attribute::Skill(Skill::Jump), BonusType::Stacking, 2.0, source, None),
                Bonus::new(Attribute::Skill(Skill::Tumble), BonusType::Stacking, 2.0, source, None),
            ]
        ),
    )
);
