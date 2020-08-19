use crate::time::js_utc_now;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn empty() -> Self {
        Self { items: vec![] }
    }

    pub fn today(&self) -> Self {
        todo!()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub epoch_millis_utc: u64,
    pub item_type: ItemType,
    pub text: String,
}

impl Item {
    pub fn new(item_type: ItemType, text: String) -> Self {
        Item {
            epoch_millis_utc: js_utc_now(),
            item_type,
            text,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemType(String);

pub enum DefaultItemType {
    Resentment,
    Fear,
}

impl DefaultItemType {
    pub fn instance(&self) -> ItemType {
        let s = match self {
            DefaultItemType::Fear => "fear",
            DefaultItemType::Resentment => "resentment",
        };
        ItemType(s.to_string())
    }
}
