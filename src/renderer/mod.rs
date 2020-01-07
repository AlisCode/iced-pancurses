mod button;
mod checkbox;
mod column;
//mod debugger;
mod image;
mod radio;
mod row;
mod scrollable;
mod slider;
mod space;
mod text;
//mod text_input;

use crate::colors::{ColorRegistry, PancursesColor};
use crate::primitive::Primitive;
use iced_native::input::{
    keyboard, keyboard::KeyCode, mouse::Button, mouse::Event as MouseEvent, ButtonState,
};
use iced_native::layout::Limits;
use iced_native::{Event, Renderer};
use pancurses::{initscr, Input, Window};

/// Pancurses Renderer implementation for iced
pub struct PancursesRenderer {
    /// Pancurses window to use to print UI elements
    window: Window,
    /// The ColorRegistry is the place to store pancurses color pairs indices
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
        // Set keypad mode; necessary for correct input handling
        window.keypad(true);

        // Listen to all mouse events
        pancurses::mousemask(pancurses::ALL_MOUSE_EVENTS, std::ptr::null_mut());
        Self {
            window,
            color_registry: Default::default(),
        }
    }
}

impl Renderer for PancursesRenderer {
    type Output = Primitive;

    fn layout<'a, Message>(
        &mut self,
        element: &iced_native::Element<'a, Message, Self>,
    ) -> iced_native::layout::Node {
        let limits = Limits::NONE
            .max_width(self.window.get_max_x() as u32)
            .max_height(self.window.get_max_y() as u32);
        element.layout(self, &limits)
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
            Some(Input::Character(c)) => {
                Some(vec![Event::Keyboard(keyboard::Event::CharacterReceived(c))])
            }
            Some(Input::KeyBackspace) => Some(vec![
                Event::Keyboard(keyboard::Event::Input {
                    state: ButtonState::Pressed,
                    key_code: KeyCode::Backspace,
                    modifiers: keyboard::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                }),
                Event::Keyboard(keyboard::Event::Input {
                    state: ButtonState::Released,
                    key_code: KeyCode::Backspace,
                    modifiers: keyboard::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                }),
            ]),
            Some(Input::KeyEnter) => Some(vec![
                Event::Keyboard(keyboard::Event::Input {
                    state: ButtonState::Pressed,
                    key_code: KeyCode::Enter,
                    modifiers: keyboard::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                }),
                Event::Keyboard(keyboard::Event::Input {
                    state: ButtonState::Released,
                    key_code: KeyCode::Enter,
                    modifiers: keyboard::ModifiersState {
                        shift: false,
                        control: false,
                        alt: false,
                        logo: false,
                    },
                }),
            ]),
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = pancurses::getmouse() {
                    match mouse_event.bstate {
                        pancurses::BUTTON1_PRESSED => Some(move_cursor_and(
                            mouse_event.x,
                            mouse_event.y,
                            vec![Event::Mouse(MouseEvent::Input {
                                state: ButtonState::Pressed,
                                button: Button::Left,
                            })],
                        )),
                        pancurses::BUTTON1_RELEASED => Some(move_cursor_and(
                            mouse_event.x,
                            mouse_event.y,
                            vec![Event::Mouse(MouseEvent::Input {
                                state: ButtonState::Released,
                                button: Button::Left,
                            })],
                        )),
                        pancurses::BUTTON1_CLICKED => Some(move_cursor_and(
                            mouse_event.x,
                            mouse_event.y,
                            vec![
                                Event::Mouse(MouseEvent::Input {
                                    state: ButtonState::Pressed,
                                    button: Button::Left,
                                }),
                                Event::Mouse(MouseEvent::Input {
                                    state: ButtonState::Released,
                                    button: Button::Left,
                                }),
                            ],
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

    // Sets nodelay to true in order to provide async actions
    pub fn nodelay(self) -> Self {
        self.window.nodelay(true);
        self
    }

    /// Draws a given primitive onto the window
    pub fn draw(&mut self, primitive: Primitive) {
        match primitive {
            Primitive::Group(prims) => prims.into_iter().for_each(|p| self.draw(p)),
            Primitive::Text(texts, bounds, color) => {
                let col = crate::colors::get_closest_color(color);
                let col_idx = self.color_registry.get_idx(PancursesColor::new(col, -1));
                self.window
                    .attrset(pancurses::COLOR_PAIR((col_idx as u32).into()));
                let mut y = 0;
                texts.into_iter().for_each(|l| {
                    self.window.mv(bounds.y as i32 + y as i32, bounds.x as i32);
                    self.window.addstr(l);
                    y += 1;
                });
            }
            Primitive::BoxDisplay(bounds) => {
                let col_idx = self
                    .color_registry
                    .get_idx(PancursesColor::new(pancurses::COLOR_WHITE, -1));
                self.window
                    .attrset(pancurses::COLOR_PAIR((col_idx as u32).into()));
                let x = bounds.x as i32;
                let y = bounds.y as i32;
                let w = bounds.width as i32;
                let h = bounds.height as i32;
                if let Ok(sub_win) = self.window.subwin(h, w, y, x) {
                    sub_win.border(0, 0, 0, 0, 0, 0, 0, 0);
                    sub_win.delwin();
                }
            }
            Primitive::Char(x, y, boxchar) => {
                let col_idx = self
                    .color_registry
                    .get_idx(PancursesColor::new(pancurses::COLOR_WHITE, -1));
                self.window
                    .attrset(pancurses::COLOR_PAIR((col_idx as u32).into()));
                self.window.mv(y, x);
                self.window.addch(boxchar);
            }
            _ => (),
        }
    }

    /// Gets the current size of the terminal root window
    pub fn size(&self) -> (u16, u16) {
        let yx = self.window.get_max_yx();
        (yx.1 as u16, yx.0 as u16)
    }
}

pub fn move_cursor_and(x: i32, y: i32, other: Vec<Event>) -> Vec<Event> {
    vec![Event::Mouse(MouseEvent::CursorMoved {
        x: x as f32,
        y: y as f32,
    })]
    .into_iter()
    .chain(other.into_iter())
    .collect()
}
