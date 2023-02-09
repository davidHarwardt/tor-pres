use std::ops::{Add, Mul};

use nannou::prelude::*;

pub trait ColorExt: Sized {
    fn with_alpha(self, a: f32) -> Rgba8;
}
impl<T: Into<Rgb8>> ColorExt for T {
    fn with_alpha(self, a: f32) -> Rgba8 {
        let v: Rgb8 = self.into();
        let (r, g, b) = v.into_components();
        Rgba::from_components((r, g, b, (a * 255.0) as _))
    }
}

pub fn lerp<T: Add<T, Output = T> + Mul<f32, Output = T>>(a: T, b: T, v: f32) -> T { a * (1.0 - v) + b * v }


