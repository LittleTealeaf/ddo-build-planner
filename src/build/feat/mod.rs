use crate::feats;

use super::{
    attribute::*,
    bonus::{BonusType, Bonus},
};

mod macros;
mod categories;
pub use categories::*;

feats!(
    Feat,
    SkillFeat(feat: SkillFeat) => (
        feat.get_name(),
        feat.get_description(),
        |_| feat.get_bonuses()
    ),
    SpellFocus(school: SpellSchool) => (
        format!("Spell Focus: {}", school.to_string()),
        format!("You spells from the {} school of magic are harder to resist, receiving a +1 bonus on Save DCs", school.to_string()),
        |source| vec![
            Bonus::new(Attribute::SpellFocus(*school), BonusType::Stacking, 1.0, source, None),
        ]
    ),
    GreaterSpellFocus(school: SpellSchool) => (
        format!("Spell Focus: {}", school.to_string()),
        format!("You spells from the {} school of magic are harder to resist, receiving a +1 bonus on Save DCs", school.to_string()),
        |source| vec![
            Bonus::new(Attribute::SpellFocus(*school), BonusType::Stacking, 1.0, source, None),
        ]
    ),
    ClassLore(class: ClassLore) => (
        format!("{} Lore", class.to_string()),
        format!("Indicates your knowledge of {} Lore, and allows for specific interactions with certain areas of the game", class.to_string()),
        |source| vec![
            Bonus::new(Attribute::ClassLore(*class), BonusType::Stacking, 1.0, source, None)
        ]
    )
);
