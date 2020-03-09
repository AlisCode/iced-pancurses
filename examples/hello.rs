use iced_native::{Color, Column, Container, Element, HorizontalAlignment, Length, Text};
use iced_pancurses::{Sandbox, TerminalRenderer};

pub struct MyState;

impl Sandbox for MyState {
    type Message = ();

    fn new() -> Self {
        MyState
    }

    fn view(&mut self) -> Element<'_, Self::Message, TerminalRenderer<std::io::Stdout>> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(
                    Text::new("Hello terminal!\nThis is a toy renderer")
                        .color(Color {
                            r: 0.,
                            g: 0.,
                            b: 1.,
                            a: 1.,
                        })
                        .width(Length::Shrink)
                        .horizontal_alignment(HorizontalAlignment::Center),
                )
                .push(Text::new("Other text").width(Length::Shrink))
                .width(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn update(&mut self, _messages: Vec<Self::Message>) {}
}

fn main() -> terminal::error::Result<()> {
    MyState::run()
}
