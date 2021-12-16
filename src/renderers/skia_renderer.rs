use glam::{UVec2, Vec2};
use image::RgbaImage;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};

use crate::{Color, Renderer, Shape};

#[derive(Default, Clone, Copy)]
pub struct SkiaRendererSettings {
    /// Size of the output image.
    pub size: UVec2,
    /// An optional background color.
    pub background: Option<Color>,
    /// Use antialiasing
    pub antialias: bool,
}

pub struct SkiaRenderer {
    antialias: bool,
    scale: f32,
    canvas: Pixmap,
}

impl Renderer for SkiaRenderer {
    type Settings = SkiaRendererSettings;

    type Output = RgbaImage;

    fn new(settings: Self::Settings) -> Self {
        let mut canvas = Pixmap::new(settings.size.x, settings.size.y).unwrap();

        if let Some(background) = settings.background {
            canvas.fill(background.into());
        }

        Self {
            antialias: settings.antialias,
            scale: settings.size.x as f32,
            canvas,
        }
    }

    fn render(&mut self, shape: &Shape) {
        // Transform from Camera Space (range from (-1, -1) to (1, 1)) to Image Space (range from (0, 0) to image size).
        let mut points = shape.points.iter().map(|p| {
            let p = Vec2::new(p.x, -p.y);
            p * self.scale / 2.0 + self.scale / 2.0
        });

        if let Some(first) = points.next() {
            let mut path = PathBuilder::new();
            path.move_to(first.x, first.y);

            // Grab second point in case we need to complete a polygon properly.
            let second = points.next();
            if let Some(second) = second {
                path.line_to(second.x, second.y);
            }

            for point in points {
                path.line_to(point.x, point.y);
            }

            // Fix ends of polygon
            if shape.is_polygon() {
                let second = second.unwrap();

                path.line_to(second.x, second.y);
            }

            let path = path.finish().unwrap();

            if let Some(stroke) = shape.stroke {
                let mut paint = Paint::default();
                paint.set_color(stroke.color.into());
                paint.anti_alias = self.antialias;

                self.canvas.stroke_path(
                    &path,
                    &paint,
                    &tiny_skia::Stroke {
                        width: stroke.width * self.scale,
                        ..Default::default()
                    },
                    Transform::identity(),
                    None,
                );
            }

            if let Some(fill) = shape.fill {
                let mut paint = Paint::default();
                paint.set_color(fill.into());
                paint.anti_alias = self.antialias;

                self.canvas.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    fn finalize(self) -> Self::Output {
        RgbaImage::from_raw(
            self.canvas.width(),
            self.canvas.height(),
            self.canvas.take(),
        )
        .unwrap()
    }
}
