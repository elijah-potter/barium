mod svg_renderer;
mod skia_renderer;

#[cfg(feature = "svg_renderer")]
pub use svg_renderer::{SvgRenderer, SvgRendererSettings};

#[cfg(feature = "tiny_skia_renderer")]
pub use skia_renderer::{SkiaRenderer, SkiaRendererSettings};