use crate::primitive::Primitive;
use crate::TerminalRenderer;

use iced_native::widget::scrollable;
use iced_native::Rectangle;

impl scrollable::Renderer for TerminalRenderer {
    fn scrollbar(
        &self,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        _offset: u32,
    ) -> Option<scrollable::Scrollbar> {
        None
    }

    fn draw(
        &mut self,
        _scrollable: &scrollable::State,
        bounds: Rectangle,
        _content_bounds: Rectangle,
        _is_mouse_over: bool,
        _is_mouse_over_scrollbar: bool,
        _scrollbar: Option<scrollable::Scrollbar>,
        offset: u32,
        content: Self::Output,
    ) -> Primitive {
        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds),
            content.with_offset(offset as i32),
        ])
    }
}
