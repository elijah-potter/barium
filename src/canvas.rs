use std::f32::consts::PI;

use crate::{color::Color, PathBuilder};
use glam::{Mat2, Vec2};

use retain_mut::RetainMut;

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
    /// The intended format the renderer will output.
    type Output;
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
    points_per_unit: usize,
    zoom: f32,
    translation: Vec2,
    to_camera_matrix: Mat2,
    to_world_matrix: Mat2,
    shapes: Vec<Shape>,
}

impl Default for Canvas {
    #[inline]
    fn default() -> Self {
        Self {
            points_per_unit: 1000,
            zoom: 1.0,
            translation: Vec2::ZERO,
            to_camera_matrix: Mat2::IDENTITY,
            to_world_matrix: Mat2::IDENTITY,
            shapes: Vec::new(),
        }
    }
}

impl Canvas {
    /// Create a new [Canvas].
    /// [points_per_unit](Self::points_per_unit) defines the resolution at which certain helper functions generate points, (circles, bezier curves).
    #[inline]
    pub fn new(points_per_unit: usize) -> Self {
        Self {
            points_per_unit,
            zoom: 1.0,
            translation: Vec2::ZERO,
            to_camera_matrix: Mat2::IDENTITY,
            to_world_matrix: Mat2::IDENTITY,
            shapes: Vec::new(),
        }
    }

    /// Render the canvas using a renderer of your choice.
    pub fn render<R: Renderer>(&self, mut renderer: R) -> R::Output {
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

    /// Rotate the camera counter-clockwise.
    pub fn rotate_camera(&mut self, radians: f32) {
        let rotate_mat = Mat2::from_angle(radians);
        self.to_camera_matrix = rotate_mat.mul_mat2(&self.to_camera_matrix);
        self.to_world_matrix = self.to_camera_matrix.inverse();
    }

    /// Moves the camera by a certain amount. This is effected by zoom.
    /// 
    /// For example, if the zoom is set to `1/100` and the camera is moved by `(1.0, 1.0)`, it will actually be moving (100.0, 100.0).
    pub fn move_camera<P: Into<Vec2>>(&mut self, translation: P) {
        self.translation -= translation.into();
        self.translation = -self.translation;
    }

    /// Zoom camera
    pub fn zoom_camera(&mut self, zoom: f32) {
        self.to_camera_matrix *= zoom;
        self.to_world_matrix = self.to_camera_matrix.inverse();
        self.zoom *= zoom;
    }

    /// Clears the canvas
    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    /// Draw a shape onto the canvas, projected from the camera.
    ///
    /// If a shape as one or fewer points, it will be discarded.
    pub fn draw_shape<C: Into<Vec<Vec2>>>(
        &mut self,
        points: C,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let mut points: Vec<Vec2> = points.into();

        if points.len() <= 1 {
            return;
        }

        let mut last_point = Vec2::ZERO * f32::INFINITY;
        RetainMut::retain_mut(&mut points, |point| {
            let r = last_point != *point;
            last_point = *point;
            *point = self.to_world_space(last_point);
            r
        });

        stroke.map(|mut v| {
            v.width /= self.zoom;
            v
        });

        self.shapes.push(Shape {
            points,
            stroke,
            fill,
        })
    }

    /// Draw a shape directly onto the canvas.
    ///
    /// If a shape as one or fewer points, it will be discarded.
    pub fn draw_shape_absolute<C: Into<Vec<Vec2>>>(
        &mut self,
        points: C,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let mut points: Vec<Vec2> = points.into();

        if points.len() <= 1 {
            return;
        }

        let mut last_point = Vec2::ZERO * f32::INFINITY;
        points.retain(|point| {
            let r = last_point != *point;
            last_point = *point;
            r
        });

        self.shapes.push(Shape {
            points,
            stroke,
            fill,
        })
    }

    /// Draw a rectangle onto the canvas, projected from the camera.
    pub fn draw_rect<P: Into<Vec2>>(
        &mut self,
        top_left: P,
        bottom_right: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let top_left = top_left.into();
        let bottom_right = bottom_right.into();

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

    /// Draw a rectangle directly onto the canvas.
    pub fn draw_rect_absolute<P: Into<Vec2>>(
        &mut self,
        top_left: P,
        bottom_right: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let top_left = top_left.into();
        let bottom_right = bottom_right.into();

        self.draw_shape_absolute(
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
    pub fn draw_regular_polygon<P: Into<Vec2>>(
        &mut self,
        center: P,
        sides: usize,
        radius: f32,
        rotation: f32,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        if sides < 3 {
            panic!("There must be at least 3 sides in a regular polygon.")
        }

        let center = center.into();

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

    /// Draws a regular polygon directly onto the canvas.
    ///
    /// Rotation is in radians.
    /// Will panic if `sides` < 3.
    pub fn draw_regular_polygon_absolute<P: Into<Vec2>>(
        &mut self,
        center: P,
        sides: usize,
        radius: f32,
        rotation: f32,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        if sides < 3 {
            panic!("There must be at least 3 sides in a regular polygon.")
        }

        let center = center.into();

        let mut points = Vec::with_capacity(sides + 1);

        for n in 0..sides {
            points.push(Vec2::new(
                radius * (2.0 * PI * n as f32 / sides as f32 + rotation).cos() + center.x,
                radius * (2.0 * PI * n as f32 / sides as f32 + rotation).sin() + center.y,
            ))
        }

        // Connect first and last points to complete polygon.
        points.push(points[0]);

        self.draw_shape_absolute(points, stroke, fill)
    }

    /// Draws a circle onto the canvas, projected from the camera.
    /// This is a wrapper over [draw_regular_polygon](Self::draw_regular_polygon).
    /// If you want high-quality circles, use that function directly or adjust [points_per_unit](Self::points_per_unit) to fit your needs.
    pub fn draw_circle<P: Into<Vec2>>(
        &mut self,
        center: P,
        radius: f32,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let center = center.into();
        let circumference = 2.0 * PI * radius;
        let sides = (circumference * self.points_per_unit as f32) as usize;
        if sides > 2 {
            self.draw_regular_polygon(center, sides, radius, 0.0, stroke, fill);
        }
    }

    /// Draws a circle directly onto the canvas.
    /// This is a wrapper over [draw_regular_polygon_absolute](Self::draw_regular_polygon_absolute).
    /// If you want high-quality circles, use that function directly or adjust [points_per_unit](Self::points_per_unit) to fit your needs.
    pub fn draw_circle_absolute<P: Into<Vec2>>(
        &mut self,
        center: P,
        radius: f32,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        let center = center.into();
        let circumference = 2.0 * PI * radius;
        let sides = (circumference * self.points_per_unit as f32) as usize;
        if sides > 2 {
            self.draw_regular_polygon(center, sides, radius, 0.0, stroke, fill);
        }
    }

    /// Draw a triangle onto the canvas, projected from the camera.
    pub fn draw_triangle<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        p2: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(vec![p0.into(), p1.into(), p2.into()], stroke, fill);
    }

    /// Draw a triangle directly onto the canvas.
    pub fn draw_triangle_absolute<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        p2: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape_absolute(vec![p0.into(), p1.into(), p2.into()], stroke, fill);
    }

    /// Draw a quad onto the canvas, projected from the camera.
    pub fn draw_quad<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        p2: P,
        p3: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(
            vec![p0.into(), p1.into(), p2.into(), p3.into()],
            stroke,
            fill,
        );
    }

    /// Draw a quad directly onto the canvas.
    pub fn draw_quad_absolute<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        p2: P,
        p3: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape_absolute(
            vec![p0.into(), p1.into(), p2.into(), p3.into()],
            stroke,
            fill,
        );
    }

    /// Create and draw a path onto the canvas, projected from the camera.
    ///
    /// This is similar to the `svg` `<path>` instruction.
    pub fn draw_path<F>(&mut self, stroke: Option<Stroke>, fill: Option<Color>, f: F)
    where
        F: FnOnce(PathBuilder) -> PathBuilder,
    {
        f(PathBuilder::new(self.points_per_unit)).build(stroke, fill, self);
    }

    /// Create and draw a path directly onto the canvas.
    ///
    /// This is similar to the `svg` `<path>` instruction.
    pub fn draw_path_absolute<F>(&mut self, stroke: Option<Stroke>, fill: Option<Color>, f: F)
    where
        F: FnOnce(PathBuilder) -> PathBuilder,
    {
        f(PathBuilder::new(self.points_per_unit)).build_absolute(stroke, fill, self);
    }

    /// Draw a quadratic bezier curve onto the canvas, projected from the camera.
    pub fn draw_quadratic_bezier<P: Into<Vec2>>(
        &mut self,
        start_point: P,
        control_point: P,
        end_point: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_path(stroke, fill, |path| {
            path.move_to(start_point.into())
                .quadratic_bezier_to(end_point.into(), control_point.into())
        });
    }

    /// Draw a quadratic bezier curve directly onto the canvas..
    pub fn draw_quadratic_bezier_absolute<P: Into<Vec2>>(
        &mut self,
        start_point: P,
        control_point: P,
        end_point: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_path_absolute(stroke, fill, |path| {
            path.move_to(start_point.into())
                .quadratic_bezier_to(end_point.into(), control_point.into())
        });
    }

    /// Draw a cubic bezier curve onto the canvas, projected from the camera.
    pub fn draw_cubic_bezier<P: Into<Vec2>>(
        &mut self,
        start_point: P,
        control_point_0: P,
        control_point_1: P,
        end_point: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_path(stroke, fill, |path| {
            path.move_to(start_point.into()).cubic_bezier_to(
                end_point.into(),
                control_point_0.into(),
                control_point_1.into(),
            )
        });
    }

    /// Draw a cubic bezier curve directly onto the canvas.
    pub fn draw_cubic_bezier_absolute<P: Into<Vec2>>(
        &mut self,
        start_point: P,
        control_point_0: P,
        control_point_1: P,
        end_point: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_path_absolute(stroke, fill, |path| {
            path.move_to(start_point.into()).cubic_bezier_to(
                end_point.into(),
                control_point_0.into(),
                control_point_1.into(),
            )
        });
    }

    /// Draw a straight line onto the canvas, projected from the camera.
    pub fn draw_line<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape(vec![p0.into(), p1.into()], stroke, fill);
    }

    /// Draw a straight line directly onto the canvas.
    pub fn draw_line_absolute<P: Into<Vec2>>(
        &mut self,
        p0: P,
        p1: P,
        stroke: Option<Stroke>,
        fill: Option<Color>,
    ) {
        self.draw_shape_absolute(vec![p0.into(), p1.into()], stroke, fill);
    }

    /// Draw a line made of several segments onto the canvas, projected from the camera.
    pub fn draw_polyline<C: Into<Vec<Vec2>>>(&mut self, points: C, stroke: Stroke) {
        self.draw_shape(points, Some(stroke), None);
    }

    /// Draw a line made of several segments directly onto the canvas.
    pub fn draw_polyline_absolute<C: Into<Vec<Vec2>>>(&mut self, points: C, stroke: Stroke) {
        self.draw_shape_absolute(points, Some(stroke), None);
    }

    /// Draw a solid shape made of several sides onto the canvas, projected from the camera.
    pub fn draw_polygon<C: Into<Vec<Vec2>>>(&mut self, points: C, fill: Color) {
        self.draw_shape(points, None, Some(fill));
    }

    /// Draw a solid shape made of several sides directly onto the canvas.
    pub fn draw_polygon_absolute<C: Into<Vec<Vec2>>>(&mut self, points: C, fill: Color) {
        self.draw_shape_absolute(points, None, Some(fill));
    }

    /// Transform any given point from world space to camera space.
    /// Allows to scale to a given resolution width.
    pub fn to_camera_space<P: Into<Vec2>>(&self, point: P) -> Vec2 {
        self.to_camera_matrix.mul_vec2(point.into() - self.translation)
    }

    /// Transform any given point from camera space to world space.
    pub fn to_world_space<P: Into<Vec2>>(&self, point: P) -> Vec2 {
        self.to_world_matrix.mul_vec2(point.into()) + self.translation
    }

    /// Get the canvas' points per unit.
    ///
    /// This is essentially how detailed it will generate certain kinds of geometry (bezier curves, circles).
    pub fn points_per_unit(&self) -> usize {
        self.points_per_unit
    }

    /// Set the canvas' points per unit.
    ///
    /// This is essentially how detailed it will generate certain kinds of geometry (bezier curves, circles).
    pub fn set_points_per_unit(&mut self, points_per_unit: usize) {
        self.points_per_unit = points_per_unit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.001;

    /// Assert that two [Vec2] are within [EPSILON] of each other.
    #[inline]
    fn assert_vec2_eq<P: Into<Vec2>>(a: P, b: P) {
        let a: Vec2 = a.into();
        let b: Vec2 = b.into();

        if !a.abs_diff_eq(b, EPSILON) {
            panic!("assertion failed: {}, {}", a, b);
        }
    }

    /// Verify that the default camera does not transform points when converting to camera space.
    #[test]
    fn no_transform_world_camera() {
        let canvas = Canvas::default();

        assert_vec2_eq(canvas.to_camera_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_camera_space(Vec2::ONE), Vec2::ONE);
        assert_vec2_eq(canvas.to_camera_space(-Vec2::ONE), -Vec2::ONE);
        assert_vec2_eq(canvas.to_camera_space((-1.0, 1.0)), Vec2::new(-1.0, 1.0));
        assert_vec2_eq(
            canvas.to_camera_space(Vec2::new(1.0, -1.0)),
            Vec2::new(1.0, -1.0),
        );
    }

    /// Verify that the default camera does not transform points when converting to world space.
    #[test]
    fn no_transform_camera_world() {
        let canvas = Canvas::default();

        assert_vec2_eq(canvas.to_world_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_world_space(Vec2::ONE), Vec2::ONE);
        assert_vec2_eq(canvas.to_world_space(-Vec2::ONE), -Vec2::ONE);
        assert_vec2_eq(
            canvas.to_world_space(Vec2::new(-1.0, 1.0)),
            Vec2::new(-1.0, 1.0),
        );
        assert_vec2_eq(
            canvas.to_world_space(Vec2::new(1.0, -1.0)),
            Vec2::new(1.0, -1.0),
        );
    }

    /// Verify that a translated camera correctly transforms points when converting to camera space.
    #[test]
    fn translate_transform_world_camera() {
        let mut canvas = Canvas::default();

        canvas.move_camera(Vec2::ONE);

        assert_vec2_eq(canvas.to_camera_space(Vec2::ZERO), Vec2::new(-1.0, -1.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::ONE), Vec2::ZERO);
        assert_vec2_eq(canvas.to_camera_space(-Vec2::ONE), -Vec2::ONE * 2.0);
        assert_vec2_eq(
            canvas.to_camera_space(Vec2::new(-1.0, 1.0)),
            Vec2::new(-2.0, 0.0),
        );
        assert_vec2_eq(
            canvas.to_camera_space(Vec2::new(1.0, -1.0)),
            Vec2::new(0.0, -2.0),
        );
    }

    /// Verify that a translated camera correctly transforms points when converting to world space.
    #[test]
    fn translate_transform_camera_world() {
        let mut canvas = Canvas::default();

        canvas.move_camera(Vec2::ONE);

        assert_vec2_eq(canvas.to_world_space(Vec2::ZERO), Vec2::new(1.0, 1.0));
        assert_vec2_eq(canvas.to_world_space(Vec2::ONE), Vec2::ONE * 2.0);
        assert_vec2_eq(canvas.to_world_space(-Vec2::ONE), Vec2::ZERO);
        assert_vec2_eq(
            canvas.to_world_space(Vec2::new(-1.0, 1.0)),
            Vec2::new(0.0, 2.0),
        );
        assert_vec2_eq(
            canvas.to_world_space(Vec2::new(1.0, -1.0)),
            Vec2::new(2.0, 0.0),
        );
    }

    /// Verify that a rotated camera correctly transforms points when converting to camera space.
    #[test]
    fn rotate_transform_world_camera() {
        let mut canvas = Canvas::default();

        canvas.rotate_camera(PI / 2.0);

        assert_vec2_eq(canvas.to_camera_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_camera_space(Vec2::ONE), Vec2::new(-1.0, 1.0));
        assert_vec2_eq(canvas.to_camera_space(-Vec2::ONE), Vec2::new(1.0, -1.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(-1.0, 1.0)), -Vec2::ONE);
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(1.0, -1.0)), Vec2::ONE);
    }

    /// Verify that a rotated camera correctly transforms points when converting to world space.
    #[test]
    fn rotate_transform_camera_world() {
        let mut canvas = Canvas::default();

        canvas.rotate_camera(PI / 2.0);

        assert_vec2_eq(canvas.to_world_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_world_space(Vec2::ONE), Vec2::new(1.0, -1.0));
        assert_vec2_eq(canvas.to_world_space(-Vec2::ONE), Vec2::new(-1.0, 1.0));
        assert_vec2_eq(canvas.to_world_space(Vec2::new(-1.0, 1.0)), Vec2::ONE);
        assert_vec2_eq(canvas.to_world_space(Vec2::new(1.0, -1.0)), -Vec2::ONE);
    }

    /// Verify that a zoomed camera correctly transforms points when converting to camera space.
    #[test]
    fn zoom_transform_world_camera() {
        let mut canvas = Canvas::default();

        canvas.zoom_camera(2.0);

        assert_vec2_eq(canvas.to_camera_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_camera_space(Vec2::ONE), Vec2::ONE * 2.0);
        assert_vec2_eq(canvas.to_camera_space(-Vec2::ONE), Vec2::ONE * -2.0);
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(-1.0, 1.0)), Vec2::new(-2.0, 2.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(1.0, -1.0)), Vec2::new(2.0, -2.0));
    }

    /// Verify that a zoomed camera correctly transforms points when converting to world space.
    #[test]
    fn zoom_transform_camera_world() {
        let mut canvas = Canvas::default();

        canvas.zoom_camera(2.0);

        assert_vec2_eq(canvas.to_world_space(Vec2::ZERO), Vec2::ZERO);
        assert_vec2_eq(canvas.to_world_space(Vec2::ONE), Vec2::ONE * 0.5);
        assert_vec2_eq(canvas.to_world_space(-Vec2::ONE), Vec2::ONE * -0.5);
        assert_vec2_eq(canvas.to_world_space(Vec2::new(-1.0, 1.0)), Vec2::new(-0.5, 0.5));
        assert_vec2_eq(canvas.to_world_space(Vec2::new(1.0, -1.0)), Vec2::new(0.5, -0.5));
    }

    /// Verify that a fully moved, rotated, and zoomed camera correctly transforms points when converting to camera space.
    #[test]
    fn full_transform_world_camera() {
        let mut canvas = Canvas::default();

        canvas.move_camera(Vec2::ONE);
        canvas.rotate_camera(PI / 2.0);
        canvas.zoom_camera(2.0);

        assert_vec2_eq(canvas.to_camera_space(Vec2::ZERO), Vec2::new(2.0, -2.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::ONE), Vec2::ZERO);
        assert_vec2_eq(canvas.to_camera_space(-Vec2::ONE), Vec2::new(4.0, -4.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(-1.0, 1.0)), Vec2::new(0.0,-4.0));
        assert_vec2_eq(canvas.to_camera_space(Vec2::new(1.0, -1.0)), Vec2::new(4.0, 0.0));
    }

    /// Verify that a fully moved, rotated, and zoomed camera correctly transforms points when converting to world space.
    #[test]
    fn full_transform_camera_world() {
        let mut canvas = Canvas::default();

        canvas.move_camera(Vec2::ONE);
        canvas.rotate_camera(PI / 2.0);
        canvas.zoom_camera(2.0);

        assert_vec2_eq(canvas.to_world_space(Vec2::ZERO), Vec2::ONE);
        assert_vec2_eq(canvas.to_world_space(Vec2::ONE), Vec2::new(1.5, 0.5));
        assert_vec2_eq(canvas.to_world_space(-Vec2::ONE), Vec2::new(0.5, 1.5));
        assert_vec2_eq(canvas.to_world_space(Vec2::new(-1.0, 1.0)), Vec2::new(1.5,1.5));
        assert_vec2_eq(canvas.to_world_space(Vec2::new(1.0, -1.0)), Vec2::new(0.5, 0.5));
    }
}
