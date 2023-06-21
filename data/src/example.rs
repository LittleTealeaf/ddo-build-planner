use ron::error::SpannedError;

/// Returns example data created from build.rs
pub fn get_test_data() -> Result<String, SpannedError> {
    include_data!(String, "test")
}
