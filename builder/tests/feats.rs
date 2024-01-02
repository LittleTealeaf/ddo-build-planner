use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    breakdowns::Breakdowns,
    feat::{Feat, Proficiency},
    types::item::WeaponType,
};

#[test]
fn simple_proficiency_provides_proficiencies() {
    let mut compiler = Breakdowns::new();
    compiler.insert_bonus(Bonus::new(
        Attribute::Feat(Feat::Proficiency(Proficiency::SimpleWeaponProficiency)),
        BonusType::Stacking,
        1,
        BonusSource::Debug(0),
        None,
    ));

    assert!(
        compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
            Proficiency::WeaponProficiency(WeaponType::Dagger)
        ))) > 0.into()
    );
}

#[test]
fn martial_proficiency_provides_proficiencies() {
    let mut compiler = Breakdowns::new();
    compiler.insert_bonus(Bonus::new(
        Attribute::Feat(Feat::Proficiency(Proficiency::MartialWeaponProficiency)),
        BonusType::Stacking,
        1,
        BonusSource::Debug(0),
        None,
    ));

    assert!(
        compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
            Proficiency::WeaponProficiency(WeaponType::Falchion)
        ))) > 0.into()
    );
}
