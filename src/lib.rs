pub mod renderers;
mod canvas;
mod color;
mod renderer;

pub use canvas::{Canvas, CanvasElement, CanvasElementVariant, Stroke, regular_polygon_points};
pub use color::Color;
pub use renderer::Renderer;
pub use glam::{UVec2, Vec2};