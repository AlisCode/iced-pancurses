//mod button;
//mod checkbox;
mod column;
//mod debugger;
//mod image;
//mod radio;
//mod row;
//mod scrollable;
//mod slider;
//mod space;
mod text;
//mod text_input;

use crate::primitive::Primitive;
use core::time::Duration;
use iced_native::input::{
    keyboard::KeyCode, mouse::Button, mouse::Event as MouseEvent, ButtonState,
};
use iced_native::layout::Limits;
use iced_native::{Event, Renderer};
use std::io::Write;
use terminal::{Action, Attribute, Clear, Retrieved, Terminal, Value};

/// Terminal Renderer implementation for Iced
///
/// This is a both the shell and the renderer, it is the basic building block of your Iced
/// Application
pub struct TerminalRenderer<W: Write> {
    /// Terminal window to use to print UI elements
    terminal: Terminal<W>,
    /// Terminal refresh delay, allows any terminal app to be non-blocking
    ///
    /// * Some(Duration) will set the target FPS of the Application
    /// * None means the application is polling user event
    refresh_delay: Option<Duration>,
}

impl Default for TerminalRenderer<std::io::Stdout> {
    /// Default config for a Pancurses renderer
    fn default() -> Self {
        match TerminalRenderer::<std::io::Stdout>::new() {
            Ok(tr) => tr,
            Err(e) => panic!("Error creating the terminal context: {}", e),
        }
    }
}

impl TerminalRenderer<std::io::Stdout> {
    pub fn new() -> terminal::error::Result<Self> {
        let mut renderer = TerminalRenderer {
            terminal: terminal::stdout(),
            refresh_delay: None,
        };

        renderer.setup_terminal()?;

        Ok(renderer)
    }
}

impl<W: Write> Renderer for TerminalRenderer<W> {
    type Output = Primitive;

    fn layout<'a, Message>(
        &mut self,
        element: &iced_native::Element<'a, Message, Self>,
    ) -> iced_native::layout::Node {
        let abc = self
            .terminal
            .get(Value::TerminalSize)
            .expect("Failed to read terminal size");
        match abc {
            Retrieved::TerminalSize(x, y) => {
                let limits = Limits::NONE.max_width(x as u32).max_height(y as u32);
                element.layout(self, &limits)
            }
            _ => unreachable!(),
        }
    }
}

impl<W: Write> TerminalRenderer<W> {
    /// Polls event from the terminal window
    pub fn handle(&self) -> terminal::error::Result<Option<Event>> {
        let input = self.terminal.get(Value::Event(None))?;
        match input {
            Retrieved::Event(Some(terminal::Event::Key(ke))) => Ok(None),
            Retrieved::Event(Some(terminal::Event::Key(KeyEvent {}))) => Ok(None),
            /*
            (Input::Character(c)) => {
                Some(vec![Event::Keyboard(keyboard::Event::CharacterReceived(c))])
            }::Event()
            Some(Input::KeyResize) => {
                self.flush();
                None
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
            */
            _ => Ok(None),
        }
    }

    pub fn clear(&mut self) -> terminal::error::Result<()> {
        self.terminal.act(Action::ClearTerminal(Clear::All))
    }

    pub fn setup_terminal(&mut self) -> terminal::error::Result<()> {
        // Resets terminal state
        self.terminal.act(Action::SetAttribute(Attribute::Reset))?;
        self.terminal.act(Action::ClearTerminal(Clear::All))?;

        // Sets up various data for correct terminal processing
        self.terminal.act(Action::ResetColor)?;
        self.terminal.act(Action::HideCursor)?;
        self.terminal.act(Action::DisableBlinking)?;
        self.terminal.act(Action::EnableRawMode)?;
        self.terminal.act(Action::EnableMouseCapture)
    }

    // Sets nodelay to true in order to provide async actions
    pub fn target_fps(mut self, fps: u64) -> Self {
        self.refresh_delay = Some(Duration::from_millis(1000 / fps));
        self
    }

    /// Draws a given primitive onto the window
    pub fn draw(&mut self, primitive: Primitive) -> terminal::error::Result<()> {
        match primitive {
            Primitive::Group(prims) => prims
                .into_iter()
                .map(|p| self.draw(p))
                .collect::<terminal::error::Result<()>>(),
            Primitive::Text(texts, bounds, color) => {
                //let col = crate::colors::get_closest_color(color);
                //let col_idx = self.color_registry.get_idx(PancursesColor::new(col, -1));
                //self.window
                //    .attrset(pancurses::COLOR_PAIR((col_idx as u32).into()));
                let mut y = 0;
                texts
                    .into_iter()
                    .map(|l| {
                        self.terminal.act(Action::MoveCursorTo(
                            bounds.x as u16,
                            bounds.y as u16 + y as u16,
                        ))?;
                        self.terminal.write(l.as_bytes())?;
                        y += 1;
                        Ok(())
                    })
                    .collect::<terminal::error::Result<()>>()
            }
            Primitive::BoxDisplay(bounds) => {
                Ok(())
                /*
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
                */
            }
            Primitive::Char(x, y, c) => {
                //let col_idx = self
                //    .color_registry
                //    .get_idx(PancursesColor::new(pancurses::COLOR_WHITE, -1));
                //self.window
                //    .attrset(pancurses::COLOR_PAIR((col_idx as u32).into()));
                self.terminal
                    .act(Action::MoveCursorTo(x as u16, y as u16))?;
                self.terminal.write(format!("{}", c).as_bytes())?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Gets the current size of the terminal root window
    pub fn size(&self) -> (u16, u16) {
        match self
            .terminal
            .get(Value::TerminalSize)
            .expect("Failed to get terminal size")
        {
            Retrieved::TerminalSize(x, y) => (x, y),
            _ => unreachable!(),
        }
    }
}
