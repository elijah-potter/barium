use std::f32::consts::PI;

use crate::{color::Color, glam_ext::Vec2Ext};
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

impl Stroke {
    pub fn new(color: Color, width: f32) -> Self {
        Self { color, width }
    }
}

pub trait Renderer {
    type Settings;
    type Output;

    fn new(settings: Self::Settings) -> Self;
    fn resolution_scale(&self) -> f32;
    fn render(&mut self, shape: &Shape);
    fn finalize(self) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Canvas {
    translate: Vec2,
    rotate: f32,
    zoom: f32,
    size: Vec2,
    shapes: Vec<Shape>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            translate: Default::default(),
            rotate: Default::default(),
            zoom: 1.0,
            size: Vec2::ONE,
            shapes: Default::default(),
        }
    }
}

impl Canvas {
    pub fn new(size: Vec2) -> Self {
        let mut new = Self {
            size,
            ..Default::default()
        };

        new.move_view_to(Vec2::ZERO);
        new
    }

    pub fn render<R: Renderer>(&self, settings: R::Settings) -> R::Output {
        let mut renderer = R::new(settings);

        for shape in &self.shapes {
            let mut transformed_shape = shape.clone();

            for point in transformed_shape.points.iter_mut() {
                *point = self.to_view_space(*point, renderer.resolution_scale());
            }

            renderer.render(&transformed_shape);
        }

        renderer.finalize()
    }

    /// Rotate the view.
    pub fn rotate_view(&mut self, radians: f32) {
        self.rotate += radians;
    }

    // Centers the view on a point.
    pub fn move_view_to(&mut self, location: Vec2) {
        self.translate = self.size * self.zoom * -0.5 + location;
    }

    /// Draws a shape onto the canvas, projected from view.
    pub fn draw_shape<C: Into<Vec<Vec2>>>(
        &mut self,
        points: C,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let mut points: Vec<Vec2> = points.into();

        for point in points.iter_mut() {
            *point = self.to_world_space(*point);
        }

        self.shapes.push(Shape {
            points,
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
    pub fn draw_regular_polygon(
        &mut self,
        center: Vec2,
        sides: usize,
        radius: f32,
        rotation: f32,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        if sides < 3 {
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

    /// Transform any given point from world space to view space.
    /// Allows to scale to a given resolution width.
    pub fn to_view_space(&self, mut point: Vec2, resolution_scale: f32) -> Vec2 {
        let view_center = self.translate + self.size * self.zoom;

        point = point.rotate(-self.rotate);
        point = point / self.zoom + view_center;
        point * resolution_scale / 2.0
    }

    /// Transform any given point from view space to world space.
    pub fn to_world_space(&self, mut point: Vec2) -> Vec2 {
        let view_center = self.translate + self.size * self.zoom * 0.5;

        point = (point - view_center) * self.zoom;
        point = point.rotate(self.rotate);

        point.y *= -1.0;
        point
    }
}
