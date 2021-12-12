use std::f32::consts::PI;

use crate::color::Color;
use glam::Vec2;

#[derive(Default, Debug, Clone, PartialEq)]
/// A polygonal shape with a stroke and fill.
pub struct Shape {
    /// Points that make up the shape.
    /// If you want the outline of the shape to be complete, the start and end points must be the same.
    pub points: Vec<Vec2>,
    /// The stroke along the points.
    pub stroke: Option<Stroke>,
    /// The area filled inside the points.
    pub fill: Option<Color>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
}

impl Stroke{
    pub fn new(color: Color, width: f32) -> Self{
        Self{
            color,
            width,
        }
    }
}

pub trait Renderer {
    type Settings;
    type Output;

    fn new(settings: Self::Settings) -> Self;
    fn render(&mut self, shape: &Shape);
    fn finalize(self) -> Self::Output;
}

#[derive(Default, Debug)]
pub struct Canvas {
    shapes: Vec<Shape>,
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render<R: Renderer>(&self, settings: R::Settings) -> R::Output {
        let mut renderer = R::new(settings);

        for shape in &self.shapes {
            renderer.render(shape);
        }

        renderer.finalize()
    }

    pub fn draw_shape<C: Into<Vec<Vec2>>>(
        &mut self,
        points: C,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.shapes.push(Shape {
            points: points.into(),
            stroke,
            fill,
        })
    }

    pub fn draw_rect(
        &mut self,
        top_left: Vec2,
        bottom_right: Vec2,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(
            vec![
                top_left,
                Vec2::new(bottom_right.x, top_left.y),
                bottom_right,
                Vec2::new(top_left.x, bottom_right.y),
                top_left,
            ],
            stroke,
            fill,
        )
    }

    /// Draws a regular polygon.
    /// 
    /// Rotation is in radians.
    /// Sides must be >= 3.
    pub fn draw_regular_polygon(&mut self, center: Vec2, sides: usize, radius: f32, rotation: f32, stroke: Option<Stroke>, fill: Option<Color>){
        if sides < 3{
            panic!("There must be at least 3 sides in a regular polygon.")
        }

        let mut points = Vec::with_capacity(sides + 1);

        for n in 0..sides {
            points.push(Vec2::new(
                radius * (2.0 * PI * n as f32 / sides as f32 + rotation).cos() + center.x,
                radius * (2.0 * PI * n as f32 / sides as f32 + rotation).sin() + center.y,
            ))
        }

        points.push(points[0]);
    
        self.draw_shape(points, stroke, fill)
    }
}
