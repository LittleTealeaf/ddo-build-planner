use crate::feats;

use super::{
    attribute::{saving_throw::SavingThrow, skill::Skill, Attribute},
    bonus::{bonuses::Bonuses, source::Source, types::BonusType, Bonus},
};

mod macros;

feats!(
    Feat,
    Feat,
    SkillFocus(skill: Skill) => (
        format!("Skill Focus: {}", skill.to_string()),
        format!("Grants a +3 bonus to {}", skill.to_string()),
        |source| vec![Bonus::new(Attribute::Skill(skill.clone()), BonusType::Stacking, 3.0, source, None)]
    ),
    Acrobatic() => (
        "Acrobatic",
        "Provides a +2 bonus to the character's Jump and Tumble skills.",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Jump), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::Tumble), BonusType::Stacking, 2.0, source, None),
        ]
    ),
    Athletic() => (
        "Athletic",
        "Provides a +2 bonus to the character's Listen and Spot skills",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Listen), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::Spot), BonusType::Stacking, 2.0, source, None),
        ]
    ),
    CombatCasting() => (
        "Combat Casting",
        "Provides a +4 bonus to the character's concentration while casting spells in combat",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Concentration), BonusType::Stacking, 4.0, source, None)
        ]
    ),
    Negotiator() => (
        "Athletic",
        "Provides a +2 bonus to the character's Diplomacy and Haggle skills",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Diplomacy), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::Haggle), BonusType::Stacking, 2.0, source, None),
        ]
    ),
    NimbleFingers() => (
        "Nimble Fingers",
        "Provides a +2 bonus to the character's Disable Device and Open Lock skills",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::DisableDevice), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::OpenLock), BonusType::Stacking, 2.0, source, None),
        ]
    ),
    SelfSufficient() => (
        "Self Sufficient",
        "Provides a +2 bonus to the character's Heal and Repair skills",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Heal), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::Repair), BonusType::Stacking, 2.0, source, None),
        ]
    ),
    Stealthy() => (
        "Stealthy",
        "Provides a +2 bonus to the character's Hide and Move Silently skills",
        |source| vec![
            Bonus::new(Attribute::Skill(Skill::Hide), BonusType::Stacking, 2.0, source, None),
            Bonus::new(Attribute::Skill(Skill::MoveSilently), BonusType::Stacking, 2.0, source, None),
        ]
    )
);
