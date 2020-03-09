use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::text_input;
use iced_native::{Font, HorizontalAlignment, Point, Rectangle, VerticalAlignment};

impl text_input::Renderer for TerminalRenderer {
    fn default_size(&self) -> u16 {
        3
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        src_text_bounds: Rectangle,
        _cursor_position: Point,
        _size: u16,
        placeholder: &str,
        value: &text_input::Value,
        _state: &text_input::State,
    ) -> Primitive {
        let mut text = value.to_string();
        if text == "" {
            text = placeholder.into();
        }
        let bounds_text = Rectangle {
            width: src_text_bounds.width,
            height: src_text_bounds.height,
            x: src_text_bounds.x + 1.,
            y: src_text_bounds.y + 1.,
        };
        let prim_text = <Self as iced_native::widget::text::Renderer>::draw(
            self,
            bounds_text,
            &text,
            1,
            Font::Default,
            None,
            HorizontalAlignment::Left,
            VerticalAlignment::Top,
        );
        Primitive::Group(vec![Primitive::BoxDisplay(bounds), prim_text])
    }
}
