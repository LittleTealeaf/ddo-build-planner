//! Provides macro utilities for simplifying module workflows

#[macro_export]
/// Automatically generates `mod <module>` and `pub use <module>::*` entries for all modules
/// listed in the parameters
macro_rules! public_modules {
    ($($module: ident),*) => {
        $(
            mod $module;
            pub use $module::*;
         )*
    };
}
