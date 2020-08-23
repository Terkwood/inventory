pub mod history;
mod item;

pub use item::*;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Inventory {
    pub items: Vec<Item>,
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
            .filter(|item| {
                Utc.timestamp_millis(item.epoch_millis_utc as i64)
                    .with_timezone(&offset)
                    .date()
                    == local_now_date
            })
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
