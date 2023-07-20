//! Example Data used for tests and demonstrations

use crate::ParseError;

/// Returns example data created from build.rs
///
/// # Errors
/// Returns errors if it was not able to be parsed
pub fn get_test_data() -> Result<String, ParseError> {
    include_data!(String, "test")
}

#[test]
fn test_data_parses() {
    let data = get_test_data();

    assert!(data.is_ok());
}
