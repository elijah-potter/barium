use crate::{Canvas, Color, Stroke};
use glam::Vec2;

/// A builder to describe the shape of a path.
///
/// Primarily meant to be used through [Canvas::draw_path] and [Canvas::draw_path_absolute].
/// The "pen" starts at the origin.
#[derive(Clone, Debug)]
pub struct PathBuilder {
    points_per_unit: usize,
    shapes: Vec<Vec<Vec2>>,
    current_shape: Vec<Vec2>,
}

impl PathBuilder {
    pub(crate) fn new(points_per_unit: usize) -> Self {
        Self {
            points_per_unit,
            shapes: Vec::new(),
            current_shape: vec![Vec2::ZERO],
        }
    }

    /// Move the "pen" to another part of the canvas without drawing a line.
    pub fn move_to<P: Into<Vec2>>(mut self, point: P) -> Self {
        if self.current_shape.len() > 1 {
            self.shapes.push(self.current_shape);
        }

        self.current_shape = Vec::new();
        self.current_shape.push(point.into());

        self
    }

    /// Draw a straight line to another spot on the canvas.
    pub fn line_to<P: Into<Vec2>>(mut self, point: P) -> Self {
        let point = point.into();
        if self.current_shape[self.current_shape.len() - 1] != point {
            self.current_shape.push(point);
        }
        self
    }

    /// Draw a quadratic bezier curve to another spot on the canvas.
    pub fn quadratic_bezier_to<P: Into<Vec2>>(mut self, end_point: P, control_point: P) -> Self {
        let start_point = self.current_shape[self.current_shape.len() - 1];
        let end_point = end_point.into();
        let control_point = control_point.into();

        let curve_length = start_point.distance(control_point) + control_point.distance(end_point);
        let point_count = curve_length * self.points_per_unit as f32;

        for i in 1..=point_count as usize {
            self.current_shape.push(Self::quadratic(
                start_point,
                control_point,
                end_point,
                i as f32 / point_count,
            ));
        }

        self
    }

    /// Draw a cubic bezier curve to another spot on the canvas.
    pub fn cubic_bezier_to<P: Into<Vec2>>(
        mut self,
        end_point: P,
        control_point_0: P,
        control_point_1: P,
    ) -> Self {
        let start_point = self.current_shape[self.current_shape.len() - 1];
        let end_point = end_point.into();
        let control_point_0 = control_point_0.into();
        let control_point_1 = control_point_1.into();

        let curve_length = start_point.distance(control_point_0)
            + control_point_0.distance(control_point_1)
            + control_point_1.distance(end_point);

        let point_count = curve_length * self.points_per_unit as f32;

        for i in 1..=point_count as usize {
            self.current_shape.push(Self::cubic(
                start_point,
                control_point_0,
                control_point_1,
                end_point,
                i as f32 / point_count,
            ));
        }

        self
    }

    /// Get the first point in the path.
    pub fn first_point(&self) -> Vec2 {
        if let Some(first) = self.shapes.first() {
            first[0]
        } else if self.current_shape.len() > 1 {
            self.current_shape[0]
        } else {
            unreachable!()
        }
    }

    /// Close the path.
    pub fn close(self) -> Self {
        let first_point = self.first_point();
        self.line_to(first_point)
    }

    pub(crate) fn build(
        mut self,
        stroke: Option<Stroke>,
        fill: Option<Color>,
        destination_canvas: &mut Canvas,
    ) {
        let mut raw_shapes = {
            self.shapes.push(self.current_shape);
            self.shapes
        };

        // We have to make a seperate shape for the fill to make sure we get the whole thing.
        if let Some(fill) = fill {
            let mut fill_shape = Vec::with_capacity(raw_shapes.iter().map(|v| v.len()).sum());

            for shape in raw_shapes.iter() {
                fill_shape.append(&mut shape.clone());
            }

            destination_canvas.draw_shape(fill_shape, None, Some(fill));
        }

        for shape in raw_shapes.drain(..) {
            destination_canvas.draw_shape(shape, stroke, None);
        }
    }

    pub(crate) fn build_absolute(
        mut self,
        stroke: Option<Stroke>,
        fill: Option<Color>,
        destination_canvas: &mut Canvas,
    ) {
        let mut raw_shapes = {
            self.shapes.push(self.current_shape);
            self.shapes
        };

        // We have to make a seperate shape for the fill to make sure we get the whole thing.
        if let Some(fill) = fill {
            let mut fill_shape = Vec::with_capacity(raw_shapes.iter().map(|v| v.len()).sum());

            for shape in raw_shapes.iter() {
                fill_shape.append(&mut shape.clone());
            }

            destination_canvas.draw_shape_absolute(fill_shape, None, Some(fill));
        }

        for shape in raw_shapes.drain(..) {
            destination_canvas.draw_shape_absolute(shape, stroke, None);
        }
    }

    fn point_on_line(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        a - ((a - b) * t)
    }

    fn quadratic(start: Vec2, middle: Vec2, end: Vec2, t: f32) -> Vec2 {
        let a = Self::point_on_line(start, middle, t);
        let b = Self::point_on_line(middle, end, t);
        Self::point_on_line(a, b, t)
    }

    fn cubic(start: Vec2, second: Vec2, third: Vec2, end: Vec2, t: f32) -> Vec2 {
        let a = Self::point_on_line(start, second, t);
        let b = Self::point_on_line(second, third, t);
        let c = Self::point_on_line(third, end, t);
        let d = Self::point_on_line(a, b, t);
        let e = Self::point_on_line(b, c, t);
        Self::point_on_line(d, e, t)
    }
}
