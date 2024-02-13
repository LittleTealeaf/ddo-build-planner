pub mod attribute;
pub mod condition;
pub mod value;

// TODO: Rewrite this but with the following:
// - Root "SelectorContainer" that implements all of the methods and generalizes based on the
// selection chosen
// - The root "SelectorContainer" contains the Vec of Attributes
// - Each of the selectors has a reference to the array, and passes that reference as needed
