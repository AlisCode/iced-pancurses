mod text;

use crate::colors::ColorRegistry;
use pancurses::{initscr, Input, Window};

/// Pancurses Renderer implementation for iced
pub struct PancursesRenderer {
    /// Pancurses window to use to print UI elements
    window: Window,
    /// Color registry in use by the backend
    color_registry: ColorRegistry,
}

impl Default for PancursesRenderer {
    /// Default config for a Pancurses renderer
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
    /// Clears the output of the renderer
    pub fn flush(&mut self) {
        self.window.clear();
        self.window.refresh();
    }

    /// Polls event from the pancurses window
    pub fn handle(&self) -> Option<Input> {
        self.window.getch()
    }

    /// Gets the size of the viewport
    pub fn size(&self) -> (u16, u16) {
        let (y, x) = self.window.get_max_yx();
        (y as u16, x as u16)
    }
}
