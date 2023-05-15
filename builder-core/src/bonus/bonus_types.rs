use crate::simple_enum;

simple_enum!(
    BonusType,
    "",
    (
        AbilityModifier "Ability Modifier",
        ActionBoost "Action Boost",
        Alchemical "Alchemical",
        Artifact "Artifact",
        Competence "Competence",
        Deflection "Deflection",
        Enhancement "Enhancement",
        Equipment "Equipment",
        Epic "Epic",
        Exceptional "Exceptional",
        Feat "Feat",
        Festive "Festive",
        Insightful "Insightful",
        Legendary "Legendary",
        Morale "Morale",
        Music "Music",
        Primal "Primal",
        Profane "Profane",
        Quality "Quality",
        Sacred "Sacred",
        Shield "Shield",
        Size "Size",
        Stacking "Stacking",
        Spooky "Spooky"
    )
);

impl BonusType {
    /// Returns `true` if the bonus type is [`Stacking`].
    ///
    /// [`Stacking`]: Condition::Stacking
    pub fn is_stacking(&self) -> bool {
        matches!(self, Self::Stacking)
    }
}

impl Default for BonusType {
    fn default() -> Self {
        Self::Stacking
    }
}
