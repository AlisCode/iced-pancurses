use iced_native::widget::text_input::State;
use iced_native::{Cache, Color, Column, Element, HorizontalAlignment, Text, TextInput};
use iced_pancurses::PancursesRenderer;

#[derive(Debug, Clone)]
pub enum MyMessage {
    OnTextInput(String),
}

pub struct MyState {
    text_input_state: State,
    curr_value: String,
    pub cache: Cache,
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Column::new()
            .spacing(1)
            .push(Text::new("Hello TextInput!"))
            .push(TextInput::new(
                &mut self.text_input_state,
                "Type something",
                &self.curr_value,
                MyMessage::OnTextInput,
            ))
            .into()
    }

    pub fn new() -> Self {
        MyState {
            text_input_state: State::new(),
            curr_value: "".into(),
            cache: Default::default(),
        }
    }

    pub fn handle_messages(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::OnTextInput(new) => self.curr_value = new,
        })
    }
}

fn main() {
    let mut renderer = PancursesRenderer::default();
    let mut state = MyState::new();
    loop {
        let cache = state.cache.clone();
        let root = state.view();
        let mut ui = iced_native::UserInterface::build(root, cache, &mut renderer);
        let primitives = ui.draw(&mut renderer);
        renderer.draw(primitives);
        if let Some(events) = renderer.handle() {
            let messages = ui.update(&renderer, events.into_iter());
            drop(ui);
            state.handle_messages(messages);
        }
        renderer.flush();
    }
}
