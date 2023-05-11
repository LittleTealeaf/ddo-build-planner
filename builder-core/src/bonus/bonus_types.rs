use crate::simple_enum;

simple_enum!(
    BonusType,
    (
        AbilityModifier "Ability Modifier",
        Artifact "Artifact",
        Enhancement "Enhancement",
        Equipment "Equipment",
        Exceptional "Exceptional",
        Feat "Feat",
        Festive "Festive",
        Insightful "Insightful",
        Legendary "Legendary",
        Profane "Profane",
        Quality "Quality",
        Sacred "Sacred",
        Stacking "Stacking"
    )
);
