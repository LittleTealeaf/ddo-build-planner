use ron::error::SpannedError;

/// Returns example data created from build.rs
///
/// # Errors
/// Returns errors if it was not able to be parsed
pub fn get_test_data() -> Result<String, SpannedError> {
    include_data!(String, "test")
}

#[test]
fn test_data_parses() {
    let data = get_test_data();

    assert!(data.is_ok());
}
