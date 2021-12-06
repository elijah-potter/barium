use std::{
    num::ParseIntError,
    ops::{Add, Div, Mul, Rem, Sub},
};

use glam::Vec4;
use image::{Rgb, Rgba};

/// Color of an object.
///
/// Contains RGBA in floating point. 0.0 is black, 1.0 is white.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Color {
    inner: Vec4,
}

impl Color {
    pub fn white() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn black() -> Self {
        Color::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn transparent() -> Self {
        Color::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            inner: Vec4::new(r, g, b, a),
        }
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.inner.x
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.inner.y
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.inner.z
    }

    #[inline]
    pub fn a(&self) -> f32 {
        self.inner.w
    }

    #[inline]
    pub fn r_mut(&mut self) -> &mut f32 {
        &mut self.inner.x
    }

    #[inline]
    pub fn g_mut(&mut self) -> &mut f32 {
        &mut self.inner.y
    }

    #[inline]
    pub fn b_mut(&mut self) -> &mut f32 {
        &mut self.inner.z
    }

    #[inline]
    pub fn a_mut(&mut self) -> &mut f32 {
        &mut self.inner.w
    }

    /// Get as a hex string.
    ///
    /// Alpha channel is optional
    pub fn as_hex(&self, include_alpha: bool) -> String {
        if include_alpha {
            format!(
                "#{:02X}{:02X}{:02X}{:02X}",
                (self.r() * 255.0) as u8,
                (self.g() * 255.0) as u8,
                (self.b() * 255.0) as u8,
                (self.a() * 255.0) as u8
            )
        } else {
            format!(
                "#{:02X}{:02X}{:02X}",
                (self.r() * 255.0) as u8,
                (self.g() * 255.0) as u8,
                (self.b() * 255.0) as u8
            )
        }
    }

    /// Parses a hex string.
    ///
    /// The hex *can* include `#` or `0x` at the beginning, but it is not required.
    /// If the alpha channel is not included, it will default to 1.0
    pub fn from_hex(hex: &str) -> Result<Self, ParseIntError> {
        let mut start_index = if hex.starts_with('#') {
            1
        } else if hex.starts_with("0x") {
            2
        } else {
            0
        };

        let r = u8::from_str_radix(&hex[start_index..start_index + 2], 16)? as f32 / 255.0;
        start_index += 2;
        let g = u8::from_str_radix(&hex[start_index..start_index + 2], 16)? as f32 / 255.0;
        start_index += 2;
        let b = u8::from_str_radix(&hex[start_index..start_index + 2], 16)? as f32 / 255.0;

        start_index += 2;

        if start_index >= hex.len(){
            return Ok(Self::new(r, g, b, 1.0));
        }

        let a = u8::from_str_radix(&hex[start_index..start_index + 2], 16)? as f32 / 255.0;

        Ok(Self::new(r, g, b, a))
    }
}

impl From<Rgb<u8>> for Color {
    fn from(rgb: Rgb<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                1.0,
            ),
        }
    }
}

impl From<&Rgb<u8>> for Color {
    fn from(rgb: &Rgb<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                1.0,
            ),
        }
    }
}

impl From<Rgba<u8>> for Color {
    fn from(rgb: Rgba<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                rgb.0[3] as f32 / 255.0,
            ),
        }
    }
}

impl From<&Rgba<u8>> for Color {
    fn from(rgb: &Rgba<u8>) -> Self {
        Color {
            inner: Vec4::new(
                rgb.0[0] as f32 / 255.0,
                rgb.0[1] as f32 / 255.0,
                rgb.0[2] as f32 / 255.0,
                rgb.0[3] as f32 / 255.0,
            ),
        }
    }
}

impl From<Color> for Rgba<u8> {
    fn from(color: Color) -> Self {
        Rgba([
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8,
            (color.a() * 255.0) as u8,
        ])
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            inner: self.inner * rhs,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            inner: self.inner / rhs,
        }
    }
}

impl Rem<f32> for Color {
    type Output = Color;

    fn rem(self, rhs: f32) -> Self::Output {
        Color {
            inner: self.inner % rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            inner: rhs.inner * self,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner + rhs.inner,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner - rhs.inner,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner * rhs.inner,
        }
    }
}

impl Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner / rhs.inner,
        }
    }
}

impl Rem<Color> for Color {
    type Output = Color;

    fn rem(self, rhs: Color) -> Self::Output {
        Color {
            inner: self.inner % rhs.inner,
        }
    }
}
