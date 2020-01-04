use iced_native::{Color, Column, Container, Element, HorizontalAlignment, Length, Text};
use iced_pancurses::{PancursesRenderer, Sandbox};

pub struct MyState;

impl Sandbox for MyState {
    type Message = ();

    fn new() -> Self {
        MyState
    }

    fn view(&mut self) -> Element<'_, Self::Message, PancursesRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(
                    Text::new("Hello pancurses!\nThis is a toy renderer")
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

fn main() {
    MyState::run()
}
