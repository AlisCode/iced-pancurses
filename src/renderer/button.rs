use crate::primitive::Primitive;
use crate::PancursesRenderer;
use iced_native::widget::button;
use iced_native::{Background, Point, Rectangle};

impl button::Renderer for PancursesRenderer {
    fn draw(
        &mut self,
        bounds: Rectangle,
        _cursor_position: Point,
        _is_pressed: bool,
        _background: Option<Background>,
        _border_radius: u16,
        content: Self::Output,
    ) -> Self::Output {
        Primitive::Group(vec![Primitive::BoxDisplay(bounds), content])
    }
}
