use super::UtcMillis;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct Item {
    pub epoch_millis_utc: u64,
    pub item_type: ItemType,
    pub text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
pub struct ItemType {
    pub name: String,
    pub emoji: String,
}

pub enum DefaultItemType {
    Resentment,
    Fear,
}

impl Item {
    pub fn new(item_type: ItemType, text: String, now: UtcMillis) -> Self {
        Item {
            epoch_millis_utc: now.0,
            item_type,
            text,
        }
    }
}

impl DefaultItemType {
    pub fn instance(&self) -> ItemType {
        let (name, emoji) = match self {
            DefaultItemType::Fear => ("Fear".to_string(), "😱".to_string()),
            DefaultItemType::Resentment => ("Resentment".to_string(), "😠".to_string()),
        };
        ItemType { name, emoji }
    }
}

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
}
