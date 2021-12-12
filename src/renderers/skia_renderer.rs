use glam::UVec2;
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
            canvas,
        }
    }

    fn render(&mut self, shape: &Shape) {
        if let Some(first) = shape.points.first() {
            let mut path = PathBuilder::new();
            path.move_to(first.x, first.y);
            for point in shape.points.iter().skip(1) {
                path.line_to(point.x, point.y);
            }

            // Fix ends if polygon
            if shape.points.len() > 3 && *first == shape.points[shape.points.len() - 1]{
                let step_two = shape.points[1];

                path.line_to(step_two.x, step_two.y);
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
                        width: stroke.width,
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
        ).unwrap()
    }
}
