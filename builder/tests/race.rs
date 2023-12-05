mod dwarf {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource},
        compiler::Compiler,
        equipment::item::types::WeaponType,
        feat::{Feat, Proficiency},
        types::{Flag, Race},
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
        compiler.add_bonus(Bonus::feat(
            Feat::Proficiency(Proficiency::MartialWeaponProficiency),
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
