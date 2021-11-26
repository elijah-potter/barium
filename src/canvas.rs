use std::f32::consts::PI;

use glam::Vec2;

use crate::{color::Color, renderer::Renderer};

#[derive(Default, Clone, Copy, Debug)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
}

#[derive(Clone, Debug)]
pub enum CanvasElementVariant {
    /// Nothing.
    Blank,
    /// A line made up of connected points.
    PolyLine { points: Vec<Vec2>, stroke: Stroke },
    /// A circle with an optional filled color with an optional outline.
    Circle {
        center: Vec2,
        radius: f32,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },
    /// A polygon with an optional filled color with an optional outline.
    Polygon {
        points: Vec<Vec2>,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },
    /// Several CanvasElements clustered together.
    Cluster { children: Vec<CanvasElement> },
}

impl Default for CanvasElementVariant {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Default, Clone, Debug)]
/// An element that can be drawn to the canvas.
pub struct CanvasElement {
    /// The optional standard deviation for a Gaussian Blur. If None, no blur is applied.
    pub blur_std_dev: Option<f32>,
    /// The type of element being drawn.
    pub variant: CanvasElementVariant,
}

/// An in-memory canvas.
#[derive(Default, Debug)]
pub struct Canvas {
    /// The visual elements in the canvas.
    elements: Vec<CanvasElement>,
}

impl Canvas {
    pub fn draw(&mut self, element: CanvasElement) {
        self.elements.push(element);
    }

    pub fn render<T: Renderer>(&self, settings: T::Settings) -> T::Output {
        let mut renderer = T::new(settings);

        for element in &self.elements {
            renderer.render(element);
        }

        renderer.finalize()
    }
}

/// Generates the points needed for a regular polygon.
/// Rotation is in radians.
pub fn regular_polygon_points(center: Vec2, sides: usize, radius: f32, rotation: f32) -> Vec<Vec2> {
    let mut points = Vec::new();

    for n in 0..sides {
        points.push(Vec2::new(
            radius * (2.0 * PI * n as f32 / sides as f32 + rotation).cos()
                + center.x,
            radius * (2.0 * PI * n as f32 / sides as f32 + rotation).sin()
                + center.y,
        ))
    }

    points
}
