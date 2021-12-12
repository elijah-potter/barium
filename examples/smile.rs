extern crate denim;
use denim::{
    renderers::{SvgRenderer, SvgRendererSettings, SkiaRenderer, SkiaRendererSettings},
    Canvas, Color, Vec2, Stroke, UVec2,
};

fn main() {
    let mut canvas = Canvas::new();

    canvas.draw_regular_polygon(
        Vec2::ONE * 200.0,
        3,
        100.0,
        0.0,
        Some(Stroke::new(Color::black(), 10.0)),
        Some(Color::white()),
    );

    canvas.draw_rect(
        Vec2::new(200.0, 500.0),
        Vec2::new(1080.0, 525.0),
        None,
        Some(Color::from_hex("#D08770").unwrap()),
    );

    std::fs::write(
        "smile.svg",
        canvas.render::<SvgRenderer>(SvgRendererSettings {
            size: Vec2::new(1280.0, 720.0),
            background: Some(Color::from_hex("#2E3440").unwrap()),
            ints_only: false,
        }),
    )
    .unwrap();

    canvas.render::<SkiaRenderer>(SkiaRendererSettings{
        size: UVec2::new(1280, 720),
        background: Some(Color::from_hex("#2E3440").unwrap()),
        antialias: true,
    }).save("img.png");
}
