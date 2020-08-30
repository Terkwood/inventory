use super::*;
use serde_derive::{Deserialize, Serialize};

const MAX_USER_BUTTONS: u8 = 4;

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct InventoryButtonCollection {
    pub user_item_types: Vec<ItemType>,
}

impl InventoryButtonCollection {
    pub fn empty() -> Self {
        Self {
            user_item_types: vec![],
        }
    }

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
            "😢", "😭", "😡", "😠", "😕", "🙄", "🙁", "😣", "😖", "😧", "😫", "😩", "😞", "😮",
            "😐", "😑", "😶", "😣", "🤐", "😔", "😨", "😬", "😰", "😱", "😳", "😵", "🤢",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    pub fn add(&mut self, button: ItemType) -> ButtonAddResult {
        if (self.user_item_types.len() as u8) < MAX_USER_BUTTONS {
            self.user_item_types.push(button);
            ButtonAddResult::Ok
        } else {
            ButtonAddResult::NotAdded
        }
    }

    pub fn delete(&mut self, button: &ItemType) {
        self.user_item_types.retain(|b| b != button)
    }
}

pub enum ButtonAddResult {
    Ok,
    NotAdded,
}
