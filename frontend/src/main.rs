use leptos::*;
use wasm_bindgen::prelude::*;

mod app;
mod components;
mod pages;
mod utils;

#[wasm_bindgen]
pub fn main() {
    // Initialize logging for debugging
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    
    log::info!("Initializing Slynqix application");
    
    mount_to_body(|| view! { <app::App /> });
}
