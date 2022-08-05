use std::str::FromStr;

use js_sys::Date;
use thiserror::Error;
use web_sys::KeyboardEvent;

use super::renderer::Renderer;

#[derive(Debug, Default)]
pub struct GameLogic {
    last_frame_time: f64,
    rotation_x: f64,
    rotation_y: f64,
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
                    self.rotation_y = 1.0;
                }
                ControlKey::ArrowDown => {
                    self.rotation_y = -1.0;
                }
                ControlKey::ArrowLeft => {
                    self.rotation_x = -1.0;
                }
                ControlKey::ArrowRight => {
                    self.rotation_x = 1.0;
                }
            }
        }
    }

    pub fn on_keyup(&mut self, event: &KeyboardEvent) {
        if let Ok(key) = event.key().parse::<ControlKey>() {
            event.prevent_default();

            match key {
                ControlKey::ArrowUp => {
                    self.rotation_y = 0.0;
                }
                ControlKey::ArrowDown => {
                    self.rotation_y = 0.0;
                }
                ControlKey::ArrowLeft => {
                    self.rotation_x = 0.0;
                }
                ControlKey::ArrowRight => {
                    self.rotation_x = 0.0;
                }
            }
        }
    }

    pub fn setup(&mut self) {
        self.last_frame_time = Date::now();
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        let _delta = self.compute_delta();

        if self.rotation_x != 0.0 && self.rotation_y != 0.0 {
            renderer.rotate_z(0.1 * self.rotation_y);
        } else if self.rotation_x != 0.0 {
            renderer.rotate_x(0.1 * self.rotation_x);
        } else if self.rotation_y != 0.0 {
            renderer.rotate_y(0.1 * self.rotation_y);
        }

        renderer.clear_frame();

        let p1 = (-50, -50, -50);
        let p2 = (50, -50, -50);
        let p3 = (50, 50, -50);
        let p4 = (-50, 50, -50);
        let p5 = (-50, -50, 50);
        let p6 = (50, -50, 50);
        let p7 = (50, 50, 50);
        let p8 = (-50, 50, 50);

        renderer.point(&p1);
        renderer.point(&p2);
        renderer.point(&p3);
        renderer.point(&p4);
        renderer.point(&p5);
        renderer.point(&p6);
        renderer.point(&p7);
        renderer.point(&p8);

        renderer.line(&p1, &p2);
        renderer.line(&p2, &p3);
        renderer.line(&p3, &p4);
        renderer.line(&p4, &p1);
        renderer.line(&p5, &p6);
        renderer.line(&p6, &p7);
        renderer.line(&p7, &p8);
        renderer.line(&p8, &p5);
        renderer.line(&p1, &p5);
        renderer.line(&p2, &p6);
        renderer.line(&p3, &p7);
        renderer.line(&p4, &p8);
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
