mod history;

pub use history::History;

use crate::time::*;
use chrono::prelude::*;
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
        let utc_now_date = Utc.timestamp_millis(js_utc_now() as i64).date();
        let items = self
            .items
            .iter()
            .filter(|item| same_date_utc(item.epoch_millis_utc, utc_now_date))
            .cloned()
            .collect();
        Inventory { items }
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item)
    }

    pub fn resolve(&mut self, epoch_millis_utc: u64) {
        self.items
            .retain(|item| item.epoch_millis_utc != epoch_millis_utc)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub epoch_millis_utc: u64,
    pub item_type: ItemType,
    pub text: String,
}

impl Item {
    pub fn new(item_type: ItemType, text: String, now: u64) -> Self {
        Item {
            epoch_millis_utc: now,
            item_type,
            text,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemType {
    pub name: String,
    pub emoji: String,
}

pub enum DefaultItemType {
    Resentment,
    Fear,
}

impl DefaultItemType {
    pub fn instance(&self) -> ItemType {
        let (name, emoji) = match self {
            DefaultItemType::Fear => ("fear".to_string(), "😱".to_string()),
            DefaultItemType::Resentment => ("resentment".to_string(), "😠".to_string()),
        };
        ItemType { name, emoji }
    }
}
