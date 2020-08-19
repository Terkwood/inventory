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
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    item_type: ItemType,
    text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemType(String);
