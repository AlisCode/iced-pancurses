use crate::primitive::Primitive;
use crate::PancursesRenderer;

use iced_native::widget::scrollable;
use iced_native::{Point, Rectangle};

impl scrollable::Renderer for PancursesRenderer {
    /*
    fn scrollbar(
        &self,
        bounds: Rectangle,
        content_bounds: Rectangle,
        offset: u32,
    ) -> Option<scrollable::Scrollbar> {
        None
    }
    */

    fn is_mouse_over_scrollbar(
        &self,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _cursor_position: Point,
    ) -> bool {
        false
    }

    fn draw(
        &mut self,
        _state: &scrollable::State,
        bounds: Rectangle,
        _content_bounds: Rectangle,
        _is_mouse_over: bool,
        _is_mouse_over_scrollbar: bool,
        offset: u32,
        content: Primitive,
    ) -> Primitive {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds),
            content.with_offset(offset as i32),
        ])
    }
}
