use im::OrdMap;

use super::Dice;

pub enum DiceFormula<T>
where
    T: PartialOrd + Ord + PartialEq + Eq,
{
    Variable(T),
    Dice(Dice),
}

impl<T> DiceFormula<T>
where
    T: PartialOrd + Ord + Clone + Copy,
{
    pub fn to_dice(self, variables: OrdMap<T, f32>) -> Result<Dice, ConvertFormulaError<T>> {
        match self {
            Self::Variable(variable) => variables
                .get(&variable)
                .map(|var| Dice::Value(*var))
                .ok_or(ConvertFormulaError::VariableNotFound(variable)),
            Self::Dice(dice) => Ok(dice)
        }
    }
}

pub enum ConvertFormulaError<T> {
    VariableNotFound(T),
}
