mod button;
mod checkbox;
mod debugger;
mod slider;
mod text;

use crate::colors::ColorRegistry;
use iced::input::{mouse::Button, mouse::Event as MouseEvent, ButtonState};
use iced::Event;
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
        window.keypad(true); // Set keypad mode
        pancurses::mousemask(pancurses::ALL_MOUSE_EVENTS, std::ptr::null_mut()); // Listen to all mouse events

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
    pub fn handle(&self) -> Option<Vec<Event>> {
        let input = self.window.getch();
        match input {
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    match mouse_event.bstate {
                        pancurses::BUTTON1_PRESSED => Some(move_cursor_and(
                            mouse_event.x,
                            mouse_event.y,
                            Event::Mouse(MouseEvent::Input {
                                state: ButtonState::Pressed,
                                button: Button::Left,
                            }),
                        )),
                        pancurses::BUTTON1_RELEASED => Some(move_cursor_and(
                            mouse_event.x,
                            mouse_event.y,
                            Event::Mouse(MouseEvent::Input {
                                state: ButtonState::Released,
                                button: Button::Left,
                            }),
                        )),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Gets the size of the viewport
    pub fn size(&self) -> (u16, u16) {
        let (y, x) = self.window.get_max_yx();
        (y as u16, x as u16)
    }
}

pub fn move_cursor_and(x: i32, y: i32, other: Event) -> Vec<Event> {
    vec![
        Event::Mouse(MouseEvent::CursorMoved {
            x: x as f32,
            y: y as f32,
        }),
        other,
    ]
}
