# Denim

`denim` is an easy to use canvas, intended to be portable and modular.

## Quick Start
Please note that currently, `denim` is not in a state where I feel comfortable enough to publish it on [crates.io], so the best way of getting it is via [GitHub](https://github.com/chilipepperhott/denim).

First add `denim` to your `Cargo.toml`, either manually:

```toml
[dependencies]
denim = { git = "https://github.com/chilipepperhott/denim" }
```

or using [cargo edit](https://github.com/killercup/cargo-edit):

```bash
cargo add --git https://github.com/chilipepperhott/denim denim
```

## Example

```rust
use std::f32::consts::PI;

use denim::renderers::{SkiaRenderer, SkiaRendererSettings, ToRgbaImage};
use denim::{
    regular_polygon_points, Canvas, CanvasElement, CanvasElementVariant,
    Color, Stroke, UVec2, Vec2,
};

// Create a new canvas
let mut canvas = Canvas::default();
// Draw smaller hexagon
canvas.draw(CanvasElement {
    variant: CanvasElementVariant::Polygon {
        points: regular_polygon_points(Vec2::new(500.0, 500.0), 6, 400.0, PI * 1.5),
        fill: None,
        stroke: Some(Stroke {
            color: Color::from_hex("#5E81AC").unwrap(),
            width: 5.0,
        }),
    },
    ..Default::default()
});

// Draw larger hexagon
canvas.draw(CanvasElement {
    variant: CanvasElementVariant::Polygon {
        points: regular_polygon_points(Vec2::new(500.0, 500.0), 6, 420.0, PI * 1.5),
        fill: None,
        stroke: Some(Stroke {
            color: Color::from_hex("#5E81AC").unwrap(),
            width: 5.0,
        }),
    },
    ..Default::default()
});

canvas.render::<SkiaRenderer>(SkiaRendererSettings {
    size: UVec2::new(1000, 1000),
    background_color: Some(Color::from_hex("#2e3440").unwrap()),
})
.to_rgba_image()
.save("smile.png")
.unwrap();
```

You can also look in the `examples` directory for more.

## About the name
`denim` is named ironically because Wikipedia lists `denim` as a material that canvases are almost never made from.