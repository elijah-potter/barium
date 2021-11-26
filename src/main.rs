use std::fs::write;

use canvas::{Canvas, CanvasElementVariant};
use glam::Vec2;

use crate::{
    canvas::{CanvasElement, Stroke},
    color::Color,
    renderers::{SvgRenderer, SvgRendererSettings},
};

mod canvas;
mod color;
mod renderer;
mod renderers;

fn main() {
    let mut canvas = Canvas::default();

    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Circle {
            center: Vec2::ONE * 300.0,
            radius: 100.0,
            fill: Some(Color::new(1.0, 0.0, 0.0, 1.0)),
            stroke: None,
        },
        blur_std_dev: Some(1.0),
        ..Default::default()
    });

    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Cluster {
            children: vec![
                CanvasElement {
                    variant: CanvasElementVariant::Polygon {
                        points: vec![Vec2::ZERO, Vec2::ONE * 100.0, Vec2::new(100.0, 0.0)],
                        fill: Some(Color::white()),
                        stroke: Some(Stroke {
                            color: Color::new(1.0, 0.0, 1.0, 1.0),
                            width: 10.0,
                        }),
                    },
                    ..Default::default()
                },
                CanvasElement {
                    variant: CanvasElementVariant::PolyLine {
                        points: vec![Vec2::ZERO, Vec2::ONE * 100.0],
                        stroke: Stroke {
                            color: Color::white(),
                            width: 2.0,
                        },
                    },
                    ..Default::default()
                },
                CanvasElement {
                    variant: CanvasElementVariant::Circle {
                        center: Vec2::ONE * 200.0,
                        radius: 50.0,
                        fill: Some(Color::white()),
                        stroke: None,
                    },
                    ..Default::default()
                },
            ],
        },
        blur_std_dev: Some(5.0)
    });

    write(
        "test.svg",
        canvas.render::<SvgRenderer>(SvgRendererSettings {
            background_color: Some(Color::black()),
            ..Default::default()
        }),
    )
    .unwrap();
}
