use std::f64::consts::{FRAC_PI_2, PI};

use wasm_bindgen::{JsValue, UnwrapThrowExt};
use web_sys::CanvasRenderingContext2d;

use super::math::{
    new_perspective, new_rotation_x, new_rotation_y, new_rotation_z, new_translation, Mat4, Vec4,
};

pub struct Renderer {
    context: CanvasRenderingContext2d,
    width: f64,
    height: f64,
    fov: f64,
    z_near: f64,
    z_far: f64,
    perspective: Mat4,
}

impl Renderer {
    pub fn new(context: CanvasRenderingContext2d, width: u32, height: u32) -> Self {
        let width = width as f64;
        let height = height as f64;

        Self {
            context,
            width,
            height,
            fov: FRAC_PI_2,
            z_near: 0.1,
            z_far: 1000.0,
            perspective: new_perspective(width, height, FRAC_PI_2, 0.1, 1000.0),
        }
    }

    pub fn clear_frame(&self) {
        self.context.set_fill_style(&JsValue::from_str("black"));
        self.context
            .fill_rect(0.0, 0.0, self.width as f64, self.height as f64);
    }

    pub fn point(&self, point: impl Into<Vec4>, rotation_x: f64, rotation_y: f64, rotation_z: f64) {
        let (x, y) = self.transform_2d(point.into(), rotation_x, rotation_y, rotation_z);

        self.context.set_fill_style(&JsValue::from_str("white"));
        self.context.begin_path();
        self.context.arc(x, y, 5.0, 0.0, PI * 2.0).unwrap_throw();
        self.context.fill();
        self.context.close_path();
    }

    pub fn line(
        &self,
        p1: impl Into<Vec4>,
        p2: impl Into<Vec4>,
        rotation_x: f64,
        rotation_y: f64,
        rotation_z: f64,
    ) {
        let (x1, y1) = self.transform_2d(p1.into(), rotation_x, rotation_y, rotation_z);
        let (x2, y2) = self.transform_2d(p2.into(), rotation_x, rotation_y, rotation_z);

        self.context.set_stroke_style(&JsValue::from_str("pink"));
        self.context.begin_path();
        self.context.move_to(x1, y1);
        self.context.line_to(x2, y2);
        self.context.stroke();
    }

    pub fn set_fov(&mut self, fov: f64) {
        self.fov = fov;
        self.perspective =
            new_perspective(self.width, self.height, self.fov, self.z_near, self.z_far);
    }

    pub fn fov(&self) -> f64 {
        self.fov
    }

    pub fn set_z_near(&mut self, z_near: f64) {
        self.z_near = z_near;
        self.perspective =
            new_perspective(self.width, self.height, self.fov, self.z_near, self.z_far);
    }

    pub fn z_near(&self) -> f64 {
        self.z_near
    }

    pub fn set_z_far(&mut self, z_far: f64) {
        self.z_far = z_far;
        self.perspective =
            new_perspective(self.width, self.height, self.fov, self.z_near, self.z_far);
    }

    pub fn z_far(&self) -> f64 {
        self.z_far
    }

    fn transform_2d(
        &self,
        point: Vec4,
        rotation_x: f64,
        rotation_y: f64,
        rotation_z: f64,
    ) -> (f64, f64) {
        let m1 = new_rotation_x(rotation_x);
        let m2 = new_rotation_y(rotation_y);
        let m3 = new_rotation_z(rotation_z);
        let m5 = new_translation(0.0, 0.0, 3.0);

        let r = point * m1 * m2 * m3 * m5;
        let r = r * self.perspective;
        let projection = [[1.0 / r.w(), 0.0, 0.0, 0.0], [0.0, 1.0 / r.w(), 0.0, 0.0]];

        let [x, y] = r * projection;

        (
            x * 0.5 * self.width + 0.5 * self.width,
            y * 0.5 * self.height + 0.5 * self.height,
        )
    }
}
