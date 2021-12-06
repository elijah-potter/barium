use std::fmt::Write;

use glam::Vec2;

use crate::{
    canvas::{CanvasElement, CanvasElementVariant},
    regular_polygon_points,
    renderer::Renderer,
};

#[derive(Clone)]
/// Settings for [SkiaRenderer].
pub struct ObjRendererSettings {
    /// How much to seperate each element on the Z-Axis.
    pub z_offset: f32,
    /// How many sides a [Ellipse](crate::CanvasElement::Ellipse) will have.
    pub ellipse_face_count: usize,
    /// Intended filename of the `.mtl` material file.
    pub mtl_filename: String,
}

/// A renderer to the Wavefront .obj format.
///
/// ## Caveats
/// Each element is drawn in 2D, offset by the configurable [z_offset](ObjRendererSettings::z_offset) from each other.
/// Any [Stroke](crate::Stroke) is ignored.
/// Any [CanvasElementPostEffect](crate::CanvasElementPostEffect) is ignored.
/// The `alpha` channel of any [Color](crate::Color) is ignored.
/// Any (Ellipse)[crate::CanvasElementVariant::Ellipse] or (Polygon)[crate::CanvasElementVariant::Polygon] whose `fill` is None, will be colored black.
///
/// ## Output
/// The renderer outputs a tuple containing `(.obj file content, .mtl file content)`. This is necessary to output both geometry and color data.
pub struct ObjRenderer {
    settings: ObjRendererSettings,
    current_z_offset: f32,
    current_vertex_index: usize,
    current_face_index: usize,
    obj: String,
    mtl: String,
}

impl Renderer for ObjRenderer {
    type Settings = ObjRendererSettings;
    type Output = (String, String);

    fn new(settings: Self::Settings) -> Self {
        Self {
            settings: settings.clone(),
            current_z_offset: 0.0,
            current_vertex_index: 1,
            current_face_index: 0,
            obj: format!("mtllib {}\n", settings.mtl_filename),
            mtl: "newmtl black\nKd 0.0 0.0 0.0\n".to_owned(),
        }
    }

    fn render(&mut self, element: &CanvasElement) {
        match &element.variant {
            CanvasElementVariant::Blank => (),
            CanvasElementVariant::PolyLine { points, stroke: _ } => {
                for point in points {
                    writeln!(
                        self.obj,
                        "v {} {} {}",
                        point.x, point.y, self.current_z_offset
                    )
                    .unwrap();

                    self.current_vertex_index += 1;
                }

                write!(self.obj, "l ").unwrap();
                for i in self.current_vertex_index - points.len()..self.current_vertex_index {
                    write!(self.obj, "{} ", i).unwrap();
                }
                writeln!(self.obj).unwrap();
                self.current_z_offset += self.settings.z_offset;
            }
            CanvasElementVariant::Ellipse {
                center,
                radius,
                fill,
                stroke: _,
            } => {
                let mut ellipse_points = regular_polygon_points(
                    Vec2::ZERO,
                    self.settings.ellipse_face_count,
                    radius.x,
                    0.0,
                );

                for point in ellipse_points.iter_mut() {
                    point.y *= radius.y / radius.x;
                    *point += *center;
                }

                self.render(&CanvasElement {
                    variant: CanvasElementVariant::Polygon {
                        points: ellipse_points,
                        fill: *fill,
                        stroke: None,
                    },
                    ..Default::default()
                })
            }
            CanvasElementVariant::Polygon {
                points,
                fill,
                stroke: _,
            } => {
                for point in points {
                    writeln!(
                        self.obj,
                        "v {} {} {}",
                        point.x, point.y, self.current_z_offset
                    )
                    .unwrap();

                    self.current_vertex_index += 1;
                }

                if let Some(fill) = fill {
                    write!(self.obj, "usemtl f{}\nf ", self.current_face_index).unwrap();

                    writeln!(
                        self.mtl,
                        "newmtl f{}\nKd {} {} {}",
                        self.current_face_index,
                        fill.r(),
                        fill.g(),
                        fill.b()
                    )
                    .unwrap();
                } else {
                    write!(self.obj, "usemtl black\n f ").unwrap()
                }

                for i in self.current_vertex_index - points.len()..self.current_vertex_index {
                    write!(self.obj, "{} ", i).unwrap();
                }
                writeln!(self.obj).unwrap();

                self.current_z_offset += self.settings.z_offset;
                self.current_face_index += 1;
            }
            CanvasElementVariant::Cluster { children } => {
                for child in children {
                    self.render(child);
                }
            }
        }
    }

    fn finalize(self) -> Self::Output {
        (self.obj, self.mtl)
    }
}
