use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {}
