use glam::Vec2;

use crate::{Color, Renderer, Shape};
use std::fmt::Write;

#[derive(Default, Clone, Copy)]
pub struct SvgRendererSettings {
    /// Size of the SVG. Shapes outside this boundry will still be included.
    pub size: Vec2,
    /// An optional background color.
    pub background: Option<Color>,
    /// Whether or not to include floating point numbers.
    /// This can dramatically reduce file size.
    pub ints_only: bool,
    /// Will make sure to include everything vertically when mapping from Camera Space to the image. Otherwise will do so horizontally.
    pub preserve_height: bool
}

pub struct SvgRenderer {
    scale: f32,
    ints_only: bool,
    document: String,
}

impl Renderer for SvgRenderer {
    type Settings = SvgRendererSettings;

    type Output = String;

    fn new(settings: Self::Settings) -> Self {
        let mut document = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
            settings.size.x, settings.size.y
        );

        if let Some(background) = settings.background {
            write!(
                document,
                "<rect fill=\"{}\" width=\"{}\" height=\"{}\"/>",
                background.as_hex(false),
                settings.size.x,
                settings.size.y
            )
            .unwrap();
        }

        let scale = if settings.preserve_height{
            settings.size.y as f32
        }else{
            settings.size.x as f32
        };

        Self {
            scale,
            ints_only: settings.ints_only,
            document,
        }
    }

    fn render(&mut self, shape: &Shape) {
        if shape.points.len() > 3 && shape.points[0] == shape.points[shape.points.len() - 1] {
            write!(self.document, "<polygon points=\"").unwrap();
        } else {
            write!(self.document, "<polyline points=\"").unwrap();
        }

        for point in shape.points.iter().map(|p| {
            // Transform from Camera Space (range from (-1, -1) to (1, 1)) to Image Space (range from (0, 0) to image size).
            let p = Vec2::new(p.x, -p.y);
            p * self.scale / 2.0 + self.scale / 2.0
        }) {
            if self.ints_only {
                write!(self.document, "{},{} ", point.x.round(), point.y.round()).unwrap();
            } else {
                write!(self.document, "{},{} ", point.x, point.y).unwrap();
            }
        }

        write!(self.document, "\" style=\"").unwrap();

        if let Some(stroke) = shape.stroke {
            write!(
                self.document,
                "stroke:{};stroke-width:{};",
                stroke.color.as_hex(false),
                stroke.width * self.scale / 2.0
            )
            .unwrap();

            if stroke.color.a() != 1.0 {
                write!(self.document, "stroke-opacity:{};", stroke.color.a()).unwrap();
            }
        }

        if let Some(fill) = shape.fill {
            write!(self.document, "fill:{};", fill.as_hex(false)).unwrap();

            if fill.a() != 1.0 {
                write!(self.document, "fill-opacity:{};", fill.a()).unwrap();
            }
        } else {
            write!(self.document, "fill:none;").unwrap();
        }

        write!(self.document, "\"/>").unwrap();
    }

    fn finalize(mut self) -> Self::Output {
        write!(self.document, "</svg>").unwrap();

        self.document
    }
}
