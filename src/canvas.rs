use std::f32::consts::PI;

use crate::{color::Color, glam_ext::Vec2Ext};
use glam::Vec2;

#[derive(Debug, Clone, PartialEq)]
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

impl Shape {
    pub fn is_polygon(&self) -> bool {
        if self.points.len() < 3 {
            false
        } else {
            self.points[0] == self.points[self.points.len() - 1]
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    pub color: Color,
    pub width: f32,
    pub line_end: LineEnd,
}

impl Stroke {
    pub fn new(color: Color, width: f32, line_end: LineEnd) -> Self {
        Self { color, width, line_end }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineEnd {
    Butt,
    Round,
}

/// A renderer for [Canvas].
///
/// Anyone who wants to implement a renderer should reference either (SkiaRenderer)[crate::renderers::SkiaRenderer] or (SvgRenderer)[crate::renderers::SvgRenderer].
pub trait Renderer {
    /// Configuration for the renderer.
    type Settings;
    /// The intended format the renderer will output.
    type Output;

    /// Create and setup the renderer.
    fn new(settings: Self::Settings) -> Self;
    /// Render a shape. All coordinates in the shape will be in Camera Space.
    fn render(&mut self, shape: &Shape);
    /// Finalize the render.
    fn finalize(self) -> Self::Output;
}

#[derive(Debug, Clone)]
/// A canvas that can be used with many backends.
///
/// There are two 'spaces': `World Space` and `View Space`.
///
/// The camera starts centered on `(0.0, 0.0)` with a `zoom` of 1.0.
///
/// This means that, by default, `View Space` and `World Space` are equal. Once the camera has been changed, any drawing will be from the perspective of `View Space` onto `World Space`.
///
/// For example, a rectangle with corners at `(-1, -1)` and `(1, 1)` will be twice as large in World Space if it is drawn while the camera's `zoom` is at `0.5`.
pub struct Canvas {
    translate: Vec2,
    rotate: f32,
    zoom: f32,
    shapes: Vec<Shape>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            translate: Default::default(),
            rotate: Default::default(),
            zoom: 1.0,
            shapes: Default::default(),
        }
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render the canvas using a renderer of your choice.
    pub fn render<R: Renderer>(&self, settings: R::Settings) -> R::Output {
        let mut renderer = R::new(settings);

        for shape in &self.shapes {
            let mut transformed_shape = shape.clone();

            for point in transformed_shape.points.iter_mut() {
                *point = self.to_camera_space(*point);
            }

            if let Some(stroke) = &mut transformed_shape.stroke {
                stroke.width *= self.zoom;
            }

            renderer.render(&transformed_shape);
        }

        renderer.finalize()
    }

    pub fn to_raw(self) -> Vec<Shape> {
        self.shapes
    }

    pub fn as_raw(&self) -> &[Shape] {
        self.shapes.as_slice()
    }

    pub fn as_raw_mut(&mut self) -> &mut [Shape] {
        self.shapes.as_mut_slice()
    }

    /// Rotate the camera.
    pub fn rotate_camera(&mut self, radians: f32) {
        self.rotate += radians;
    }

    /// Rotate the camera to specific rotation.
    pub fn rotate_camera_to(&mut self, radians: f32) {
        self.rotate = radians;
    }

    /// Moves the camera by a certain amount. This is not effected by zoom.
    pub fn move_camera(&mut self, translation: Vec2) {
        self.translate += translation;
    }

    /// Centers the camera on a point. This is not effected by zoom.
    pub fn move_camera_to(&mut self, location: Vec2) {
        self.translate = location;
    }

    /// Zoom camera. This cannot be zero.
    pub fn zoom_camera(&mut self, zoom: f32) {
        assert!(zoom > 0.0);

        self.zoom *= zoom;
    }

    /// Zoom to a specific amount.
    pub fn zoom_camera_to(&mut self, zoom: f32) {
        assert!(zoom > 0.0);

        self.zoom = zoom;
    }

    /// Draws a shape onto the canvas, projected from the camera.
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

    /// Transform any given point from world space to camera space.
    /// Allows to scale to a given resolution width.
    pub fn to_camera_space(&self, point: Vec2) -> Vec2 {
        ((point - self.translate) * self.zoom).rotate(-self.rotate)
    }

    /// Transform any given point from camera space to world space.
    pub fn to_world_space(&self, point: Vec2) -> Vec2 {
        point.rotate(self.rotate) / self.zoom + self.translate
    }
}

#[cfg(test)]
mod tests {}
