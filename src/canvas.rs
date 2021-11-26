use glam::Vec2;

use crate::{color::Color, renderer::Renderer};

#[derive(Default, Clone, Copy, Debug)]
pub struct Stroke{
    pub color: Color,
    pub width: f32
}

#[derive(Clone, Debug)]
pub enum CanvasElementVariant{
    /// Nothing.
    Blank,
    /// A line made up of connected points.
    PolyLine{
        points: Vec<Vec2>,
        stroke: Stroke
    },
    /// A circle with an optional filled color with an optional outline.
    Circle{
        center: Vec2,
        radius: f32,
        fill: Option<Color>,
        stroke: Option<Stroke>
    },
    /// A polygon with an optional filled color with an optional outline.
    Polygon{
        points: Vec<Vec2>,
        fill: Option<Color>,
        stroke: Option<Stroke>
    },
    /// Several CanvasElements clustered together.
    Cluster{
        children: Vec<CanvasElement>
    }
}

impl Default for CanvasElementVariant{
    fn default() -> Self {
        Self::Blank
    }
}

#[derive(Default, Clone, Debug)]
/// An element that can be drawn to the canvas.
pub struct CanvasElement{
    /// The optional standard deviation for a Gaussian Blur. If None, no blur is applied.
    pub blur_std_dev: Option<f32>,
    /// The type of element being drawn.
    pub variant: CanvasElementVariant
}

/// An in-memory canvas.
#[derive(Default, Debug)]
pub struct Canvas{
    /// The visual elements in the canvas. 
    elements: Vec<CanvasElement>
}

impl Canvas{
    pub fn draw(&mut self, element: CanvasElement){
        self.elements.push(element);
    }

    pub fn render<T: Renderer>(&self, settings: T::Settings) -> T::Output{
        let mut renderer = T::new(settings);

        for element in &self.elements{
            renderer.render(element);
        }

        renderer.finalize()
    }
}
