use crate::model::{Inventory, InventoryButtonCollection};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};

const INVENTORY_KEY: &str = "inventory";
const BUTTONS_KEY: &str = "buttons";

pub struct InventoryRepo {
    pub storage_service: StorageService,
}

impl InventoryRepo {
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

pub struct ButtonsRepo {
    pub storage_service: StorageService,
}

impl ButtonsRepo {
    pub fn new() -> Self {
        let storage_service =
            StorageService::new(Area::Local).expect("storage was disabled by the user");

        Self { storage_service }
    }

    pub fn read_buttons(&self) -> InventoryButtonCollection {
        if let Json(Ok(restored_model)) = self.storage_service.restore(BUTTONS_KEY) {
            restored_model
        } else {
            InventoryButtonCollection::empty()
        }
    }

    pub fn save_buttons(&mut self, buttons: &InventoryButtonCollection) {
        let value = Json(buttons);
        self.storage_service.store(INVENTORY_KEY, value)
    }
}
