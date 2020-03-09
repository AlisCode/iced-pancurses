use iced_native::{Checkbox, Column, Container, Element, Length, Text};
use iced_pancurses::{TerminalRenderer, Sandbox};

pub struct MyState {
    checked_test_checkbox: bool,
    checked_test_other_checkbox: bool,
}

#[derive(Debug, Clone)]
pub enum MyMessage {
    ToggleTestCheckbox,
    ToggleOtherCheckbox,
}

impl Sandbox for MyState {
    type Message = MyMessage;

    fn view(&mut self) -> Element<MyMessage, TerminalRenderer> {
        let text = match (self.checked_test_checkbox, self.checked_test_other_checkbox) {
            (true, true) => "Both checked!",
            (false, true) | (true, false) => "Only one checked",
            _ => "Zero checked",
        };
        Container::new(
            Column::new()
                .spacing(1)
                .push(Text::new(text).width(Length::Shrink))
                .push(Checkbox::new(
                    self.checked_test_checkbox,
                    "Test checkbox",
                    |_| MyMessage::ToggleTestCheckbox,
                ))
                .push(Checkbox::new(
                    self.checked_test_other_checkbox,
                    "Test other checkbox",
                    |_| MyMessage::ToggleOtherCheckbox,
                ))
                .width(Length::Shrink),
        )
        .center_x()
        .center_y()
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn new() -> Self {
        MyState {
            checked_test_checkbox: false,
            checked_test_other_checkbox: false,
        }
    }

    fn update(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|msg| match msg {
            MyMessage::ToggleTestCheckbox => {
                self.checked_test_checkbox = !self.checked_test_checkbox
            }
            MyMessage::ToggleOtherCheckbox => {
                self.checked_test_other_checkbox = !self.checked_test_other_checkbox
            }
        })
    }
}
fn main() {
    MyState::run()
}
