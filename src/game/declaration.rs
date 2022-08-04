use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(typescript_custom_section)]
const GAME_OPTIONS: &'static str = r#"
export interface GameOptions {
    devMode?: boolean;
    container?: HTMLElement;
}

export interface Game extends EventTarget {
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = GameOptions)]
    pub type GameOptions;

    #[wasm_bindgen(method, getter)]
    pub fn container(this: &GameOptions) -> Option<HtmlElement>;

    #[wasm_bindgen(method, getter = devMode)]
    pub fn dev_mode(this: &GameOptions) -> Option<bool>;
}
