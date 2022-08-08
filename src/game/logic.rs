use std::str::FromStr;

use js_sys::Date;
use thiserror::Error;
use web_sys::KeyboardEvent;

use super::{math::Vec4, renderer::Renderer};

#[derive(Debug, Default)]
pub struct GameLogic {
    last_frame_time: f64,
    rotate_x: f64,
    rotate_y: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
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
                    self.rotate_y = 1.0;
                }
                ControlKey::ArrowDown => {
                    self.rotate_y = -1.0;
                }
                ControlKey::ArrowLeft => {
                    self.rotate_x = -1.0;
                }
                ControlKey::ArrowRight => {
                    self.rotate_x = 1.0;
                }
            }
        }
    }

    pub fn on_keyup(&mut self, event: &KeyboardEvent) {
        if let Ok(key) = event.key().parse::<ControlKey>() {
            event.prevent_default();

            match key {
                ControlKey::ArrowUp => {
                    self.rotate_y = 0.0;
                }
                ControlKey::ArrowDown => {
                    self.rotate_y = 0.0;
                }
                ControlKey::ArrowLeft => {
                    self.rotate_x = 0.0;
                }
                ControlKey::ArrowRight => {
                    self.rotate_x = 0.0;
                }
            }
        }
    }

    pub fn setup(&mut self) {
        self.last_frame_time = Date::now();
        self.rotate_x = 0.0;
        self.rotate_y = 0.0;
        self.rotation_x = 0.0;
        self.rotation_y = 0.0;
        self.rotation_z = 0.0;
    }

    pub fn draw(&mut self, renderer: &mut Renderer) {
        let _delta = self.compute_delta();

        if self.rotate_x != 0.0 && self.rotate_y != 0.0 {
            self.rotation_z += 0.1 * self.rotate_y;
        } else if self.rotate_x != 0.0 {
            self.rotation_x += 0.1 * self.rotate_x;
        } else if self.rotate_y != 0.0 {
            self.rotation_y += 0.1 * self.rotate_y;
        }

        renderer.clear_frame();

        let points = [
            (0, 0, 0),
            (1, 0, 0),
            (1, 1, 0),
            (0, 1, 0),
            (0, 0, 1),
            (1, 0, 1),
            (1, 1, 1),
            (0, 1, 1),
        ];

        for (i, point) in points.into_iter().enumerate() {
            renderer.point(&point, self.rotation_x, self.rotation_y, self.rotation_z);
        }

        for i in 0..4 {
            renderer.line(
                &points[i],
                &points[(i + 1) % 4],
                self.rotation_x,
                self.rotation_y,
                self.rotation_z,
            );
            renderer.line(
                &points[i + 4],
                &points[(i + 1) % 4 + 4],
                self.rotation_x,
                self.rotation_y,
                self.rotation_z,
            );
            renderer.line(
                &points[i],
                &points[i + 4],
                self.rotation_x,
                self.rotation_y,
                self.rotation_z,
            );
        }
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
