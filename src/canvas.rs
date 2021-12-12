use std::f32::consts::PI;

use glam::Vec2;

use crate::{renderer::Renderer, Color, Transform};

#[derive(Default, Clone, Copy, Debug)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
}

#[derive(Clone, Debug)]
/// A variant of a CanvasElement
pub enum CanvasElementVariant {
    /// Draws nothing.
    ///
    /// This allows CanvasElementVariant to implement Default.
    Blank,
    /// A line made up of connected points.
    PolyLine { points: Vec<Vec2>, stroke: Stroke },
    /// A circle with an optional filled color and an optional outline.
    Ellipse {
        transform: Transform,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },
    /// A polygon with an optional filled color and an optional outline.
    Polygon {
        points: Vec<Vec2>,
        fill: Option<Color>,
        stroke: Option<Stroke>,
    },
    /// Several CanvasElements clustered together.
    Cluster { children: Vec<CanvasElement> },
}

#[derive(Clone, Copy, Debug)]
pub enum CanvasElementPostEffect {
    /// A standard Gaussian Blur.
    GaussianBlur { std_dev: f32 },
    /// Move, scale or rotate object.
    Adjust { transform: Transform },
}

impl Default for CanvasElementVariant {
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Default, Clone, Debug)]
/// An element that can be drawn to the canvas.
pub struct CanvasElement {
    /// The type of element being drawn.
    pub variant: CanvasElementVariant,
    /// Post processing effects to be applied to the element.
    pub post_effects: Vec<CanvasElementPostEffect>,
}

/// An in-memory canvas.
#[derive(Default, Clone, Debug)]
pub struct Canvas {
    /// The visual elements in the canvas.
    elements: Vec<CanvasElement>,
}

impl Canvas {
    /// Create a new [Canvas]
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Create a canvas from an existing collection of [CanvasElement].
    pub fn from_raw<C: Into<Vec<CanvasElement>>>(collection: C) -> Self {
        Self {
            elements: collection.into(),
        }
    }

    /// Draws an element onto the canvas.
    pub fn draw(&mut self, element: CanvasElement) {
        self.elements.push(element);
    }

    /// Removes the last element, returning it if the operation was successful.
    pub fn undo(&mut self) -> Option<CanvasElement> {
        self.elements.pop()
    }

    /// Consumes the canvas and returns the inner [Vec<CanvasElement>].
    pub fn to_raw(self) -> Vec<CanvasElement> {
        self.elements
    }

    /// Render the canvas with a renderer of your choice.
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
            radius * (2.0 * PI * n as f32 / sides as f32 + rotation).cos() + center.x,
            radius * (2.0 * PI * n as f32 / sides as f32 + rotation).sin() + center.y,
        ))
    }

    points
}

/// Generates the points needed for a rectangle.
pub fn rect_polygon_points(top_left: Vec2, bottom_right: Vec2) -> Vec<Vec2> {
    vec![
        top_left,
        Vec2::new(bottom_right.x, top_left.y),
        bottom_right,
        Vec2::new(top_left.x, bottom_right.y),
    ]
}
