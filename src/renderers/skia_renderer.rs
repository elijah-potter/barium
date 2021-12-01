use glam::UVec2;
use image::{Rgba, RgbaImage};
use tiny_skia::{
    LineCap, Paint, PathBuilder, Pixmap, PixmapMut, PixmapPaint, PixmapRef, Rect, Transform,
};

use crate::{
    canvas::{CanvasElement, CanvasElementPostEffect, CanvasElementVariant},
    color::Color,
    renderer::Renderer,
};

#[derive(Clone)]
/// Settings for [SkiaRenderer].
pub struct SkiaRendererSettings {
    pub size: UVec2,
    pub background_color: Option<Color>,
}

/// A renderer to raster images that uses [tiny_skia](https://github.com/RazrFalcon/tiny-skia).
pub struct SkiaRenderer {
    canvas: Pixmap,
}

impl SkiaRenderer {
    fn default_stroke(width: f32) -> tiny_skia::Stroke {
        tiny_skia::Stroke {
            width,
            miter_limit: Default::default(),
            line_cap: LineCap::Round,
            line_join: Default::default(),
            dash: None,
        }
    }

    fn render(mut canvas: PixmapMut, element: &CanvasElement) {
        let mut temp_canvas = if element.post_effects.is_empty() {
            canvas.to_owned()
        } else {
            Pixmap::new(canvas.width(), canvas.height()).unwrap()
        };

        match &element.variant {
            CanvasElementVariant::Blank => (),
            CanvasElementVariant::PolyLine { points, stroke } => {
                if let Some(first) = points.first() {
                    let mut path = PathBuilder::new();
                    path.move_to(first.x, first.y);
                    for point in points.iter().skip(1) {
                        path.line_to(point.x, point.y);
                    }
                    let path = path.finish().unwrap();

                    let mut paint = Paint::default();
                    let rgba = Rgba::<u8>::from(stroke.color);
                    paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                    paint.anti_alias = true;

                    temp_canvas.stroke_path(
                        &path,
                        &paint,
                        &Self::default_stroke(stroke.width),
                        Transform::identity(),
                        None,
                    );
                }
            }
            CanvasElementVariant::Ellipse {
                center,
                radius,
                fill,
                stroke,
            } => {
                let path = PathBuilder::from_oval(
                    Rect::from_ltrb(
                        center.x - radius.x,
                        center.y - radius.y,
                        center.x + radius.x,
                        center.y + radius.y,
                    )
                    .unwrap(),
                )
                .unwrap();

                if let Some(fill) = fill {
                    let mut paint = Paint::default();
                    let rgba = Rgba::<u8>::from(*fill);
                    paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                    paint.anti_alias = true;

                    temp_canvas.fill_path(
                        &path,
                        &paint,
                        tiny_skia::FillRule::Winding,
                        Transform::identity(),
                        None,
                    );
                }

                if let Some(stroke) = stroke {
                    let mut paint = Paint::default();
                    let rgba = Rgba::<u8>::from(stroke.color);
                    paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                    paint.anti_alias = true;

                    temp_canvas.stroke_path(
                        &path,
                        &paint,
                        &Self::default_stroke(stroke.width),
                        Transform::identity(),
                        None,
                    );
                }
            }
            CanvasElementVariant::Polygon {
                points,
                fill,
                stroke,
            } => {
                if let Some(first) = points.first() {
                    let mut path = PathBuilder::new();
                    path.move_to(first.x, first.y);
                    for point in points.iter().skip(1) {
                        path.line_to(point.x, point.y);
                    }

                    if let Some(fill) = fill {
                        let path = path.clone().finish().unwrap();

                        let mut paint = Paint::default();
                        let rgba = Rgba::<u8>::from(*fill);
                        paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                        paint.anti_alias = true;

                        temp_canvas.fill_path(
                            &path,
                            &paint,
                            tiny_skia::FillRule::Winding,
                            Transform::identity(),
                            None,
                        );
                    }

                    if let Some(stroke) = stroke {
                        path.close();
                        let path = path.finish().unwrap();

                        let mut paint = Paint::default();
                        let rgba = Rgba::<u8>::from(stroke.color);
                        paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                        paint.anti_alias = true;

                        temp_canvas.stroke_path(
                            &path,
                            &paint,
                            &Self::default_stroke(stroke.width),
                            Transform::identity(),
                            None,
                        );
                    }
                }
            }
            CanvasElementVariant::Cluster { children } => {
                for child in children {
                    Self::render(temp_canvas.as_mut(), child);
                }
            }
        }

        if element.post_effects.is_empty() {
            canvas.data_mut().copy_from_slice(temp_canvas.data());
        } else {
            let mut image = temp_canvas.to_rgba_image();

            for effect in &element.post_effects {
                match effect {
                    CanvasElementPostEffect::GaussianBlur { std_dev } => {
                        image = image::imageops::blur(&image, *std_dev);
                    }
                }
            }

            canvas.draw_pixmap(
                0,
                0,
                PixmapRef::from_bytes(image.as_raw(), image.width(), image.height()).unwrap(),
                &PixmapPaint::default(),
                Transform::identity(),
                None,
            );
        }
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

        Self { canvas }
    }

    fn render(&mut self, element: &CanvasElement) {
        Self::render(self.canvas.as_mut(), element);
    }

    fn finalize(self) -> Self::Output {
        self.canvas
    }
}

pub trait ToRgbaImage{
    fn to_rgba_image(self) -> RgbaImage;
}

impl ToRgbaImage for Pixmap{
    fn to_rgba_image(self) -> RgbaImage {
        RgbaImage::from_raw(
            self.width(),
            self.height(),
            self.take(),
        )
        .unwrap()
    }
}