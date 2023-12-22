use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

/// A trait implemented for all floating point types ([f32] & [f64]).
pub trait Real:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
    const ONE: Self;

    /// Convert the float value to [f32].
    fn from_f32(f: f32) -> Self;
    /// Convert the float value to [f64].
    fn from_f64(f: f64) -> Self;

    /// Create a float value from [f32].
    fn to_f32(self) -> f32;
    /// Create a float value from [f64].
    fn to_f64(self) -> f64;
}

impl Real for f32 {
    const ONE: Self = 1.0;

    fn from_f32(f: f32) -> Self {
        f
    }
    fn from_f64(f: f64) -> Self {
        f as f32
    }

    fn to_f32(self) -> f32 {
        self
    }
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Real for f64 {
    const ONE: Self = 1.0;

    fn from_f32(f: f32) -> Self {
        f as f64
    }
    fn from_f64(f: f64) -> Self {
        f
    }

    fn to_f32(self) -> f32 {
        self as f32
    }
    fn to_f64(self) -> f64 {
        self
    }
}

#[inline(always)]
pub fn lerp<R, T>(range: impl Into<RangeInclusive<R>>, t: T) -> R
where
    T: Real + Mul<R, Output = R>,
    R: Copy + Add<R, Output = R>,
{
    let range = range.into();
    (T::ONE - t) * *range.start() + t * *range.end()
}
