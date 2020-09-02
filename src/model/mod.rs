pub mod history;
mod inventory_button_collection;
mod item;

pub use inventory_button_collection::*;
pub use item::*;

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Ord, Eq)]
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn today_shows_most_recent_first() {
        let mut inv = Inventory::empty();
        inv.add(Item::new(
            DefaultItemType::Fear.instance(),
            "foo".to_string(),
            UtcMillis(1500),
        ));
        inv.add(Item::new(
            DefaultItemType::Fear.instance(),
            "foo".to_string(),
            UtcMillis(500),
        ));

        let actual = inv.today(UtcMillis(500), FixedOffset::west(0));

        assert_eq!(actual.items.len(), 2);

        // They should be grouped such that most recent is first.
        assert_eq!(actual.items[0].epoch_millis_utc, 500)
    }

    #[test]
    fn test_add_order() {
        let mut inv = Inventory::empty();
        inv.add(Item::new(
            DefaultItemType::Fear.instance(),
            "foo".to_string(),
            UtcMillis(1500),
        ));
        inv.add(Item::new(
            DefaultItemType::Fear.instance(),
            "foo".to_string(),
            UtcMillis(500),
        ));

        assert_eq!(inv.items.len(), 2);
        assert_eq!(inv.items[0].epoch_millis_utc, 500);
        assert_eq!(inv.items[1].epoch_millis_utc, 1500);
    }
}
