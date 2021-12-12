use glam::Vec2;
use std::fmt::Write;

use crate::{
    canvas::{CanvasElement, CanvasElementPostEffect, CanvasElementVariant},
    color::Color,
    renderer::Renderer,
    Transform,
};

#[derive(Clone)]
/// Settings for [SvgRenderer].
pub struct SvgRendererSettings {
    pub size: Vec2,
    pub background_color: Option<Color>,
    /// Uses integers instead of floats in certain situations.
    /// For complex files, this may reduce file size significantly.
    pub reduced_precision: bool,
}

/// A renderer to Scalable Vector Graphics.
pub struct SvgRenderer {
    reduced_precision: bool,
    document: String,
    blur_values: Vec<f32>,
}

impl SvgRenderer {
    fn render(&mut self, element: &CanvasElement, parent_transform: Transform) {
        // Add up all adjust effects.
        let mut full_transform = parent_transform;
        for effect in &element.post_effects {
            if let CanvasElementPostEffect::Adjust { transform } = effect {
                full_transform += *transform;
            }
        }

        match &element.variant {
            CanvasElementVariant::Blank => return,
            CanvasElementVariant::PolyLine { points, stroke } => {
                // Transform points
                let points = points.iter().map(|v| v);

                write!(self.document, "<polyline points=\"").unwrap();

                if self.reduced_precision {
                    for point in points {
                        write!(self.document, "{:.0},{:.0} ", point.x, point.y).unwrap();
                    }
                } else {
                    for point in points {
                        write!(self.document, "{},{} ", point.x, point.y).unwrap();
                    }
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

                if !element.post_effects.is_empty() {
                    write!(self.document, "filter=\"").unwrap();

                    for effect in &element.post_effects {
                        match effect {
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            }
                            CanvasElementPostEffect::Adjust { transform } => (),
                        }
                    }

                    write!(self.document, "\"").unwrap();
                }

                write!(self.document, "/>").unwrap();
            }
            CanvasElementVariant::Ellipse {
                transform,
                fill,
                stroke,
            } => {
                if transform.scale == Vec2::splat(transform.scale.x) {
                    write!(
                        self.document,
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                        transform.translate.x, transform.translate.y, transform.scale.x
                    )
                    .unwrap();
                } else {
                    write!(
                        self.document,
                        "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"",
                        transform.translate.x, transform.translate.y, transform.scale.x, transform.scale.y
                    )
                    .unwrap();
                }

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

                if !element.post_effects.is_empty() {
                    write!(self.document, " filter=\"").unwrap();

                    for effect in &element.post_effects {
                        match effect {
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            }
                            CanvasElementPostEffect::Adjust { transform } => (),
                        }
                    }

                    write!(self.document, "\"").unwrap();
                }

                write!(self.document, "/>").unwrap()
            }
            CanvasElementVariant::Polygon {
                points,
                fill,
                stroke,
            } => {
                write!(self.document, "<polygon points=\"",).unwrap();

                if self.reduced_precision {
                    for point in points {
                        write!(self.document, "{:.0},{:.0} ", point.x, point.y).unwrap();
                    }
                } else {
                    for point in points {
                        write!(self.document, "{},{} ", point.x, point.y).unwrap();
                    }
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

                if !element.post_effects.is_empty() {
                    write!(self.document, " filter=\"").unwrap();

                    for effect in &element.post_effects {
                        match effect {
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            }
                            CanvasElementPostEffect::Adjust { transform } => (),
                        }
                    }

                    write!(self.document, "\"").unwrap();
                }

                write!(self.document, "/>").unwrap()
            }
            CanvasElementVariant::Cluster { children } => {
                if !element.post_effects.is_empty() {
                    write!(self.document, "<g filter=\"").unwrap();

                    for effect in &element.post_effects {
                        match effect {
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            }
                            CanvasElementPostEffect::Adjust { transform } => (),
                        }
                    }

                    write!(self.document, "\">").unwrap();
                }

                for child in children {
                    self.render(child, full_transform);
                }

                if !element.post_effects.is_empty() {
                    write!(self.document, "</g>").unwrap();
                }
            }
        }

        for effect in &element.post_effects {
            match effect {
                CanvasElementPostEffect::GaussianBlur { std_dev } => {
                    if !self.blur_values.contains(std_dev) {
                        self.blur_values.push(*std_dev);
                    }
                }
                CanvasElementPostEffect::Adjust { transform } => (),
            }
        }
    }
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
            reduced_precision: settings.reduced_precision,
            document,
            blur_values: Vec::new(),
        }
    }

    fn render(&mut self, element: &CanvasElement) {
        self.render(element, Transform::one())
    }

    fn finalize(mut self) -> Self::Output {
        write!(self.document, "<defs>").unwrap();

        for blur_value in self.blur_values {
            write!(self.document, "<filter id=\"f{}\" x=\"-1000%\" y=\"-1000%\" width=\"2000%\" height=\"2000%\"><feGaussianBlur in=\"SourceGraphic\" stdDeviation=\"{}\" /></filter>", blur_value, blur_value).unwrap();
        }

        write!(self.document, "</defs>").unwrap();

        write!(self.document, "</svg>").unwrap();

        self.document
    }
}
