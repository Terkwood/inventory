use crate::model::Inventory;
use chrono::prelude::*;
use group_by::group_by;

#[derive(Debug)]
pub struct History {
    pub days: Vec<Day>,
    pub offset: FixedOffset,
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
pub struct Day {
    pub date: Date<FixedOffset>,
    pub inventory: Inventory,
}

impl History {
    /// Group the given inventory by day, for a given offset(timezone).
    pub fn from(inventory: &Inventory, offset: FixedOffset) -> Self {
        let mut days: Vec<Day> = group_by(&inventory.items, |mr| {
            Utc.timestamp_millis(mr.epoch_millis_utc as i64)
                .with_timezone(&offset)
                .date()
        })
        .iter()
        .map(|d| {
            let items = d.1.iter().map(|&i| i.clone()).collect();
            Day {
                date: d.0,
                inventory: Inventory { items },
            }
        })
        .collect();
        days.sort_unstable();
        days.reverse();
        Self { days, offset }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::*;

    const LOCAL_OFFSET_SECONDS: i32 = 240;

    fn make_time(days: u32) -> DateTime<Utc> {
        Utc.ymd(2020, 08, days + 1).and_hms(0, 0, 0)
    }

    fn inventory_with(size: u32) -> Inventory {
        let item_type = crate::model::DefaultItemType::Resentment.instance();

        let items = &mut vec![];
        for i in 0..size {
            items.push(Item::new(
                item_type.clone(),
                "nil".to_string(),
                UtcMillis(make_time(i).timestamp_millis() as u64),
            ))
        }
        Inventory {
            items: items.clone(),
        }
    }
    #[test]
    fn test_grouping() {
        let size = 10;
        let inventory = inventory_with(size);
        let history = History::from(&inventory, FixedOffset::west(LOCAL_OFFSET_SECONDS));

        assert_eq!(history.days.len(), size as usize);
        for day in history.days {
            assert_eq!(day.inventory.items.len(), 1)
        }
        assert_eq!(history.offset, FixedOffset::west(LOCAL_OFFSET_SECONDS))
    }

    #[test]
    fn test_from_order() {
        let size = 3;
        let inventory = inventory_with(3);
        let history = History::from(&inventory, FixedOffset::west(LOCAL_OFFSET_SECONDS));

        for i in 0..(size - 1) {
            let this_date = history.days[i].date;
            let next_date = history.days[i + 1].date;
            assert!(this_date > next_date);
        }
    }
}
