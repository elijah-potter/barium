use glam::Vec2;
use std::fmt::Write;

use crate::{
    canvas::{CanvasElement, CanvasElementVariant},
    color::Color,
    renderer::Renderer,
};

pub struct SvgRendererSettings {
    pub size: Vec2,
    pub background_color: Option<Color>,
}

impl Default for SvgRendererSettings {
    fn default() -> Self {
        Self {
            size: Vec2::new(1000.0, 1000.0),
            background_color: Default::default(),
        }
    }
}

pub struct SvgRenderer {
    document: String,
    blur_values: Vec<f32>,
}

impl Renderer for SvgRenderer {
    type Settings = SvgRendererSettings;
    type Output = String;

    fn new(settings: Self::Settings) -> Self {
        let mut document = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">",
            settings.size.x, settings.size.y
        );

        if let Some(background) = settings.background_color {
            write!(
                document,
                "<rect fill=\"{}\" width=\"{}\" height=\"{}\"/>",
                background.as_hex(false),
                settings.size.x,
                settings.size.y
            )
            .unwrap();
        }

        Self {
            document,
            blur_values: Vec::new(),
        }
    }

    fn render(&mut self, element: &CanvasElement) {
        match &element.variant {
            CanvasElementVariant::Blank => return,
            CanvasElementVariant::PolyLine { points, stroke } => {
                write!(self.document, "<polyline points=\"").unwrap();

                for point in points {
                    write!(self.document, "{},{} ", point.x, point.y).unwrap();
                }

                write!(
                    self.document,
                    "\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\" stroke-linecap=\"round\" ",
                    stroke.color.as_hex(false),
                    stroke.width
                )
                .unwrap();

                if stroke.color.a() < 1.0 {
                    write!(self.document, "stroke-opacity=\"{}\" ", stroke.color.a()).unwrap();
                }

                if let Some(blur_std_dev) = element.blur_std_dev {
                    write!(self.document, " filter=\"url(#f{})\"", blur_std_dev).unwrap();
                }

                write!(self.document, "/>").unwrap();
            }
            CanvasElementVariant::Circle {
                center,
                radius,
                fill,
                stroke,
            } => {
                write!(
                    self.document,
                    "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                    center.x, center.y, radius
                )
                .unwrap();

                if let Some(fill) = fill {
                    write!(self.document, " fill=\"{}\"", fill.as_hex(false)).unwrap();

                    if fill.a() < 1.0 {
                        write!(self.document, " fill-opacity=\"{}\"", fill.a()).unwrap();
                    }
                } else {
                    write!(self.document, " fill-opacity=\"0\"").unwrap();
                }

                if let Some(stroke) = stroke {
                    write!(
                        self.document,
                        " stroke=\"{}\" stroke-width=\"{}\"",
                        stroke.color.as_hex(false),
                        stroke.width
                    )
                    .unwrap();

                    if stroke.color.a() < 1.0 {
                        write!(self.document, " stroke-opacity=\"{}\"", stroke.color.a()).unwrap();
                    }
                }

                if let Some(blur_std_dev) = element.blur_std_dev {
                    write!(self.document, " filter=\"url(#f{})\"", blur_std_dev).unwrap();
                }

                write!(self.document, "/>").unwrap()
            }
            CanvasElementVariant::Polygon {
                points,
                fill,
                stroke,
            } => {
                write!(self.document, "<polygon points=\"",).unwrap();

                for point in points {
                    write!(self.document, "{},{} ", point.x, point.y);
                }

                write!(self.document, "\"").unwrap();

                if let Some(fill) = fill {
                    write!(self.document, " fill=\"{}\"", fill.as_hex(false)).unwrap();

                    if fill.a() < 1.0 {
                        write!(self.document, " fill-opacity=\"{}\"", fill.a()).unwrap();
                    }
                } else {
                    write!(self.document, " fill-opacity=\"0\"").unwrap();
                }

                if let Some(stroke) = stroke {
                    write!(
                        self.document,
                        " stroke=\"{}\" stroke-width=\"{}\"",
                        stroke.color.as_hex(false),
                        stroke.width
                    )
                    .unwrap();

                    if stroke.color.a() < 1.0 {
                        write!(self.document, " stroke-opacity=\"{}\"", stroke.color.a()).unwrap();
                    }
                }

                if let Some(blur_std_dev) = element.blur_std_dev {
                    write!(self.document, " filter=\"url(#f{})\"", blur_std_dev).unwrap();
                }

                write!(self.document, "/>").unwrap()
            }
            CanvasElementVariant::Cluster { children } => {
                if let Some(blur_std_dev) = element.blur_std_dev {
                    write!(self.document, "<g filter=\"url(#f{})\">", blur_std_dev).unwrap();
                }

                for child in children {
                    self.render(child);
                }

                if element.blur_std_dev.is_some() {
                    write!(self.document, "</g>").unwrap();
                }
            }
        }

        if let Some(blur_std_dev) = element.blur_std_dev {
            if !self.blur_values.contains(&blur_std_dev) {
                self.blur_values.push(blur_std_dev);
            }
        }
    }

    fn finalize(mut self) -> Self::Output {
        write!(self.document, "<defs>");

        for blur_value in self.blur_values {
            write!(self.document, "<filter id=\"f{}\"><feGaussianBlur in=\"SourceGraphic\" stdDeviation=\"{}\" /></filter>", blur_value, blur_value).unwrap();
        }

        write!(self.document, "</defs>");

        write!(self.document, "</svg>").unwrap();

        self.document
    }
}
