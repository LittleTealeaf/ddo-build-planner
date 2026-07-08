use crate::types::ability::Ability;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, derive_more::Display, Hash)]
pub enum Attribute {
    #[display("{_0} Score")]
    AbilityScore(Ability),
    #[display("{_0} Modifier")]
    AbilityModifier(Ability),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn testing() {
        let attribute = Attribute::AbilityScore(Ability::Charisma);
        let s = format!("{attribute}");
        println!("{s}");
    }
}
