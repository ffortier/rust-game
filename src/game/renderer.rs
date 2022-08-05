use std::f64::consts::PI;

use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::CanvasRenderingContext2d;

pub struct Renderer {
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    center_x: f64,
    center_y: f64,

    projection: [[f64; 3]; 2],
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        Self {
            context,
            width: width as f64,
            height: height as f64,
            center_x: width as f64 / 2.0,
            center_y: height as f64 / 2.0,
            projection: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
        }
    }

    pub fn rotate_x(&mut self, increment: f64) {
        self.rotation_x += increment;
    }

    pub fn rotate_y(&mut self, increment: f64) {
        self.rotation_y += increment;
    }

    pub fn rotate_z(&mut self, increment: f64) {
        self.rotation_z += increment;
    }

    pub fn clear_frame(&self) {
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    pub fn point(&self, point: impl Into<Point>) {
        let (x, y) = self.transform_2d(point.into());

        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.begin_path();
        self.context.arc(x, y, 5.0, 0.0, PI * 2.0).unwrap_throw();
        self.context.fill();
        self.context.close_path();
    }

    pub fn line(&self, p1: impl Into<Point>, p2: impl Into<Point>) {
        let (x1, y1) = self.transform_2d(p1.into());
        let (x2, y2) = self.transform_2d(p2.into());

        self.context.set_stroke_style(&JsValue::from_str("pink"));
        self.context.begin_path();
        self.context.move_to(x1, y1);
        self.context.line_to(x2, y2);
        self.context.stroke();
    }

    fn transform_2d(&self, point: Point) -> (f64, f64) {
        let [x, y] = point
            .apply_rotation(self.rotation_x, self.rotation_y, self.rotation_z)
            .project_2d(&self.projection);

        self.normalize(x, y)
    }

    fn normalize(&self, x: f64, y: f64) -> (f64, f64) {
        (x + self.center_x, self.height - y - self.center_y)
    }
}

pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn project_2d(&self, projection: &[[f64; 3]; 2]) -> [f64; 2] {
        [
            projection[0][0] * self.x + projection[0][1] * self.y + projection[0][2] * self.z,
            projection[1][0] * self.x + projection[1][1] * self.y + projection[1][2] * self.z,
        ]
    }

    pub fn apply_rotation(&self, rotation_x: f64, rotation_y: f64, rotation_z: f64) -> Self {
        let Point { x, y, z } = self;

        // rotation z
        let (x, y, z) = (
            rotation_z.cos() * x - rotation_z.sin() * y + 0.0 * z,
            rotation_z.sin() * x + rotation_z.cos() * y + 0.0 * z,
            0.0 * x + 0.0 * y + 1.0 * z,
        );

        // rotation y
        let (x, y, z) = (
            1.0 * x + 0.0 * y + 0.0 * z,
            0.0 * x + rotation_y.cos() * y - rotation_y.sin() * z,
            0.0 * x + rotation_y.sin() * y + rotation_y.cos() * z,
        );

        // rotation x
        let (x, y, z) = (
            rotation_x.cos() * x + 0.0 * y - rotation_x.sin() * z,
            0.0 * z + 1.0 * y + 0.0 * z,
            rotation_x.sin() * x + 0.0 * y + rotation_x.cos() * z,
        );

        Self { x, y, z }
    }
}

impl From<&(f64, f64, f64)> for Point {
    fn from((x, y, z): &(f64, f64, f64)) -> Self {
        Self {
            x: *x,
            y: *y,
            z: *z,
        }
    }
}

impl From<&(i32, i32, i32)> for Point {
    fn from((x, y, z): &(i32, i32, i32)) -> Self {
        Self {
            x: *x as f64,
            y: *y as f64,
            z: *z as f64,
        }
    }
}
