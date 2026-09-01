#[macro_export]
macro_rules! load_data {
    ($function: ident, $type: ty, $file: expr) => {
        fn $function() -> Result<$type, ron::de::SpannedError> {
            ron::from_str(include_str!(concat!(env!("OUT_DIR"), "/", $file)))
        }

        paste::item! {
            #[cfg(test)]
            #[test]
            fn [<test_ $function>]() {
                $function().unwrap();
            }
        }
    }
}
