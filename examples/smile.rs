extern crate denim;

use denim::{
    renderers::{SvgRenderer, SvgRendererSettings},
    Canvas, Color, Stroke, Vec2, LineEnd,
};

fn main() {
    // Create a canvas, centered on (0, 0). The camera ranges from (-1.0, -1.0) to (1.0, 1.0).
    let mut canvas = Canvas::new();

    let background_color = Color::from_hex("#2E3440").unwrap();

    // Draw square
    canvas.draw_rect(
        Vec2::ONE * -0.9,
        Vec2::ONE * 0.9,
        None,
        Some(Color::from_hex("#D08770").unwrap()),
    );

    // Draw eyes
    canvas.draw_regular_polygon(
        Vec2::new(-0.5, 0.5),
        32,
        0.1,
        0.0,
        None,
        Some(background_color),
    );

    canvas.draw_regular_polygon(
        Vec2::new(0.5, 0.5),
        32,
        0.1,
        0.0,
        None,
        Some(background_color),
    );

    // Draw mouth with a custom shape
    canvas.draw_shape(
        vec![
            Vec2::new(0.8, 0.0),
            Vec2::new(0.0, -0.8),
            Vec2::new(-0.8, 0.0),
        ],
        Some(Stroke {
            color: background_color,
            width: 10.0,
            line_end: LineEnd::Butt
        }),
        None,
    );

    let svg = canvas.render::<SvgRenderer>(SvgRendererSettings {
        size: Vec2::splat(1000.0),
        background: Some(Color::black()),
        ints_only: false,
        preserve_height: false
    });

    std::fs::write("smile.svg", svg).unwrap();
}