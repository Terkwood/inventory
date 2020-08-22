use crate::model::Inventory;
use chrono::prelude::*;

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

    #[test]
    fn test_grouping() {
        let item_type = crate::model::DefaultItemType::Resentment.instance();
        let inventory = Inventory::new(item_type, "nil", 0);

        todo!()
    }
}
