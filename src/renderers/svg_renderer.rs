use glam::Vec2;
use std::fmt::Write;

use crate::{canvas::{CanvasElement, CanvasElementPostEffect, CanvasElementVariant}, color::Color, renderer::Renderer};

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
    document_size: Vec2,
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
            document_size: settings.size,
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

                if !element.post_effects.is_empty(){
                    write!(self.document, "filter=\"").unwrap();

                    for effect in &element.post_effects{
                        match effect{
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            },
                        }
                    }

                    write!(self.document, "\"").unwrap();
                }

                write!(self.document, "/>").unwrap();
            }
            CanvasElementVariant::Ellipse {
                center,
                radius,
                fill,
                stroke,
            } => {
                if *radius == Vec2::splat(radius.x) {
                    write!(
                        self.document,
                        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"",
                        center.x, center.y, radius.x
                    )
                    .unwrap();
                }else{
                    write!(
                        self.document,
                        "<ellipse cx=\"{}\" cy=\"{}\" rx=\"{}\" ry=\"{}\"",
                        center.x, center.y, radius.x, radius.y
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

                if !element.post_effects.is_empty(){
                    write!(self.document, " filter=\"").unwrap();

                    for effect in &element.post_effects{
                        match effect{
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            },
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

                for point in points {
                    write!(self.document, "{},{} ", point.x, point.y).unwrap();
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

                if !element.post_effects.is_empty(){
                    write!(self.document, " filter=\"").unwrap();

                    for effect in &element.post_effects{
                        match effect{
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            },
                        }
                    }

                    write!(self.document, "\"").unwrap();
                }

                write!(self.document, "/>").unwrap()
            }
            CanvasElementVariant::Cluster { children } => {
                if !element.post_effects.is_empty(){
                    write!(self.document, "<g filter=\"").unwrap();

                    for effect in &element.post_effects{
                        match effect{
                            CanvasElementPostEffect::GaussianBlur { std_dev } => {
                                write!(self.document, "url(#f{})", std_dev).unwrap();
                            },
                        }
                    }

                    write!(self.document, "\">").unwrap();
                }

                for child in children {
                    self.render(child);
                }

                if !element.post_effects.is_empty() {
                    write!(self.document, "</g>").unwrap();
                }
            }
        }

        for effect in &element.post_effects{
            match effect{
                CanvasElementPostEffect::GaussianBlur { std_dev } => {
                    if !self.blur_values.contains(&std_dev) {
                        self.blur_values.push(*std_dev);
                    }
                },
            }
        }
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
