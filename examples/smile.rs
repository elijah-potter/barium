extern crate barium;

use barium::{
    renderers::{SkiaRenderer, SvgRenderer},
    Canvas, Color, LineEnd, Stroke, UVec2, Vec2,
};

fn main() -> anyhow::Result<()> {
    // Create a canvas, centered on (0, 0). The camera ranges from (-1.0, -1.0) to (1.0, 1.0).
    let mut canvas = Canvas::new(1000);

    // Draw face
    canvas.draw_circle(
        Vec2::ZERO,
        1.0,
        None,
        Some(Color::from_hex("#fecb00").unwrap()),
    );

    // Draw eyes
    canvas.draw_line(
        Vec2::new(-0.5, 0.25),
        Vec2::new(-0.5, 0.0),
        Some(Stroke {
            color: Color::black(),
            width: 0.2,
            line_end: LineEnd::Round,
        }),
        None,
    );

    canvas.draw_line(
        Vec2::new(0.5, 0.25),
        Vec2::new(0.5, 0.0),
        Some(Stroke {
            color: Color::black(),
            width: 0.2,
            line_end: LineEnd::Round,
        }),
        None,
    );

    // Draw mouth
    canvas.draw_quadratic_bezier(
        Vec2::new(-0.5, -0.3),
        Vec2::Y * -0.5,
        Vec2::new(0.5, -0.3),
        None,
        Some(Color::black()),
    );

    // Save to png
    let png = canvas.render(SkiaRenderer::new(
        UVec2::splat(1000),
        Some(Color::black()),
        false,
        true,
    ));
    png.save("smile.png")?;

    // Save to svg
    let svg = canvas.render(SvgRenderer::new(
        Vec2::splat(1000.0),
        Some(Color::black()),
        false,
        false,
        32,
    ));

    std::fs::write("smile.svg", svg)?;

    Ok(())
}
