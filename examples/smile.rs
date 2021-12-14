extern crate denim;

use denim::{
    renderers::{SkiaRenderer, SkiaRendererSettings},
    Canvas, Color, Stroke, UVec2, Vec2,
};

fn main() {
    // Create a canvas, centered on (0, 0). The view ranges from (-1.0, -1.0) to (1.0, 1.0).
    let mut canvas = Canvas::new(Vec2::new(2.0, 2.0));

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
        }),
        None,
    );

    canvas.render::<SkiaRenderer>(SkiaRendererSettings{
        size: UVec2::new(1000, 1000),
        background: Some(background_color),
        antialias: true,
    }).save("img.png").unwrap();
}
