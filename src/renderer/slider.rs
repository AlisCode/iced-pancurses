use crate::PancursesRenderer;

use iced::widget::slider::{Renderer as SliderRenderer, State as SliderState};
use iced::{MouseCursor, Point, Rectangle};

use std::ops::RangeInclusive;

impl SliderRenderer for PancursesRenderer {
    fn draw(
        &mut self,
        _cursor_position: Point,
        bounds: Rectangle,
        _state: &SliderState,
        range: RangeInclusive<f32>,
        value: f32,
    ) -> MouseCursor {
        let (range_start, range_end) = range.into_inner();
        //let marker_offset = (value - range_start) * (bounds.width) / (range_end - range_start);
        let marker_offset =
            (bounds.width - 1.) * ((value - range_start) / (range_end - range_start).max(1.0));

        if let Ok(sub_win) = self.window.subwin(
            bounds.height as i32,
            bounds.width as i32,
            bounds.y as i32,
            bounds.x as i32,
        ) {
            let col = self.color_registry["white"];
            sub_win.attrset(pancurses::COLOR_PAIR(col.into()));
            sub_win.border(0, 0, 0, 0, 0, 0, 0, 0);
            sub_win.mv(1, 0);
            let col = self.color_registry["positive"];
            sub_win.attrset(pancurses::COLOR_PAIR(col.into()));
            sub_win.hline('-', bounds.width as i32);

            sub_win.mv(1, marker_offset as i32);
            let col = self.color_registry["primary"];
            sub_win.attrset(pancurses::COLOR_PAIR(col.into()));
            sub_win.addch('x');
            sub_win.delwin();
        }
        MouseCursor::OutOfBounds
    }
}
