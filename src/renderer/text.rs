use crate::renderer::PancursesRenderer;

use iced::widget::text::{HorizontalAlignment, Renderer as TextRenderer, VerticalAlignment};
use iced::{Node, Size, Style};

impl TextRenderer<&str> for PancursesRenderer {
    /// TODO: actually implement text layouting
    fn node(&self, style: Style, content: &str, _size: Option<u16>) -> Node {
        let lines = content.lines().count();
        let max_len = content
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(1);

        Node::with_measure(style, move |_bounds| Size {
            width: max_len as f32,
            height: lines as f32,
        })
    }

    /// TODO: wrap text
    fn draw(
        &mut self,
        bounds: iced::Rectangle,
        content: &str,
        _size: Option<u16>,
        color: Option<&str>,
        _horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) {
        self.window.mv(bounds.y as i32, bounds.x as i32);
        if let Some(col) = color {
            let col = self.color_registry[col];
            self.window.attrset(pancurses::COLOR_PAIR(col.into()));
        }
        self.window.refresh();
        self.window.addstr(content);
    }
}
