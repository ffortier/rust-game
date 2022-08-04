use std::str::FromStr;

use js_sys::Date;
use thiserror::Error;
use web_sys::KeyboardEvent;

use super::renderer::Renderer;

#[derive(Debug, Default)]
pub struct GameLogic {
    last_frame_time: f64,
    direction: f64,
}

#[derive(Debug)]
enum ControlKey {
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}

#[derive(Debug, Error)]
enum ControlKeyError {
    #[error("not a control key")]
    NotAControlKey,
}

impl FromStr for ControlKey {
    type Err = ControlKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ArrowUp" => Ok(ControlKey::ArrowUp),
            "ArrowDown" => Ok(ControlKey::ArrowDown),
            "ArrowLeft" => Ok(ControlKey::ArrowLeft),
            "ArrowRight" => Ok(ControlKey::ArrowRight),
            _ => Err(ControlKeyError::NotAControlKey),
        }
    }
}

impl GameLogic {
    pub fn on_keydown(&mut self, event: &KeyboardEvent) {
        if let Ok(key) = event.key().parse::<ControlKey>() {
            event.prevent_default();

            match key {
                ControlKey::ArrowUp => {
                    self.direction = std::f64::consts::FRAC_PI_2;
                }
                ControlKey::ArrowDown => {
                    self.direction = std::f64::consts::FRAC_PI_2 + std::f64::consts::PI;
                }
                ControlKey::ArrowLeft => {
                    self.direction = std::f64::consts::PI;
                }
                ControlKey::ArrowRight => {
                    self.direction = 0.0;
                }
            }
        }
    }

    pub fn on_keyup(&mut self, event: &KeyboardEvent) {
        if let Ok(_) = event.key().parse::<ControlKey>() {
            event.prevent_default();
        }
    }

    pub fn setup(&mut self) {
        self.last_frame_time = Date::now();
    }

    pub fn update(&mut self) {
        let delta = self.compute_delta();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        renderer.clear_frame();
        renderer.draw_arrow(self.direction);
    }

    pub fn is_running(&self) -> bool {
        return true;
    }

    fn compute_delta(&mut self) -> f64 {
        let frame_time = Date::now();
        let delta = frame_time - self.last_frame_time;

        self.last_frame_time = frame_time;

        delta
    }
}
