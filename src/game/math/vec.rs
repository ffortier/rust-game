use std::ops::Mul;

use super::Mat4;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec4 {
    pub value: [f64; 4],
}

impl Vec4 {
    pub fn x(&self) -> f64 {
        self.value[0]
    }

    pub fn y(&self) -> f64 {
        self.value[1]
    }

    pub fn z(&self) -> f64 {
        self.value[2]
    }

    pub fn w(&self) -> f64 {
        self.value[3]
    }
}

impl Mul<[[f64; 4]; 2]> for Vec4 {
    type Output = [f64; 2];

    fn mul(self, rhs: [[f64; 4]; 2]) -> Self::Output {
        [
            self.value[0] * rhs[0][0]
                + self.value[1] * rhs[0][1]
                + self.value[2] * rhs[0][2]
                + self.value[3] * rhs[0][3],
            self.value[0] * rhs[1][0]
                + self.value[1] * rhs[1][1]
                + self.value[2] * rhs[1][2]
                + self.value[3] * rhs[1][3],
        ]
    }
}

impl AsRef<[f64; 4]> for Vec4 {
    fn as_ref(&self) -> &[f64; 4] {
        &self.value
    }
}

impl From<[f64; 4]> for Vec4 {
    fn from(value: [f64; 4]) -> Self {
        Self { value }
    }
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            value: [x, y, z, w],
        }
    }
}

impl Mul<Mat4> for Vec4 {
    type Output = Vec4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        rhs * self
    }
}

impl From<(i32, i32, i32)> for Vec4 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self {
            value: [x as f64, y as f64, z as f64, 1.0],
        }
    }
}

impl From<&(i32, i32, i32)> for Vec4 {
    fn from((x, y, z): &(i32, i32, i32)) -> Self {
        Self {
            value: [*x as f64, *y as f64, *z as f64, 1.0],
        }
    }
}
