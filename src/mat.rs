use std::ops::Mul;

use crate::{vec2, Vec2};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Mat2x3 {
    v: [[f32; 3]; 2],
}

impl Mat2x3 {
    /// Identity transform.
    pub const IDENTITY: Self = Self::new([1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

    /// Create a new 2x3 matrix.
    #[inline]
    pub const fn new(a: [f32; 3], b: [f32; 3]) -> Self {
        Self {
            v: [[a[0], a[1], a[2]], [b[0], b[1], b[2]]],
        }
    }

    /// Matrix for converting the screen coordinate system to the backend coordinate system.
    #[inline]
    pub fn default_proj(width: f32, height: f32) -> Self {
        Self::new([2.0 / width, 0.0, -1.0], [0.0, -2.0 / height, 1.0])
    }

    /// Multiply the projection matrix (self) by the transform matrix.
    #[inline]
    pub fn mul_transform(&self, transform: Self) -> Self {
        // this is an optimized way of doing this
        let (x, y) = (self.v[0][0], self.v[1][1]);
        Self::new(
            [
                x * transform.v[0][0],
                x * transform.v[0][1],
                x * transform.v[0][2] + self.v[0][2],
            ],
            [
                y * transform.v[1][0],
                y * transform.v[1][1],
                y * transform.v[1][2] + self.v[1][2],
            ],
        )
    }

    #[inline]
    pub fn mul_vec2(&self, v: Vec2) -> Vec2 {
        vec2(
            self.v[0][0] * v.x + self.v[0][1] * v.y + self.v[0][2],
            self.v[1][0] * v.x + self.v[1][1] * v.y + self.v[1][2],
        )
    }

    #[inline]
    pub fn transform_vec2s(&self, v: &mut [Vec2]) {
        v.iter_mut().for_each(|v| *v = self.mul_vec2(*v));
    }
}

impl Mul for Mat2x3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul_transform(rhs)
    }
}
