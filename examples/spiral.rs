use std::f32::consts::PI;

use denim::{
    renderers::{SkiaRenderer, SkiaRendererSettings},
    Canvas, Color, Stroke, UVec2, Vec2,
};

fn main() {
    let mut canvas = Canvas::new();

    // Create a single spiral
    let mut points = Vec::with_capacity(1000);
    for n in 0..1000 {
        points.push(Vec2::new(
            n as f32 / 500.0 * (2.0 * PI * n as f32 / 200.0).cos(),
            n as f32 / 500.0 * (2.0 * PI * n as f32 / 200.0).sin(),
        ))
    }

    // Draw the spiral multiple times, the camera before each one.
    for i in 0..8 {
        canvas.draw_shape(
            points.clone(),
            Some(Stroke {
                color: Color::white() * (i as f32 / 8.0),
                width: 0.01,
                line_end: denim::LineEnd::Round,
            }),
            None,
        );

        canvas.rotate_camera(PI / 4.0);
    }

    // Render the canvas to a PNG
    canvas
        .render::<SkiaRenderer>(SkiaRendererSettings {
            size: UVec2::new(1000, 1000),
            background: Some(Color::black()),
            antialias: true,
            preserve_height: false,
        })
        .save("spiral.png").unwrap();
}
