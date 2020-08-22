use crate::model::Inventory;
use chrono::prelude::*;
use std::time::Duration;

pub struct History {
    pub days: Vec<Inventory>,
}

impl History {
    /// Group the given inventory by day, for a given offset(timezone).
    pub fn from(inventory: Inventory, offset: FixedOffset) -> Self {
        todo!()
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

    fn make_time(days: u32) -> DateTime<Utc> {
        Utc.ymd(2020, 08, days + 1).and_hms(0, 0, 0)
    }

    #[test]
    fn test_grouping() {
        let item_type = crate::model::DefaultItemType::Resentment.instance();

        let items = &mut vec![];
        for i in 0..10 {
            items.push(Item::new(
                item_type.clone(),
                "nil".to_string(),
                make_time(i).timestamp_millis() as u64,
            ))
        }
        let inventory = Inventory {
            items: items.clone(),
        };

        println!("{:?}", inventory);

        todo!()
    }
}
