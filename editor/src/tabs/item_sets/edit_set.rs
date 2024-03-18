use builder::equipment::set_bonus::ItemSet;

#[derive(Debug, Clone)]
pub struct ItemSetEditor {
    item_set: ItemSet,
}

impl ItemSetEditor {
    pub const fn new(item_set: ItemSet) -> Self {
        Self { item_set }
    }
}

#[derive(Debug, Clone)]
pub enum ItemSetEditorMessage {}
