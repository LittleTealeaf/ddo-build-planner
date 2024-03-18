//! ron serialization utility

mod pretty_config {
    use ron::ser::PrettyConfig;

    pub fn compact_pretty_config() -> PrettyConfig {
        PrettyConfig::new()
            .indentor(String::new())
            .compact_arrays(true)
    }
}
