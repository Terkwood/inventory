#![recursion_limit = "1024"]
extern crate serde_derive;
extern crate serde_json;

mod inventory;
mod repo;
mod web;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<web::App>::new().mount_to_body();
}
