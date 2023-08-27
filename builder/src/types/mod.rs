//! Contains all of the various types within Dungeons & Dragons: Online

macro_rules! module {
    ($module: ident) => {
        mod $module;
        pub use $module::*;
    };
}

module!(ability);
module!(skill);
