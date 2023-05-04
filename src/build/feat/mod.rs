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

// (
//     Acrobatic => (
//         vec![
//             Bonus::new(Attribute::Skill(Skill::Jump), BonusType::Stacking, 2.0, Source::Feat(Feat::Acrobatic()), None),
//             Bonus::new(Attribute::Skill(Skill::Tumble), BonusType::Stacking, 2.0, Source::Feat(Feat::Acrobatic()), None),
//         ]
//     ),
// ),
// (
//     Alertness => (
//         vec![
//             Bonus::new(Attribute::Skill(Skill::Listen), BonusType::Stacking, 2.0, Source::Feat(Feat::Alertness()), None),
//             Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Stacking, 2.0, Source::Feat(Feat::Alertness()), None),
//         ]
//     ),
// ),
// (
//     Athletic => (
//         vec![
//             Bonus::new(Attribute::Skill(Skill::Balance), BonusType::Stacking, 2.0, Source::Feat(Feat::Athletic()), None),
//             Bonus::new(Attribute::Skill(Skill::Swim), BonusType::Stacking, 2.0, Source::Feat(Feat::Athletic()), None),
//         ]
//     ),
// ),
// (
//     Bullhead => (
//         vec![
//             Bonus::new(Attribute::Skill(Skill::Intimidate), BonusType::Stacking, 2.0, Source::Feat(Feat::Bullhead()), None),
//             Bonus::new(Attribute::SavingThrow(SavingThrow::Will), BonusType::Stacking, 2.0, Source::Feat(Feat::Bullhead()), None),
//         ]
//     ),
// ),
// (
//     Discipline => (
//         vec![
//             Bonus::new(Attribute::Skill(Skill::Concentration), BonusType::Stacking, 2.0, Source::Feat(Feat::Discipline()), None),
//             Bonus::new(Attribute::SavingThrow(SavingThrow::Will), BonusType::Stacking, 2.0, Source::Feat(Feat::Discipline()), None),
//         ]
//     ),
// )
