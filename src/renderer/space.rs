use crate::primitive::Primitive;
use crate::PancursesRenderer;
use iced_native::{space, Rectangle};

impl space::Renderer for PancursesRenderer {
    fn draw(&mut self, _bounds: Rectangle) -> Self::Output {
        Primitive::Empty
    }
}