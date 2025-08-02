use wasm_bindgen::prelude::*;

mod app;
mod components;
mod services;
mod utils;
mod i18n;

pub use app::App;

#[wasm_bindgen(start)]
pub fn main() {
    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());
    
    // Initialize panic hook for better error reporting
    console_error_panic_hook::set_once();
    
    log::info!("AI4Thai Crop Guardian frontend starting...");
    
    // Mount the Yew app
    yew::Renderer::<App>::new().render();
}