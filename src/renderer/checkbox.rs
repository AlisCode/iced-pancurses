use crate::PancursesRenderer;

use iced::widget::checkbox::Renderer as CheckboxRenderer;
use iced::{MouseCursor, Point, Rectangle};

impl CheckboxRenderer for PancursesRenderer {
    fn draw(
        &mut self,
        _cursor_position: Point,
        bounds: Rectangle,
        _text_bounds: Rectangle,
        is_checked: bool,
    ) -> MouseCursor {
        let x = bounds.x as i32;
        let y = bounds.y as i32;
        self.window.mv(y, x);
        if is_checked {
            self.window.addch('x');
        } else {
            self.window.addch('o');
        }
        MouseCursor::OutOfBounds
    }
}
