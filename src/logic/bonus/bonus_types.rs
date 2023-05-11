use crate::simple_enum;

simple_enum!(
    BonusType,
    (
        Stacking "Stacking",
        Enhancement "Enhancement",
        Equipment "Equipment",
        Insightful "Insightful",
        Quality "Quality",
        Feat "Feat",
        AbilityModifier "Ability Modifier",
        Artifact "Artifact",
        Legendary "Legendary",
        Sacred "Sacred",
        Exceptional "Exceptional",
        Festive "Festive",
        Profane "Profane"
    )
);
