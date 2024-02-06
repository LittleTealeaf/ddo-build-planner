use crate::ParseError;

/// Returns example data created from build.rs
///
/// # Errors
/// Returns errors if it was not able to be parsed
pub fn get_test_data() -> Result<String, ParseError> {
    include_data!("test")
}

#[test]
fn test_data_parses() {
    get_test_data().expect("Expected Test data to Parse");
}
