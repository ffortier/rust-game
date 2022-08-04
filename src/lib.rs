use wasm_bindgen::prelude::*;

mod game;

#[wasm_bindgen(start)]
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init().unwrap_throw();
}
