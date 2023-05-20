use crate::{simple_enum, attribute::GetCloned};

simple_enum!(SpellSchool, "", (Abjuration "Abjuration", Conjuration "Conjuration", Divination "Divintation", Enchantment "Enchantment", Evocation "Evocation", Illusion "Illusion", Necromancy "Necromancy", Transmutation "Transmutation", All "All"));

impl GetCloned<SpellSchool> for SpellSchool {
    fn get_cloned(&self) -> Option<Vec<SpellSchool>> {
        if let Self::All = self {
            Some(vec![
                Self::Abjuration,
                Self::Conjuration,
                Self::Divination,
                Self::Enchantment,
                Self::Evocation,
                Self::Illusion,
                Self::Necromancy,
                Self::Transmutation,
            ])
        } else {
            None
        }
    }
}
