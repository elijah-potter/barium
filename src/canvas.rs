use std::f32::consts::PI;

use crate::color::Color;
use glam::{Mat2, Vec2};

/// A polygonal shape with a stroke and fill.
#[derive(Debug, Clone, PartialEq)]
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
    /// Checks if a shape is a polygon, otherwise it is a polyline.
    pub fn is_polygon(&self) -> bool {
        if self.points.len() < 3 {
            false
        } else {
            self.points[0] == self.points[self.points.len() - 1]
        }
    }
}

/// A structure that describes a line stroke.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stroke {
    /// Color of the stroke
    pub color: Color,
    /// Width of the stroke
    pub width: f32,
    /// How each end of the line terminates (a.k.a line cap).
    pub line_end: LineEnd,
}

impl Stroke {
    /// Create a new [Stroke]
    #[inline]
    pub fn new(color: Color, width: f32, line_end: LineEnd) -> Self {
        Self {
            color,
            width,
            line_end,
        }
    }
}

/// How to end [stroked](Stroke) line.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineEnd {
    /// Line continues past the final point and ends with a square.
    Butt,
    /// Line continues past the final point and ends with a circle.
    Round,
}

/// A renderer for [Canvas].
///
/// If you want to implement your own rendering backend,
/// reference either [SkiaRenderer](crate::renderers::SkiaRenderer) or [SvgRenderer](crate::renderers::SvgRenderer).
pub trait Renderer {
    /// Configuration for the renderer.
    type Settings;
    /// The intended format the renderer will output.
    type Output;
    /// Create and setup the renderer.
    fn new(settings: Self::Settings) -> Self;
    /// Render a shape. Provided coordinates will be in Camera Space (from the perspective of the camera).
    fn render(&mut self, shape: &Shape);
    /// Finalize the render.
    fn finalize(self) -> Self::Output;
}

/// A canvas that can be used with many backends.
///
/// There are two 'spaces': `World Space` and `View Space`.
///
/// The camera starts centered on `(0.0, 0.0)` with a `zoom` of 1.0.
///
/// This means that, by default, `View Space` and `World Space` are equal. Once the camera has been changed, any drawing will be from the perspective of `View Space` onto `World Space`.
///
/// For example, a rectangle with corners at `(-1, -1)` and `(1, 1)` will be twice as large in World Space if it is drawn while the camera's `zoom` is at `0.5`.
#[derive(Debug, Clone)]
pub struct Canvas {
    zoom: f32,
    translate: Vec2,
    to_camera_mat: Mat2,
    to_world_mat: Mat2,
    shapes: Vec<Shape>,
}

impl Default for Canvas {
    #[inline]
    fn default() -> Self {
        Self {
            zoom: 1.0,
            translate: Vec2::ZERO,
            to_camera_mat: Mat2::IDENTITY,
            to_world_mat: Mat2::IDENTITY,
            shapes: Vec::new(),
        }
    }
}

impl Canvas {
    /// Create a new [Canvas].
    #[inline]
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

    /// Returns a [Vec] of all the [Shapes](Shape) drawn on the canvas.
    pub fn to_raw(self) -> Vec<Shape> {
        self.shapes
    }

    /// Returns a slice of all the [Shapes](Shape) drawn on the canvas.
    pub fn as_raw(&self) -> &[Shape] {
        self.shapes.as_slice()
    }

    /// Returns a mutable slice of all the [Shapes](Shape) drawn on the canvas.
    pub fn as_raw_mut(&mut self) -> &mut [Shape] {
        self.shapes.as_mut_slice()
    }

    /// Rotate the camera.
    pub fn rotate_camera(&mut self, radians: f32) {
        let rotate_mat = Mat2::from_angle(radians);
        self.to_camera_mat = rotate_mat.mul_mat2(&self.to_camera_mat);
        self.to_world_mat = self.to_camera_mat.inverse();
    }

    /// Moves the camera by a certain amount. This is not effected by zoom.
    pub fn move_camera(&mut self, translation: Vec2) {
        self.translate += translation;
    }

    /// Zoom camera
    pub fn zoom_camera(&mut self, zoom: f32) {
        self.to_camera_mat *= zoom;
        self.to_world_mat = self.to_camera_mat.inverse();
        self.zoom *= zoom;
    }

    /// Clears the canvas
    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    /// Draws a shape onto the canvas, projected from the camera.
    pub fn draw_shape<C: Into<Vec<Vec2>>>(
        &mut self,
        points: C,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let mut points: Vec<Vec2> = points.into();

        if points.is_empty() {
            return;
        } else if points.len() == 1 {
            panic!("A shape must contain more than one point.");
        }

        let mut iter = points.iter_mut().peekable();

        while let Some(point) = iter.next() {
            if let Some(peeked) = iter.peek() {
                if *point == **peeked {
                    panic!("There cannot be sequential points that are the same.");
                }
            }

            *point = self.to_world_space(*point);
        }

        self.shapes.push(Shape {
            points,
            stroke,
            fill,
        })
    }

    /// Adds a shape directly onto the canvas, with no transformations.
    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    /// Draw a rectangle onto the canvas, projected from the camera.
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

    /// Draws a regular polygon onto the canvas, projected from the camera.
    ///
    /// Rotation is in radians.
    /// Will panic if `sides` < 3.
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

        // Connect first and last points to complete polygon.
        points.push(points[0]);

        self.draw_shape(points, stroke, fill)
    }

    /// Draw a triangle onto the canvas, projected from the camera.
    pub fn draw_triangle(
        &mut self,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(vec![p0, p1, p2], stroke, fill);
    }

    /// Draw a quad onto the canvas, projected from the camera.
    pub fn draw_quad(
        &mut self,
        p0: Vec2,
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(vec![p0, p1, p2, p3], stroke, fill);
    }

    /// Draw a straight line onto the canvas, projected from the camera.
    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, stroke: Option<Stroke>, fill: Option<Color>) {
        self.draw_shape(vec![p0, p1], stroke, fill);
    }

    /// Transform any given point from world space to camera space.
    /// Allows to scale to a given resolution width.
    pub fn to_camera_space(&self, point: Vec2) -> Vec2 {
        let point = point - self.translate;
        self.to_camera_mat.mul_vec2(point)
    }

    /// Transform any given point from camera space to world space.
    pub fn to_world_space(&self, point: Vec2) -> Vec2 {
        let point = self.to_world_mat.mul_vec2(point);
        point + self.translate
    }
}
