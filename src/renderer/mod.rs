mod colors;
mod text;

use crate::renderer::colors::ColorRegistry;
use pancurses::{initscr, Input, Window};

pub struct PancursesRenderer {
    window: Window,
    color_registry: ColorRegistry,
}

impl Default for PancursesRenderer {
    fn default() -> Self {
        let window = initscr();
        pancurses::noecho();
        pancurses::curs_set(0);
        pancurses::start_color();
        pancurses::use_default_colors();

        let color_registry = ColorRegistry::default();

        Self {
            window,
            color_registry,
        }
    }
}

impl PancursesRenderer {
    pub fn flush(&mut self) {
        self.window.clear();
        self.window.refresh();
    }

    pub fn handle(&self) -> Option<Input> {
        self.window.getch()
    }

    pub fn size(&self) -> (u16, u16) {
        let (y, x) = self.window.get_max_yx();
        (y as u16, x as u16)
    }
}
