use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    compiler::Compiler,
    feat::{Feat, Proficiency}, types::item::WeaponType,
};

#[test]
fn simple_proficiency_provides_proficiencies() {
    let mut compiler = Compiler::default();
    compiler.add_bonus(Bonus::new(
        Attribute::Feat(Feat::Proficiency(Proficiency::SimpleWeaponProficiency)),
        BonusType::Stacking,
        1f32.into(),
        BonusSource::Debug(0),
        None,
    ));

    assert!(
        compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
            Proficiency::WeaponProficiency(WeaponType::Dagger)
        ))) > 0f32
    );
}

#[test]
fn martial_proficiency_provides_proficiencies() {
    let mut compiler = Compiler::default();
    compiler.add_bonus(Bonus::new(
        Attribute::Feat(Feat::Proficiency(Proficiency::MartialWeaponProficiency)),
        BonusType::Stacking,
        1f32.into(),
        BonusSource::Debug(0),
        None,
    ));

    assert!(
        compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
            Proficiency::WeaponProficiency(WeaponType::Falchion)
        ))) > 0f32
    );
}
