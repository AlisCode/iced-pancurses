use iced_native::widget::text_input::State;
use iced_native::{Column, Container, Element, Length, Text, TextInput};
use iced_pancurses::{PancursesRenderer, Sandbox};

#[derive(Debug, Clone)]
pub enum MyMessage {
    OnTextInput(String),
}

struct MyState {
    text_input_state: State,
    curr_value: String,
}

impl Sandbox for MyState {
    type Message = MyMessage;

    fn new() -> Self {
        MyState {
            text_input_state: State::new(),
            curr_value: "".into(),
        }
    }

    fn view(&mut self) -> Element<'_, MyMessage, PancursesRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(Text::new("Hello TextInput!").width(Length::Shrink))
                .push(
                    TextInput::new(
                        &mut self.text_input_state,
                        "Type something",
                        &self.curr_value,
                        MyMessage::OnTextInput,
                    )
                    .width(Length::Units(20)),
                )
                .width(Length::Shrink),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn update(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::OnTextInput(new) => self.curr_value = new,
        })
    }
}

fn main() {
    MyState::run()
}
