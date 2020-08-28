use super::*;

const MAX_USER_BUTTONS: u8 = 3;

#[derive(Clone, PartialEq)]
pub struct InventoryButtonCollection {
    pub user_item_types: Vec<ItemType>,
}

impl InventoryButtonCollection {
    pub fn all(&self) -> Vec<ItemType> {
        let mut r = self.user_item_types.clone();
        r.insert(0, DefaultItemType::Fear.instance());
        r.insert(0, DefaultItemType::Resentment.instance());
        r
    }

    pub fn free_user_buttons(&self) -> u8 {
        (self.all().len() - DefaultItemType::all().len()) as u8 - MAX_USER_BUTTONS
    }

    pub fn allowed_emojis() -> Vec<String> {
        vec![
            "😢", "😭", "😡", "😠", "😕", "🙄", "🙁", "😣", "😖", "😧", "😫", "😩", "🤯", "😞",
            "😮", "😲", "😐", "😑", "😶", "😣", "🤐", "😔", "😨", "😬", "🥺", "😰", "😱", "😳",
            "🤪", "🥴", "😵", "🤬", "🤢", "🤮",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}
