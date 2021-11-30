use crate::canvas::CanvasElement;

pub trait Renderer {
    type Settings;
    type Output;

    fn new(settings: Self::Settings) -> Self;
    fn render(&mut self, element: &CanvasElement);
    fn finalize(self) -> Self::Output;
}
