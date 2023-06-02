/// Generates a simple enum implementing the [ToString] trait.
///
/// This acts as a simple means of building large enums implementing the [ToString] trait without splitting up the code. There are two types of enums that can be produced from this macro: Simple enums (Enums with no sub types), or Complex Enums (Enums with additional types).
///
/// When building a simple enum, all that is needed is a list of the Identifiers, and their ToString equivilent. Their string equivilants should be written as static `&str` (just in `""`), as there are no sub types to modify the output.
///
/// An example of a simple string is as follows:
///
/// ```
/// use builder_core::simple_enum;
///
/// simple_enum!(
///     SimpleEnum,
///     "This is a Simple Enum",
///     (
///         OptionA "Option A",
///         OptionB "Option B",
///         OptionC "Option C"
///     )
/// );
///
/// assert_eq!(String::from("Option A"), SimpleEnum::OptionA.to_string());
/// ```
///
/// Enums can also be complex. The primary use of this is to create enums that have subsidary types. To indicate sub-variables, use `()` with every enum, and assign variables as you would in function parameters. When implementing [ToString], you will have the provided variable names as parameters to use in the expression.
///
/// ```
/// use builder_core::simple_enum;
///
/// simple_enum!(
///     SimpleEnum,
///     "This is a simple enum",
///     (
///         OptionA "Option A",
///         OptionB "Option B",
///         OptionC "Option C"
///     )
/// );
///
/// simple_enum!(
///     ComplexEnum,
///     "This is a Complex Enum",
///     (
///         ComplexA() String::from("Complex A"),
///         ComplexB(item: SimpleEnum) format!("Complex B: {}", item.to_string())
///     )
/// );
///
/// assert_eq!(String::from("Complex A"), ComplexEnum::ComplexA().to_string());
/// assert_eq!(String::from("Complex B: Option B"), ComplexEnum::ComplexB(SimpleEnum::OptionB).to_string());
/// ```
#[macro_export]
#[deprecated = "Implement enums manually with the Display trait instead"]
macro_rules! simple_enum {
    ($enum_name: ident, $documentation: expr, ($($id: ident $name: expr),*)) => {
        #[doc = $documentation]
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, enum_map::Enum)]
        pub enum $enum_name {
            $(
                #[doc = $name]
                $id
            ),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                String::from(match self {
                    $(Self::$id => $name),*
                })
            }
        }
    };
    ($enum_name: ident, $documentation: expr, ($($id: ident($($param_name: ident: $param_type: ty),*) $name: expr),*)) => {
        #[doc = $documentation]
        #[derive(Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Debug, enum_map::Enum)]
        pub enum $enum_name {
            $($id($($param_type),*)),*
        }

        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$id($($param_name),*) => $name),*
                }
            }
        }
    }
}
