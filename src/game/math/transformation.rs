use super::Mat4;

pub fn new_translation(x: f64, y: f64, z: f64) -> Mat4 {
    Mat4 {
        value: [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn new_rotation_x(angle: f64) -> Mat4 {
    Mat4 {
        value: [
            [angle.cos(), 0.0, angle.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-angle.sin(), 0.0, angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn new_rotation_y(angle: f64) -> Mat4 {
    Mat4 {
        value: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, angle.cos(), -angle.sin(), 0.0],
            [0.0, angle.sin(), angle.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn new_rotation_z(angle: f64) -> Mat4 {
    Mat4 {
        value: [
            [angle.cos(), -angle.sin(), 0.0, 0.0],
            [angle.sin(), angle.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}

pub fn new_perspective(w: f64, h: f64, fov: f64, z_near: f64, z_far: f64) -> Mat4 {
    let a = h / w;
    let f = 1.0 / (fov * 0.5).tan();
    let q1 = z_far / (z_far - z_near);
    let q2 = (-z_far * z_near) / (z_far - z_near);

    Mat4 {
        value: [
            [a * f, 0.0, 0.0, 0.0],
            [0.0, f, 0.0, 0.0],
            [0.0, 0.0, q1, q2],
            [0.0, 0.0, 1.0, 0.0],
        ],
    }
}
