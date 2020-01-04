use iced_native::widget::slider::State as SliderState;
use iced_native::{Column, Container, Element, Length, Slider, Text};
use iced_pancurses::{PancursesRenderer, Sandbox};

pub struct MyState {
    slider_state: SliderState,
    curr_val: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    ChangeVal(f32),
}

impl Sandbox for MyState {
    type Message = MyMessage;

    fn new() -> Self {
        MyState {
            slider_state: SliderState::new(),
            curr_val: 0.,
        }
    }

    fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Container::new(
            Column::new()
                .spacing(1)
                .push(
                    Text::new(&format!("Hello sliders! {:.2}", self.curr_val))
                        .width(Length::Shrink),
                )
                .push(
                    Column::new()
                        .height(Length::Units(3))
                        .push(Slider::new(
                            &mut self.slider_state,
                            0.0..=100.,
                            self.curr_val,
                            |new| MyMessage::ChangeVal(new),
                        ))
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
            MyMessage::ChangeVal(x) => self.curr_val = x,
        });
    }
}

fn main() {
    MyState::run()
}
