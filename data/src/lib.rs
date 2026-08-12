use ddo_core::items::feat::Feat;

#[macro_export]
macro_rules! load_data {
    ($function: ident, $type: ty, $file: expr) => {
        /// Loads Data that has been Serialized in the binary
        ///
        /// # Errors
        /// Returns an error if there is a parsing issue from the data
        pub fn $function() -> Result<$type, ron::de::SpannedError> {
            ron::from_str(include_str!(concat!(env!("OUT_DIR"), "/", $file)))
        }

        paste::item! {
            #[cfg(test)]
            #[test]
            fn [<test_ $function>]() {
                $function().unwrap();
            }
        }
    };
}

load_data!(load_feats, Vec<Feat>, "feats");

#[cfg(test)]
mod test {
    load_data!(test_data, Vec<String>, "test");
}
