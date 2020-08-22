use crate::model::Inventory;
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const INVENTORY_KEY: &str = "inventory";

pub struct Repo {
    pub storage_service: StorageService,
}

impl Repo {
    pub fn new() -> Self {
        let storage_service =
            StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage_service }
    }

    pub fn read_inventory(&self) -> Inventory {
        if let Json(Ok(restored_model)) = self.storage_service.restore(INVENTORY_KEY) {
            restored_model
        } else {
            Inventory::empty()
        }
    }

    pub fn save_inventory(&mut self, inventory: &Inventory) {
        let value = Json(inventory);
        self.storage_service.store(INVENTORY_KEY, value)
    }
}
