#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BonusType {
    Stacking,
    Flag,
    AbilityScore,
    AbilityModifier,
    Enhancement,
    Equipment,
    Insightful,
    Quality,
    Profane,
    Exceptional,
    Primal,
    Sacred,
}
