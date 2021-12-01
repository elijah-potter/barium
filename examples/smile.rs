extern crate denim;

use std::f32::consts::PI;

use denim::renderers::{SkiaRenderer, SkiaRendererSettings, ToRgbaImage};
use denim::{
    rect_polygon_points, regular_polygon_points, Canvas, CanvasElement, CanvasElementVariant,
    Color, Stroke, UVec2, Vec2,
};

fn main() {
    let mut canvas = Canvas::default();

    // Left eye
    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Ellipse {
            center: Vec2::new(300.0, 300.0),
            radius: Vec2::splat(100.0),
            fill: Some(Color::from_hex("#BF616A").unwrap()),
            stroke: None,
        },
        ..Default::default()
    });

    // Right eye
    canvas.draw(CanvasElement {
        variant: CanvasElementVariant::Ellipse {
            center: Vec2::new(1620.0, 300.0),
            radius: Vec2::splat(100.0),
            fill: Some(Color::from_hex("#BF616A").unwrap()),
            stroke: None,
        },
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
            background_color: Some(Color::from_hex("#2E3440").unwrap()),
        })
        .to_rgba_image()
        .save("smile.png")
        .unwrap();
}
