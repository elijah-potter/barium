pub mod renderers;
mod canvas;
mod color;
mod renderer;

pub use canvas::{Canvas, CanvasElement, CanvasElementVariant, Stroke};
pub use color::Color;
pub use renderer::Renderer;