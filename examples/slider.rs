use iced_native::widget::slider::State as SliderState;
use iced_native::{Cache, Column, Element, Length, Slider, Text};
use iced_pancurses::PancursesRenderer;

pub struct MyState {
    slider_state: SliderState,
    curr_val: f32,
    pub cache: Cache,
}

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    ChangeVal(f32),
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Column::new()
            .spacing(1)
            .push(Text::new(&format!("Hello sliders! {:.2}", self.curr_val)))
            .push(Column::new().height(Length::Units(3)).push(Slider::new(
                &mut self.slider_state,
                0.0..=100.,
                self.curr_val,
                |new| MyMessage::ChangeVal(new),
            )))
            .into()
    }

    pub fn new() -> Self {
        MyState {
            slider_state: SliderState::new(),
            curr_val: 0.,
            cache: Cache::default(),
        }
    }

    pub fn handle_messages(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::ChangeVal(x) => self.curr_val = x,
        });
    }
}

fn main() {
    let mut renderer = PancursesRenderer::default();
    let mut state = MyState::new();
    loop {
        let cache = state.cache.clone();
        let root = state.view();
        let mut ui = iced_native::UserInterface::build(root, cache, &mut renderer);
        let prim = ui.draw(&mut renderer);
        renderer.draw(prim);
        if let Some(events) = renderer.handle() {
            let messages = ui.update(&mut renderer, events.into_iter());
            drop(ui);
            state.handle_messages(messages);
        }
        renderer.flush();
    }
}
