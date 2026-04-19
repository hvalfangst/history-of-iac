use leptos::*;
use wasm_bindgen::prelude::*;

mod components;
mod data;
mod i18n;

use components::App;

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
