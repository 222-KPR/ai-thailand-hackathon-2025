use wasm_bindgen::prelude::*;

mod simple_app;
pub use simple_app::SimpleApp;

// Initialize WASM
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<SimpleApp>::new().render();
}
