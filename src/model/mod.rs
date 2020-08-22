pub mod history;

use crate::time::*;
use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub epoch_millis_utc: u64,
    pub item_type: ItemType,
    pub text: String,
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UtcMillis(pub u64);

impl Inventory {
    pub fn empty() -> Self {
        Self { items: vec![] }
    }

    pub fn today(&self, now: UtcMillis, offset: FixedOffset) -> Self {
        let local_now_date = Utc
            .timestamp_millis(now.0 as i64)
            .with_timezone(&offset)
            .date();
        let items = self
            .items
            .iter()
            .filter(|item| same_date(UtcMillis(item.epoch_millis_utc), local_now_date, offset))
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
            DefaultItemType::Fear => ("fear".to_string(), "😱".to_string()),
            DefaultItemType::Resentment => ("resentment".to_string(), "😠".to_string()),
        };
        ItemType { name, emoji }
    }
}
