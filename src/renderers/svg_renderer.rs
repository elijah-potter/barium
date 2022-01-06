use glam::Vec2;

use crate::{Color, LineEnd, Renderer, Shape};
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
    pub preserve_height: bool,
    /// The number of vertices a shape must have to qualify for circle estimation checking.
    pub circle_vertex_threshold: usize,
}

pub struct SvgRenderer {
    scale: f32,
    center_offset: Vec2,
    ints_only: bool,
    circle_vertex_threshold: usize,
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

        let (scale, center_offset) = if settings.preserve_height {
            let scale = settings.size.y as f32 / 2.0;
            (scale, Vec2::new(settings.size.x as f32 / 2.0 / scale, 1.0))
        } else {
            let scale = settings.size.x as f32 / 2.0;
            (scale, Vec2::new(1.0, settings.size.y as f32 / 2.0 / scale))
        };

        let circle_vertex_threshold = if settings.circle_vertex_threshold < 3 {
            3
        } else {
            settings.circle_vertex_threshold
        };

        Self {
            scale,
            center_offset,
            ints_only: settings.ints_only,
            circle_vertex_threshold,
            document,
        }
    }

    fn render(&mut self, shape: &Shape) {
        // Check if shape approximates a circle, if so, render it as such.
        let is_circle = if shape.points.len() >= self.circle_vertex_threshold && shape.is_polygon()
        {
            let center = shape.points.iter().sum::<Vec2>() / shape.points.len() as f32;
            let d = center.distance(shape.points[0]);

            let mut is_circle = Some((center * Vec2::new(1.0, -1.0) * self.scale + self.center_offset, d * self.scale));
            for point in &shape.points {
                if center.distance(*point) - d > d * 0.1 {
                    is_circle = None;
                    break;
                }
            }
            is_circle
        } else {
            None
        };

        if shape.points.len() > 3 && shape.is_polygon() {
            if let Some((circle_center, circle_radius)) = is_circle {
                write!(
                    self.document,
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}",
                    circle_center.x, circle_center.y, circle_radius
                )
                .unwrap();
            } else {
                write!(self.document, "<polygon points=\"").unwrap();
            }
        } else {
            write!(self.document, "<polyline points=\"").unwrap();
        }

        if is_circle.is_none() {
            for point in shape.points.iter().map(|p| {
                // Transform from Camera Space (range from (-1, -1) to (1, 1)) to Image Space (range from (0, 0) to image size).
                let p = Vec2::new(p.x, -p.y) + self.center_offset;
                p * self.scale
            }) {
                if self.ints_only {
                    write!(self.document, "{},{} ", point.x.round(), point.y.round()).unwrap();
                } else {
                    write!(self.document, "{},{} ", point.x, point.y).unwrap();
                }
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

            match stroke.line_end {
                LineEnd::Butt => todo!(),
                LineEnd::Round => write!(self.document, "stroke-linecap:round;").unwrap(),
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
