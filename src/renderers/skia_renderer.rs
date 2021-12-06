use glam::{UVec2, Vec2};
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
///
/// Note: [Zoom](SkiaRendererSettings) cannot be <= 0.0.
pub struct SkiaRendererSettings {
    pub size: UVec2,
    pub zoom: f32,
    pub translate: Vec2,
    pub anti_alias: bool,
    pub background_color: Option<Color>,
}

impl Default for SkiaRendererSettings {
    fn default() -> Self {
        Self {
            size: UVec2::splat(1000),
            zoom: 1.0,
            translate: Vec2::ZERO,
            anti_alias: true,
            background_color: None,
        }
    }
}

/// Variables needed to transform elements within the canvas.
struct TransformMatrix {
    translate: Vec2,
    size: Vec2,
    zoom: f32,
}

impl TransformMatrix {
    fn transform(&self, point: Vec2) -> Vec2 {
        (point + self.translate - self.size) * self.zoom + self.size
    }
}

/// A renderer to raster images that uses [tiny_skia](https://github.com/RazrFalcon/tiny-skia).
pub struct SkiaRenderer {
    transform: TransformMatrix,
    anti_alias: bool,
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

    fn render(
        transform: &TransformMatrix,
        anti_alias: bool,
        mut canvas: PixmapMut,
        element: &CanvasElement,
    ) {
        let mut temp_canvas = if element.post_effects.is_empty() {
            canvas.to_owned()
        } else {
            Pixmap::new(canvas.width(), canvas.height()).unwrap()
        };

        match &element.variant {
            CanvasElementVariant::Blank => (),
            CanvasElementVariant::PolyLine { points, stroke } => {
                if let Some(first) = points.first() {
                    let first = transform.transform(*first);

                    // Build path
                    let mut path = PathBuilder::new();
                    path.move_to(first.x, first.y);
                    for point in points.iter().skip(1) {
                        let point = transform.transform(*point);
                        path.line_to(point.x, point.y);
                    }
                    let path = path.finish().unwrap();

                    let mut paint = Paint::default();
                    let rgba = Rgba::<u8>::from(stroke.color);
                    paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                    paint.anti_alias = anti_alias;

                    temp_canvas.stroke_path(
                        &path,
                        &paint,
                        &Self::default_stroke(stroke.width * transform.zoom),
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
                let center = transform.transform(*center);
                let radius = *radius * transform.zoom;

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
                    paint.anti_alias = anti_alias;

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
                    paint.anti_alias = anti_alias;

                    temp_canvas.stroke_path(
                        &path,
                        &paint,
                        &Self::default_stroke(stroke.width * transform.zoom),
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
                    let first = transform.transform(*first);

                    let mut path = PathBuilder::new();
                    path.move_to(first.x, first.y);
                    for point in points.iter().skip(1) {
                        let point = transform.transform(*point);
                        path.line_to(point.x, point.y);
                    }

                    if let Some(fill) = fill {
                        let path = path.clone().finish().unwrap();

                        let mut paint = Paint::default();
                        let rgba = Rgba::<u8>::from(*fill);
                        paint.set_color_rgba8(rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3]);
                        paint.anti_alias = anti_alias;

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
                        paint.anti_alias = anti_alias;

                        temp_canvas.stroke_path(
                            &path,
                            &paint,
                            &Self::default_stroke(stroke.width * transform.zoom),
                            Transform::identity(),
                            None,
                        );
                    }
                }
            }
            CanvasElementVariant::Cluster { children } => {
                for child in children {
                    Self::render(transform, anti_alias, temp_canvas.as_mut(), child);
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
        if settings.zoom <= 0.0 {
            panic!("Zoom cannot be <= 0.0.")
        }

        let mut canvas = Pixmap::new(settings.size.x, settings.size.y).unwrap();

        if let Some(background_color) = settings.background_color {
            let rgba: Rgba<u8> = background_color.into();
            canvas.fill(tiny_skia::Color::from_rgba8(
                rgba.0[0], rgba.0[1], rgba.0[2], rgba.0[3],
            ));
        }

        Self {
            transform: TransformMatrix {
                translate: settings.translate,
                size: settings.size.as_vec2() / 2.0,
                zoom: settings.zoom,
            },
            anti_alias: settings.anti_alias,
            canvas,
        }
    }

    fn render(&mut self, element: &CanvasElement) {
        Self::render(
            &self.transform,
            self.anti_alias,
            self.canvas.as_mut(),
            element,
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
