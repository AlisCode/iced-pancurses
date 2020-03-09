use crate::TerminalRenderer;
use iced_native::renderer::Debugger;
use iced_native::{Layout, Color, Point};

impl Debugger for TerminalRenderer {
    fn explain<Message>(&mut self, widget: &dyn Widget<Message, Self>, layout: Layout, cursor_position: Point, color: Color) {
        let bounds = layout.bounds();
        if let Ok(sub_win) = self.window.subwin(
            bounds.height as i32,
            bounds.width as i32,
            bounds.y as i32,
            bounds.x as i32,
        ) {
            sub_win.border(0, 0, 0, 0, 0, 0, 0, 0);
            sub_win.delwin();
        }
    }
}
