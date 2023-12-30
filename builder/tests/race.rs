mod dwarf {
    use builder::{
        attribute::Attribute,
        bonus::{Bonus, BonusSource},
        breakdowns::Breakdowns,
        feat::{Feat, Proficiency},
        types::{flag::Flag, item::WeaponType, race::Race},
    };

    #[test]
    fn dwarven_war_axe() {
        let mut compiler = Breakdowns::new();
        compiler.insert_bonus(Bonus::flag(Flag::Race(Race::Dwarf), BonusSource::Debug(0)));
        assert!(
            compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
                Proficiency::WeaponProficiency(WeaponType::DwarvenWarAxe)
            ))) == 0.into()
        );
        compiler.insert_bonus(Bonus::feat(
            Feat::Proficiency(Proficiency::MartialWeaponProficiency),
            BonusSource::Debug(1),
            None,
        ));
        assert!(
            compiler.get_attribute(&Attribute::Feat(Feat::Proficiency(
                Proficiency::WeaponProficiency(WeaponType::DwarvenWarAxe)
            ))) > 0.into()
        );
    }
}
