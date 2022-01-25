use glam::{UVec2, Vec2};
use image::RgbaImage;
use tiny_skia::{FillRule, LineCap, Paint, PathBuilder, Pixmap, Transform};

use crate::canvas::Shape;
use crate::{Color, LineEnd, Renderer};

/// Renderer that uses the [tiny_skia](https://github.com/RazrFalcon/tiny-skia) crate.
/// This is NOT actual Skia, but a Rust port.
#[derive(Clone)]
pub struct SkiaRenderer {
    antialias: bool,
    scale: f32,
    center_offset: Vec2,
    canvas: Pixmap,
}

impl SkiaRenderer {
    /// Create a new [SkiaRenderer].
    ///
    /// `preserve_height` allows you to decide which axis to preserve.
    /// If `true`, then the rendered image will map `-1..=1` in the y axis in camera space to `size.y..=0`.
    /// If `false` then the rendered image will be mapped for the x axis.
    pub fn new(
        size: UVec2,
        background: Option<Color>,
        antialias: bool,
        preserve_height: bool,
    ) -> Self {
        let mut canvas = Pixmap::new(size.x, size.y).unwrap();

        if let Some(background) = background {
            canvas.fill(background.into());
        }

        let (scale, center_offset) = if preserve_height {
            let scale = size.y as f32 / 2.0;
            (scale, Vec2::new(size.x as f32 / 2.0 / scale, 1.0))
        } else {
            let scale = size.x as f32 / 2.0;
            (scale, Vec2::new(1.0, size.y as f32 / 2.0 / scale))
        };

        Self {
            antialias,
            scale,
            center_offset,
            canvas,
        }
    }
}

impl Renderer for SkiaRenderer {
    type Output = RgbaImage;

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
