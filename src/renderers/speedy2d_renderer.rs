use crate::{Color, LineEnd, Renderer, Shape, Stroke};
use glam::{Mat2, UVec2, Vec2};
use glutin::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};
use once_cell::sync::OnceCell;
use speedy2d::{dimen::Vector2, shape::Polygon, GLRenderer};
use std::{
    f32::consts::PI,
    sync::mpsc::{sync_channel, SyncSender},
};

#[cfg(target_os = "windows")]
use glutin::platform::windows::EventLoopExtWindows;

#[cfg(target_os = "linux")]
use glutin::platform::unix::EventLoopExtUnix;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("Only windows and linux are supported when using Speedy2D.");

static SPEEDY2D_CANVAS_CHANNEL: OnceCell<SyncSender<Speedy2dRenderer>> = OnceCell::new();

/// Settings to configure [Speedy2dRenderer]
#[derive(Default, Clone)]
pub struct Speedy2dRendererSettings {
    /// Size of the output window. This will only matter the first time this renderer is used.
    pub window_size: UVec2,
    /// An optional background color.
    pub background: Option<Color>,
    /// Will make sure to include everything vertically when mapping from Camera Space to the image. Otherwise will do so horizontally.
    pub preserve_height: bool,
    /// Title of the window to render to. This will only matter if this is the first time you rendered with [Speedy2dRenderer].
    pub window_title: String,
}

/// A renderer that uses [Speedy2D](https://github.com/QuantumBadger/Speedy2D).
///
/// A single window will open. If a Speedy2D window is already open, it will render to that window.
/// This renderer currently only works on Windows and Linux, but more platforms are planned.
///
/// All rendering currently happens on a seperate thread.
#[derive(Default, Clone)]
pub struct Speedy2dRenderer {
    polygons: Vec<(Polygon, speedy2d::color::Color)>,
    window_size: UVec2,
    window_title: String,
    scale: f32,
    center_offset: Vec2,
    background: Option<Color>,
}

impl Renderer for Speedy2dRenderer {
    type Settings = Speedy2dRendererSettings;

    type Output = ();

    fn new(settings: Self::Settings) -> Self {
        let (scale, center_offset) = if settings.preserve_height {
            let scale = settings.window_size.y as f32 / 2.0;
            (
                scale,
                Vec2::new(settings.window_size.x as f32 / 2.0 / scale, 1.0),
            )
        } else {
            let scale = settings.window_size.x as f32 / 2.0;
            (
                scale,
                Vec2::new(1.0, settings.window_size.y as f32 / 2.0 / scale),
            )
        };

        Self {
            polygons: Vec::new(),
            window_size: settings.window_size,
            window_title: settings.window_title,
            scale,
            center_offset,
            background: settings.background,
        }
    }

    fn render(&mut self, shape: &Shape) {
        let points: Vec<Vector2<f32>> = shape
            .points
            .iter()
            .map(|p| {
                let p = (Vec2::new(p.x, -p.y) + self.center_offset) * self.scale;
                (p.x, p.y).into()
            })
            .collect();

        if let Some(fill) = shape.fill {
            self.polygons
                .push((Polygon::new(points.as_slice()), fill.into()));
        }

        // Draw stroke
        if let Some(stroke) = shape.stroke {
            let mut points = points.iter().peekable();

            if let Some(mut last_point) = points.next() {
                let second_point = *points.next().unwrap();

                let mut gradient_normalized = (second_point - *last_point).normalize().unwrap();
                let mut gradient_thickness =
                    gradient_normalized * (stroke.width * self.scale / 2.0);
                let mut offset = gradient_thickness.rotate_90_degrees_anticlockwise();

                self.polygons.push((
                    Polygon::new(&[
                        last_point + offset,
                        last_point - offset,
                        second_point - offset,
                        second_point + offset,
                    ]),
                    stroke.color.into(),
                ));

                fn line_end(
                    last_point: &Vector2<f32>,
                    stroke: Stroke,
                    scale: f32,
                    gradient_normalized: Vector2<f32>,
                    gradient_thickness: Vector2<f32>,
                    offset: Vector2<f32>,
                ) -> Polygon {
                    match stroke.line_end {
                        LineEnd::Butt => Polygon::new(&[
                            last_point - gradient_thickness + offset,
                            last_point - gradient_thickness - offset,
                            last_point - offset,
                            last_point + offset,
                        ]),
                        LineEnd::Round => {
                            // Generate half-circle
                            let center = last_point;
                            let radius = scale * stroke.width / 2.0;
                            let sides = 32;

                            let rotation = Mat2::from_cols(
                                Vec2::new(-gradient_normalized.y, gradient_normalized.x),
                                Vec2::new(-gradient_normalized.x, -gradient_normalized.y),
                            );

                            let mut points = Vec::with_capacity(sides);

                            points.push(last_point - offset);

                            for n in 0..sides {
                                let p = rotation.mul_vec2(Vec2::new(
                                    radius * (PI * n as f32 / sides as f32).cos(),
                                    radius * (PI * n as f32 / sides as f32).sin(),
                                ));

                                points.push(Vector2::new(p.x + center.x, p.y + center.y));
                            }

                            points.push(last_point + offset);

                            Polygon::new(&points)
                        }
                    }
                }

                self.polygons.push((
                    line_end(
                        last_point,
                        stroke,
                        self.scale,
                        gradient_normalized,
                        gradient_thickness,
                        offset,
                    ),
                    stroke.color.into(),
                ));

                last_point = &second_point;

                // Draw main line
                for point in points {
                    gradient_normalized = (point - *last_point).normalize().unwrap();
                    gradient_thickness = gradient_normalized * (stroke.width * self.scale / 2.0);
                    offset = gradient_thickness.rotate_90_degrees_anticlockwise();

                    self.polygons.push((
                        Polygon::new(&[
                            last_point - gradient_thickness + offset,
                            last_point - gradient_thickness - offset,
                            point - offset,
                            point + offset,
                        ]),
                        stroke.color.into(),
                    ));

                    last_point = point;
                }

                self.polygons.push((
                    line_end(
                        last_point,
                        stroke,
                        self.scale,
                        gradient_normalized * -1.0,
                        gradient_thickness * -1.0,
                        offset,
                    ),
                    stroke.color.into(),
                ));
            }
        }
    }

    fn finalize(self) -> Self::Output {
        let sender = match SPEEDY2D_CANVAS_CHANNEL.get() {
            Some(sender) => sender.clone(),
            None => {
                // Spawn a new thread to do rendering.
                let (sender, receiver) = sync_channel::<Self>(4);
                std::thread::spawn(move || {
                    // Since we are running Speedy2D on a seperate thread, we have to manage the OpenGL context ourselves.
                    let el = EventLoop::new_any_thread();
                    let wb = WindowBuilder::new().with_resizable(false);
                    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

                    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

                    let mut renderer = unsafe {
                        GLRenderer::new_for_gl_context((640, 480), |fn_name| {
                            windowed_context.get_proc_address(fn_name) as *const _
                        })
                    }
                    .unwrap();

                    let mut last_update = Self::default();

                    el.run(move |event: Event<()>, _, control_flow| {
                        // Update the canvas if there is a new one.
                        if let Ok(update) = receiver.try_recv() {
                            last_update = update;
                            windowed_context.window().set_inner_size(PhysicalSize::new(
                                last_update.window_size.x,
                                last_update.window_size.y,
                            ));
                            renderer.set_viewport_size_pixels(Vector2 {
                                x: last_update.window_size.x,
                                y: last_update.window_size.y,
                            });
                            windowed_context
                                .window()
                                .set_title(last_update.window_title.as_str());
                        }

                        match event {
                            Event::LoopDestroyed => return,
                            Event::WindowEvent { event, .. } => match event {
                                WindowEvent::Resized(physical_size) => {
                                    windowed_context.resize(physical_size)
                                }
                                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                                _ => (),
                            },
                            _ => (),
                        }

                        renderer.draw_frame(|graphics| {
                            if let Some(background) = last_update.background {
                                graphics.clear_screen(background.into());

                                for (polygon, color) in &last_update.polygons {
                                    graphics.draw_polygon(polygon, *color)
                                }
                            }
                        });
                        windowed_context.swap_buffers().unwrap();
                    });
                });

                SPEEDY2D_CANVAS_CHANNEL.set(sender.clone()).unwrap();

                sender
            }
        };

        sender.send(self).unwrap();
    }
}
