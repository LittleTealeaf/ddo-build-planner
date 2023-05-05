#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ClassLore {
    Arcane,
    Religious,
    Wilderness,
}

impl ToString for ClassLore {
    fn to_string(&self) -> String {
        String::from(match self {
            ClassLore::Arcane => "Arcane",
            ClassLore::Religious => "Religious",
            ClassLore::Wilderness => "Wilderness",
        })
    }
}
