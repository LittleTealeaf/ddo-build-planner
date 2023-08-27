//! Contains all of the various types within Dungeons & Dragons: Online

macro_rules! public_modules {
    ($($module: ident),*) => {
        $(
            mod $module;
            pub use $module::*;
         )*
    };
}

public_modules!(ability, skill, spell_school);
