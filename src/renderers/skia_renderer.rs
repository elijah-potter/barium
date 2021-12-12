use glam::{UVec2, Vec2};
use image::{Rgba, RgbaImage};
use tiny_skia::{LineCap, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, PixmapRef, Rect};

use crate::{
    canvas::{CanvasElement, CanvasElementPostEffect, CanvasElementVariant},
    color::Color,
    renderer::Renderer,
    Transform,
};

#[derive(Clone)]
/// Settings for [SkiaRenderer].
pub struct SkiaRendererSettings {
    pub size: UVec2,
    pub anti_alias: bool,
    pub background_color: Option<Color>,
}

impl Default for SkiaRendererSettings {
    fn default() -> Self {
        Self {
            size: UVec2::splat(1000),
            anti_alias: true,
            background_color: None,
        }
    }
}

/// A renderer to raster images that uses [tiny_skia](https://github.com/RazrFalcon/tiny-skia).
pub struct SkiaRenderer {
    anti_alias: bool,
    canvas: Pixmap,
}

impl SkiaRenderer {
    fn default_stroke(width: f32) -> tiny_skia::Stroke {
        tiny_skia::Stroke {
            width,
            line_cap: LineCap::Round,
            ..Default::default()
        }
    }

    fn render(
        mut canvas: PixmapMut,
        element: &CanvasElement,
    ) {
        
    }
}

impl Renderer for SkiaRenderer {
    type Settings = SkiaRendererSettings;
    type Output = Pixmap;

    fn new(settings: Self::Settings) -> Self {
        let mut canvas = Pixmap::new(settings.size.x, settings.size.y).unwrap();

        if let Some(background_color) = settings.background_color {
            let rgba: Rgba<u8> = background_color.into();
            canvas.fill(tiny_skia::Color::from_rgba8(
                rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3],
            ));
        }

        Self {
            anti_alias: settings.anti_alias,
            canvas,
        }
    }

    fn render(&mut self, element: &CanvasElement) {
        Self::render(
            self.canvas.as_mut(),
            element,
            Transform::one(),
            self.anti_alias,
        );
    }

    fn finalize(self) -> Self::Output {
        self.canvas
    }
}

pub trait ToRgbaImage {
    fn to_rgba_image(self) -> RgbaImage;
}

impl ToRgbaImage for Pixmap {
    fn to_rgba_image(self) -> RgbaImage {
        RgbaImage::from_raw(self.width(), self.height(), self.take()).unwrap()
    }
}
