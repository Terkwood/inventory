use crate::model::Inventory;
use chrono::prelude::*;

pub struct History {
    pub days: Vec<Day>,
    pub offset: FixedOffset,
}

pub struct Day {
    pub date: Date<FixedOffset>,
    pub inventory: Inventory,
}

impl History {
    /// Group the given inventory by day, for a given offset(timezone).
    pub fn from(inventory: &Inventory, offset: FixedOffset) -> Self {
        let days: Vec<Day> = group_by(&inventory.items, |mr| {
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
        Self { days, offset }
    }
}

fn group_by<I, F, K, T>(xs: I, mut key_fn: F) -> Vec<(K, Vec<T>)>
where
    I: IntoIterator<Item = T>,
    F: FnMut(&T) -> K,
    K: Eq,
{
    let mut groups = Vec::<(K, Vec<T>)>::new();
    for item in xs {
        let key = key_fn(&item);
        let last = groups.last_mut();
        if let Some((_, group)) = last.filter(|(k, _)| k == &key) {
            group.push(item);
        } else {
            groups.push((key, vec![item]));
        }
    }
    groups
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::model::*;

    const LOCAL_OFFSET_SECONDS: i32 = 240;

    fn make_time(days: u32) -> DateTime<Utc> {
        Utc.ymd(2020, 08, days + 1).and_hms(0, 0, 0)
    }

    #[test]
    fn test_grouping() {
        let item_type = crate::model::DefaultItemType::Resentment.instance();

        let size = 10;
        let items = &mut vec![];
        for i in 0..size {
            items.push(Item::new(
                item_type.clone(),
                "nil".to_string(),
                UtcMillis(make_time(i).timestamp_millis() as u64),
            ))
        }
        let inventory = Inventory {
            items: items.clone(),
        };

        let history = History::from(&inventory, FixedOffset::west(LOCAL_OFFSET_SECONDS));

        assert_eq!(history.days.len(), size as usize);
        for day in history.days {
            assert_eq!(day.inventory.items.len(), 1)
        }
        assert_eq!(history.offset, FixedOffset::west(LOCAL_OFFSET_SECONDS))
    }
}
