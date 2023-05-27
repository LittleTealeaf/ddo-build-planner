use crate::{
    attribute::types::{Ability, ArmorClass},
    bonus::{Bonus, BonusSource, BonusType},
};

const BASE: BonusSource = BonusSource::Base;
const STACKING: BonusType = BonusType::Stacking;

pub fn build_default_values() -> Vec<Bonus> {
    vec![
        vec![Bonus::new(
            ArmorClass::ArmorScalar.into(),
            STACKING,
            1f32.into(),
            BASE,
            None,
        )],
        Ability::VALUES
            .map(|ability| Bonus::new(ability.into(), STACKING, 8f32.into(), BASE, None))
            .to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect()
}

#[cfg(test)]
mod tests {
    use crate::compiler::Compiler;

    use super::*;

    #[test]
    fn default_abilities() {
        let compiler = Compiler::default();

        for ability in Ability::VALUES {
            assert_eq!(compiler.get_attribute(&ability.into()), 8f32);
        }
    }
}
