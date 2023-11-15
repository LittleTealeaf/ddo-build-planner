mod dwarf {
    use builder::{
        attribute::{flags::Flag, Attribute},
        bonus::{Bonus, BonusSource, BonusType},
        compiler::Compiler,
        equipment::item::types::WeaponType,
        feat::{Feat, Proficiency},
        types::Race,
    };
    use utils::float::ErrorMargin;

    #[test]
    fn dwarven_war_axe() {
        let mut compiler = Compiler::default();
        compiler.add_bonus(Bonus::flag(Flag::Race(Race::Dwarf), BonusSource::Debug(0)));
        assert!(compiler
            .get_attribute(&Attribute::Feat(Feat::Proficiency(
                Proficiency::WeaponProficiency(WeaponType::DwarvenWarAxe)
            )))
            .within_margin(&0f32));
        compiler.add_bonus(Bonus::new(
            Attribute::Feat(Feat::Proficiency(Proficiency::MartialWeaponProficiency)),
            BonusType::Stacking,
            1f32.into(),
            BonusSource::Debug(1),
            None,
        ));
        assert!(
            compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
                Proficiency::WeaponProficiency(WeaponType::DwarvenWarAxe)
            ))) > 0f32
        );
    }
}
