use super::*;

const MAX_USER_BUTTONS: u8 = 3;

#[derive(Clone, PartialEq)]
pub struct InventoryButtonCollection {
    pub user_item_types: Vec<ItemType>,
}

impl InventoryButtonCollection {
    pub fn all(&self) -> Vec<ItemType> {
        let mut r = self.user_item_types.clone();
        for d in DefaultItemType::all() {
            r.insert(0, d.instance())
        }
        r
    }

    pub fn free_user_buttons(&self) -> u8 {
        MAX_USER_BUTTONS - (self.all().len() - DefaultItemType::all().len()) as u8
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
