use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::{space, Rectangle};

impl space::Renderer for TerminalRenderer {
    fn draw(&mut self, _bounds: Rectangle) -> Self::Output {
        Primitive::Empty
    }
}
