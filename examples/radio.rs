use iced_native::{Color, Column, Container, Element, Length, Radio, Text};
use iced_pancurses::{TerminalRenderer, Sandbox};

struct MyState {
    selected_color: ExampleColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleColor {
    White,
    Yellow,
    Blue,
    Red,
}

impl ExampleColor {
    pub fn as_iced_color(&self) -> Color {
        match self {
            ExampleColor::White => Color::WHITE,
            ExampleColor::Yellow => Color {
                r: 1.,
                g: 1.,
                b: 0.,
                a: 1.,
            },
            ExampleColor::Blue => Color {
                r: 0.,
                g: 0.,
                b: 1.,
                a: 1.,
            },
            ExampleColor::Red => Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        }
    }
}

impl Sandbox for MyState {
    type Message = MyMessage;

    fn view(&mut self) -> Element<'_, MyMessage, TerminalRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(
                    Text::new("Colored text")
                        .width(Length::Shrink)
                        .color(self.selected_color.as_iced_color()),
                )
                .push(
                    Column::new()
                        .push(Radio::new(
                            ExampleColor::White,
                            "White",
                            Some(self.selected_color),
                            |_| MyMessage::SelectColor(ExampleColor::White),
                        ))
                        .push(Radio::new(
                            ExampleColor::Yellow,
                            "Yellow",
                            Some(self.selected_color),
                            |_| MyMessage::SelectColor(ExampleColor::Yellow),
                        ))
                        .push(Radio::new(
                            ExampleColor::Blue,
                            "Blue",
                            Some(self.selected_color),
                            |_| MyMessage::SelectColor(ExampleColor::Blue),
                        ))
                        .push(Radio::new(
                            ExampleColor::Red,
                            "Red",
                            Some(self.selected_color),
                            |_| MyMessage::SelectColor(ExampleColor::Red),
                        )),
                ),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn new() -> Self {
        MyState {
            selected_color: ExampleColor::White,
        }
    }

    fn update(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::SelectColor(c) => self.selected_color = c,
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    SelectColor(ExampleColor),
}

fn main() {
    MyState::run()
}
