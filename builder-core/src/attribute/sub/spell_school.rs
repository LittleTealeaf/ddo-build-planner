use crate::simple_enum;

simple_enum!(SpellSchool, "", (Abjuration "Abjuration", Conjuration "Conjuration", Divination "Divintation", Enchantment "Enchantment", Evocation "Evocation", Illusion "Illusion", Necromancy "Necromancy", Transmutation "Transmutation", All "All"));

impl SpellSchool {
    pub fn get_cloned_schools(&self) -> Option<Vec<SpellSchool>> {
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
