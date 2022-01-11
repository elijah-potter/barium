use glam::{UVec2, Vec2};
use image::RgbaImage;
use tiny_skia::{FillRule, LineCap, Paint, PathBuilder, Pixmap, Transform};

use crate::canvas::Shape;
use crate::{Color, LineEnd, Renderer};

#[derive(Default, Clone, Copy)]
pub struct SkiaRendererSettings {
    /// Size of the output image.
    pub size: UVec2,
    /// An optional background color.
    pub background: Option<Color>,
    /// Use antialiasing
    pub antialias: bool,
    /// Will make sure to include everything vertically when mapping from Camera Space to the image. Otherwise will do so horizontally.
    pub preserve_height: bool,
}

pub struct SkiaRenderer {
    antialias: bool,
    scale: f32,
    center_offset: Vec2,
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

        let (scale, center_offset) = if settings.preserve_height {
            let scale = settings.size.y as f32 / 2.0;
            (scale, Vec2::new(settings.size.x as f32 / 2.0 / scale, 1.0))
        } else {
            let scale = settings.size.x as f32 / 2.0;
            (scale, Vec2::new(1.0, settings.size.y as f32 / 2.0 / scale))
        };

        Self {
            antialias: settings.antialias,
            scale,
            center_offset,
            canvas,
        }
    }

    fn render(&mut self, shape: &Shape) {
        // Transform from Camera Space (range from (-1, -1) to (1, 1)) to Image Space (range from (0, 0) to image size).
        let mut points = shape.points.iter().map(|p| {
            let p = Vec2::new(p.x, -p.y) + self.center_offset;
            p * self.scale
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
                        line_cap: match stroke.line_end {
                            LineEnd::Butt => LineCap::Butt,
                            LineEnd::Round => LineCap::Round,
                        },
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
