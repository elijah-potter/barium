#[cfg(feature = "tiny_skia_renderer")]
mod skia_renderer;
#[cfg(feature = "svg_renderer")]
mod svg_renderer;

#[cfg(feature = "svg_renderer")]
pub use svg_renderer::{SvgRenderer};

#[cfg(feature = "tiny_skia_renderer")]
pub use skia_renderer::{SkiaRenderer};
