use crate::renderer::text::TextLayout;
use crate::PancursesRenderer;
use iced::widget::button::{Class, Renderer as ButtonRenderer, State};
use iced::widget::text::HorizontalAlignment;
use iced::{MouseCursor, Point, Rectangle};

impl ButtonRenderer for PancursesRenderer {
    fn draw(
        &mut self,
        _cursor_position: Point,
        bounds: Rectangle,
        _state: &State,
        label: &str,
        class: Class,
    ) -> MouseCursor {
        let x = bounds.x as i32;
        let y = bounds.y as i32;
        let w = bounds.width as i32;
        let h = bounds.height as i32;

        match class {
            Class::Primary => self
                .window
                .attrset(pancurses::COLOR_PAIR(self.color_registry["primary"])),
            Class::Secondary => self
                .window
                .attrset(pancurses::COLOR_PAIR(self.color_registry["secondary"])),
            Class::Positive => self
                .window
                .attrset(pancurses::COLOR_PAIR(self.color_registry["positive"])),
        };

        if let Ok(sub_win) = self.window.subwin(h, w, y, x) {
            sub_win.border(0, 0, 0, 0, 0, 0, 0, 0);
            let layout_text =
                TextLayout::wrap(label, w as u32, h as u32, HorizontalAlignment::Center);
            let mut y = 0;
            layout_text.into_iter().for_each(|l| {
                sub_win.mvaddstr(y, 0, l);
                y += 1;
            });
            sub_win.delwin();
        }
        MouseCursor::OutOfBounds
    }
}
