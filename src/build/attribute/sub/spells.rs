use crate::attribute_subtype;

attribute_subtype!(SpellSchool, (Evocation "Evocation"), (Illusion "Illusion"), (Conjuration "Conjuration"), (Abjuration "Abjuration"), (Enchantment "Enchantment"), (Transmutation "Transmutation"), (Necromancy "Necromancy"));

attribute_subtype!(SpellDamageType, (Acid "Acid"), (Cold "Cold"), (Electric "Electric"), (Fire "Fire"), (Force "Force"), (Light "Light"), (Negative "Negative"), (Poison "Poison"), (Positive "Positive"), (Repair "Repair"), (Sonic "Sonic"), (Universal "Universal"));
