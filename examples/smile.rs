extern crate denim;

use std::f32::consts::PI;

use denim::renderers::skia_renderer::{SkiaRenderer, SkiaRendererSettings, ToRgbaImage};
use denim::renderers::svg_renderer::{SvgRenderer, SvgRendererSettings};
use denim::{
    rect_polygon_points, regular_polygon_points, Canvas, CanvasElement, CanvasElementPostEffect,
    CanvasElementVariant, Color, Stroke, Transform, UVec2, Vec2,
};

fn main() {
    let mut canvas = Canvas::default();

    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Cluster {
            children: vec![
                /// Left eye    
                CanvasElement {
                    variant: CanvasElementVariant::Ellipse {
                        transform: Transform {
                            translate: Vec2::new(300.0, 300.0),
                            rotation: PI / 4.0,
                            scale: Vec2::new(100.0, 50.0),
                        },
                        fill: Some(Color::from_hex("#BF616A").unwrap()),
                        stroke: None,
                    },
                    ..Default::default()
                },
                /// Right eye
                CanvasElement {
                    variant: CanvasElementVariant::Ellipse {
                        transform: Transform {
                            translate: Vec2::new(1620.0, 300.0),
                            rotation: 0.0,
                            scale: Vec2::new(100.0, 50.0),
                        },
                        fill: Some(Color::from_hex("#BF616A").unwrap()),
                        stroke: None,
                    },
                    ..Default::default()
                },
            ],
        },
        post_effects: vec![CanvasElementPostEffect::Adjust {
            transform: Transform {
                rotation: PI / 2.0,
                ..Transform::one()
            },
        }],
        ..Default::default()
    });

    // Nose
    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::PolyLine {
            points: regular_polygon_points(Vec2::new(960.0, 540.0), 3, 100.0, PI * 1.5),
            stroke: Stroke {
                color: Color::from_hex("#D08770").unwrap(),
                width: 5.0,
            },
        },
        ..Default::default()
    });

    // Lips
    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Polygon {
            points: rect_polygon_points(Vec2::new(300.0, 750.0), Vec2::new(1620.0, 800.0)),
            fill: Some(Color::from_hex("#D08770").unwrap()),
            stroke: None,
        },
        ..Default::default()
    });

    // Render the canvas using the tiny-skia backend
    canvas
        .render::<SkiaRenderer>(SkiaRendererSettings {
            size: UVec2::new(1920, 1080),
            anti_alias: true,
            background_color: Some(Color::from_hex("#2E3440").unwrap()),
        })
        .to_rgba_image()
        .save("smile.png")
        .unwrap();

    // Render the canvas using the svg backend
    std::fs::write(
        "smile.svg",
        canvas.render::<SvgRenderer>(SvgRendererSettings {
            size: Vec2::new(1920.0, 1080.0),
            background_color: Some(Color::from_hex("#2E3440").unwrap()),
            reduced_precision: false,
        }),
    )
    .unwrap();
}
