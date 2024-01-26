use glam::Vec2;

use crate::{Color, LineEnd, Renderer, Shape};
use std::fmt::Write;

/// A renderer for Scalable Vector Graphics.
///
/// Unless a shape approximates a circle, it will be drawn as either a polygon or a polyline.
/// If it does approximate a circle and meets [circle_vertex_threshold](SvgRenderer), it will be drawn as a circle.
#[derive(Clone)]
pub struct SvgRenderer {
    scale: f32,
    center_offset: Vec2,
    ints_only: bool,
    circle_vertex_threshold: usize,
    document: String,
}

impl SvgRenderer {
    /// Creates a new [SvgRenderer]
    ///
    /// `preserve_height` allows you to decide which axis to preserve.
    /// If `true`, then the rendered image will map `-1..=1` in the y axis in camera space to `size.y..=0`.
    /// If `false` then the rendered image will be mapped for the x axis.
    pub fn new(
        size: Vec2,
        background: Option<Color>,
        ints_only: bool,
        preserve_height: bool,
        circle_vertex_threshold: usize,
    ) -> Self {
        let mut document = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
            size.x, size.y
        );

        if let Some(background) = background {
            write!(
                document,
                "<rect fill=\"{}\" width=\"{}\" height=\"{}\"/>",
                background.as_hex(false),
                size.x,
                size.y
            )
            .unwrap();
        }

        let (scale, center_offset) = if preserve_height {
            let scale = size.y / 2.0;
            (scale, Vec2::new(size.x / 2.0 / scale, 1.0))
        } else {
            let scale = size.x / 2.0;
            (scale, Vec2::new(1.0, size.y / 2.0 / scale))
        };

        let circle_vertex_threshold = if circle_vertex_threshold < 3 {
            3
        } else {
            circle_vertex_threshold
        };

        Self {
            scale,
            center_offset,
            ints_only,
            circle_vertex_threshold,
            document,
        }
    }
}

impl Renderer for SvgRenderer {
    type Output = String;

    fn render(&mut self, shape: &Shape) {
        if !shape.is_drawable() {
            return;
        }

        // Check if shape approximates a circle, if so, render it as such.
        let is_circle = if shape.points.len() >= self.circle_vertex_threshold && shape.is_polygon()
        {
            let center = shape.points.iter().sum::<Vec2>() / shape.points.len() as f32;
            let d = center.distance(shape.points[0]);

            let mut is_circle = Some((
                (Vec2::new(center.x, -center.y) + self.center_offset) * self.scale,
                d * self.scale,
            ));
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
                stroke.width * self.scale
            )
            .unwrap();

            if stroke.color.a() != 1.0 {
                write!(self.document, "stroke-opacity:{};", stroke.color.a()).unwrap();
            }

            match stroke.line_end {
                LineEnd::Butt => write!(self.document, "stroke-linecap:butt;").unwrap(),
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
