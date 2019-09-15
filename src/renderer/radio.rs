use crate::PancursesRenderer;
use iced::widget::radio::Renderer as RadioRenderer;
use iced::{MouseCursor, Point, Rectangle};

impl RadioRenderer for PancursesRenderer {
    fn draw(
        &mut self,
        _cursor_position: Point,
        bounds: Rectangle,
        _label_bounds: Rectangle,
        is_selected: bool,
    ) -> MouseCursor {
        let char_radio = if is_selected { 'x' } else { 'o' };
        let x = bounds.x as i32;
        let y = bounds.y as i32;
        self.window.mvaddch(y, x, char_radio);

        MouseCursor::OutOfBounds
    }
}
