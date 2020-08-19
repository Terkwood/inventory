use crate::inventory::Inventory;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const INVENTORY_KEY: &str = "inventory";

pub struct Repo {
    pub inventory: Inventory,
}

impl Repo {
    pub fn load() -> Self {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");

        todo!()
    }
}
