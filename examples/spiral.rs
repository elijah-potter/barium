use std::{f32::consts::PI};

use denim::{
    renderers::{SkiaRenderer},
    Canvas, Color, Stroke, UVec2, Vec2,
};

fn main() -> anyhow::Result<()>{
    let mut canvas = Canvas::new(1000);

    // Create a single spiral
    let mut points = Vec::with_capacity(2000);
    for n in 0..2000 {
        points.push(Vec2::new(
            n as f32 / 250.0 * (2.0 * PI * n as f32 / 200.0).cos(),
            n as f32 / 250.0 * (2.0 * PI * n as f32 / 200.0).sin(),
        ))
    }

    // Draw the spiral multiple times, moving the camera, before each one and adjusting the brightness.
    for i in 0..9 {
        canvas.draw_shape(
            points.clone(),
            Some(Stroke {
                color: (Color::white() * (i as f32 / 9.0)).with_a(1.0),
                width: 0.01,
                line_end: denim::LineEnd::Round,
            }),
            None,
        );

        canvas.rotate_camera(PI / 4.0);
    }

    canvas.render::<SkiaRenderer>(SkiaRenderer::new(
        UVec2::splat(1000),
        Some(Color::black()),
        true,
        false,
    )).save("spiral.png")?;

    Ok(())
}
