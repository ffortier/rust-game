use std::ops::Mul;

use super::Vec4;

#[derive(Debug, Clone, Copy, Default)]
pub struct Mat4 {
    pub value: [[f64; 4]; 4],
}

impl Mat4 {}

impl AsRef<[[f64; 4]; 4]> for Mat4 {
    fn as_ref(&self) -> &[[f64; 4]; 4] {
        &self.value
    }
}

impl From<[[f64; 4]; 4]> for Mat4 {
    fn from(value: [[f64; 4]; 4]) -> Self {
        Self { value }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Self::Output {
        Vec4::new(
            self.value[0][0] * rhs.value[0]
                + self.value[0][1] * rhs.value[1]
                + self.value[0][2] * rhs.value[2]
                + self.value[0][3] * rhs.value[3],
            self.value[1][0] * rhs.value[0]
                + self.value[1][1] * rhs.value[1]
                + self.value[1][2] * rhs.value[2]
                + self.value[1][3] * rhs.value[3],
            self.value[2][0] * rhs.value[0]
                + self.value[2][1] * rhs.value[1]
                + self.value[2][2] * rhs.value[2]
                + self.value[2][3] * rhs.value[3],
            self.value[3][0] * rhs.value[0]
                + self.value[3][1] * rhs.value[1]
                + self.value[3][2] * rhs.value[2]
                + self.value[3][3] * rhs.value[3],
        )
    }
}
