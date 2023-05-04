use crate::build::{bonus::bonuses::Bonuses, bonus::Bonus};

// #[macro_export]
// macro_rules! feats {
//     ($name: ident, $($identifier: tt => $bonuses: expr),*) => {
//         enum $name {
//             $($identifier,)*
//         }
//         impl Bonuses for $name {
//             fn get_bonuses(&self) -> Vec<Bonus> {
//                 match self {
//                     $($identifier => $bonuses,)*
//                 }
//             }
//         }
//     }
//
// }
//
// feats!(MyFeats, Test => {vec![]},);
//
// fn test() {
//     MyFeats::Test.get_bonuses();
// }
