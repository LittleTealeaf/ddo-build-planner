use builder::{
    attribute::Attribute,
    bonus::{Bonus, BonusSource, BonusType},
    compiler::Compiler,
    equipment::item::types::WeaponType,
    feat::{Feat, Proficiency},
};

#[test]
fn simple_proficinecy_provides_proficiencies() {
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
