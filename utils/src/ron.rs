//! ron serialization utility

pub mod pretty_config {
    //! Different Prettier Configurations
    use ron::ser::PrettyConfig;

    /// Compact Pretty Config setup, primarily for storing in the data crate.
    ///
    /// This attempts to optimize both space while still separating fields in lines to allow for
    /// simpler git tracking
    #[must_use]
    pub fn compact_pretty_config() -> PrettyConfig {
        PrettyConfig::new()
            .indentor(String::new())
            .compact_arrays(true)
            .separator(String::new())
    }
}
