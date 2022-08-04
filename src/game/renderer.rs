use std::f64::consts::{FRAC_PI_2, PI};

use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::CanvasRenderingContext2d;

pub struct Renderer {
    context: CanvasRenderingContext2d,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        Self {
            context,
            width,
            height,
        }
    }

    pub fn clear_frame(&self) {
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    pub fn draw_arrow(&self, direction: f64) {
        let text;

        if direction == FRAC_PI_2 {
            text = "↑";
        } else if direction == PI {
            text = "←";
        } else if direction == PI + FRAC_PI_2 {
            text = "↓";
        } else {
            text = "→";
        }

        self.context.set_fill_style(&JsValue::from_str("white"));

        self.context
            .fill_text(text, self.width as f64 / 2.0, self.height as f64 / 2.0)
            .unwrap_throw();
    }
}
