use iced_native::widget::button::State as ButtonState;
use iced_native::{Button, Column, Container, Element, Length, Text};
use iced_pancurses::{PancursesRenderer, Sandbox};

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    ClickedButton,
}

pub struct MyState {
    button_state: ButtonState,
    clicked: u32,
}

impl Sandbox for MyState {
    type Message = MyMessage;

    fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(
                    Text::new(&format!("Button clicked {} times", self.clicked))
                        .width(Length::Shrink),
                )
                .push(
                    Button::new(&mut self.button_state, Text::new("Hello!"))
                        .padding(1)
                        .on_press(MyMessage::ClickedButton),
                )
                .width(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn new() -> Self {
        MyState {
            button_state: ButtonState::new(),
            clicked: 0,
        }
    }

    fn update(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::ClickedButton => self.clicked += 1,
        });
    }
}

fn main() {
    MyState::run()
}
