#[cfg(test)]
#[macro_export]
macro_rules! test_default_bonuses {
    ($name: ident) => {
        #[test]
        fn default_bonuses_have_base_source() {
            use $crate::bonus::BonusSource;

            for bonus in $name::get_default_bonuses() {
                assert_eq!(bonus.get_source(), BonusSource::Base);
            }
        }
    };
}

