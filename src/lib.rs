mod canvas;
mod color;
mod renderer;
pub mod renderers;

pub use canvas::{regular_polygon_points, Canvas, CanvasElement, CanvasElementVariant, Stroke};
pub use color::Color;
pub use glam::{UVec2, Vec2};
pub use renderer::Renderer;
