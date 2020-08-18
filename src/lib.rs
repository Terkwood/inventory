#![recursion_limit = "1024"]
extern crate serde_derive;
extern crate serde_json;

mod inventory;
mod web;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::App::<web::Model>::new().mount_to_body();
}
