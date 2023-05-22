use crate::attribute::GetCloned;

/// Describes the different schools that spells fall under in DDO.
#[derive(
    Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, enum_map::Enum,
)]
pub enum SpellSchool {
    /// Abjuration Spells
    Abjuration,
    /// Conjuration Spells, or spells that *conjure* things.
    Conjuration,
    /// Divination Spells, or spells closely related to divine classes / features.
    Divination,
    /// Enchantment Spells, or spells that compell a target to do something.
    Enchantment,
    /// Evocation Spells, or spells that deal damage.
    Evocation,
    /// Spells that makes some form of false illusion.
    Illusion,
    /// Spells that deal with death.
    Necromancy,
    /// Spells that transmute or transform targets.
    Transmutation,
    /// Shortcut for all of the other spell schools.
    All,
}
impl ToString for SpellSchool {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Abjuration => "Abjuration",
            Self::Conjuration => "Conjuration",
            Self::Divination => "Divintation",
            Self::Enchantment => "Enchantment",
            Self::Evocation => "Evocation",
            Self::Illusion => "Illusion",
            Self::Necromancy => "Necromancy",
            Self::Transmutation => "Transmutation",
            Self::All => "All",
        })
    }
}

impl SpellSchool {
    /// All SpellSchools except for [`SpellSchool::All`]
    pub const VALUES: [SpellSchool; 8] = [
        Self::Abjuration,
        Self::Conjuration,
        Self::Divination,
        Self::Enchantment,
        Self::Evocation,
        Self::Illusion,
        Self::Necromancy,
        Self::Transmutation,
    ];
}

impl GetCloned<SpellSchool> for SpellSchool {
    fn get_cloned(&self) -> Option<Vec<SpellSchool>> {
        if matches!(self, Self::All) {
            Some(Self::VALUES.to_vec())
        } else {
            None
        }
    }
}
