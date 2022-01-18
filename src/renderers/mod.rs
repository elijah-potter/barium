#[cfg(feature = "tiny_skia_renderer")]
mod skia_renderer;
#[cfg(feature = "speedy2d_renderer")]
mod speedy2d_renderer;
#[cfg(feature = "svg_renderer")]
mod svg_renderer;

#[cfg(feature = "svg_renderer")]
pub use svg_renderer::{SvgRenderer, SvgRendererSettings};

#[cfg(feature = "tiny_skia_renderer")]
pub use skia_renderer::{SkiaRenderer, SkiaRendererSettings};

#[cfg(feature = "speedy2d_renderer")]
pub use speedy2d_renderer::{Speedy2dRenderer, Speedy2dRendererSettings};
