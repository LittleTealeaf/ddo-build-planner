use builder::attribute::Attribute;
use ron::{from_str, to_string};
use utils::enums::StaticOptions;

mod serialization {

    use super::*;
    #[test]
    fn static_attributes_serialize() {
        for attribute in Attribute::get_static() {
            let serialized = to_string(&attribute).expect("Could not serialize attribute");
            let deserialized: Attribute =
                from_str(&serialized).expect("Could not deserialize attribute");

            assert_eq!(
                attribute, deserialized,
                "Expected {attribute}, found {deserialized}"
            );
        }
    }

    #[test]
    fn set_bonuses_serialize() {
        let attribute = Attribute::ItemSet(String::from("Test"));
        let serialized = to_string(&attribute).expect("Could not serialize attribute");
        let deserialized: Attribute =
            from_str(&serialized).expect("Could not deserialize attribute");

        assert_eq!(
            attribute, deserialized,
            "Expected {attribute}, found {deserialized}"
        );
    }
}
