use crate::PancursesRenderer;
use iced::renderer::Debugger;
use iced::Layout;

impl Debugger for PancursesRenderer {
    type Color = ();

    fn explain(&mut self, layout: &Layout, _color: ()) {
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
