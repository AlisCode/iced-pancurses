use iced::{Cache, Column, Element, Text, Slider};
use iced::widget::slider::{State as SliderState};
use iced_pancurses::PancursesRenderer;

pub struct MyState {
    viewport_size: (u16, u16),
    slider_state: SliderState,
    curr_val: f32,
    pub cache: Cache,
}

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    ChangeVal(f32)
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Column::new()
            .max_width(self.viewport_size.0)
            .max_height(self.viewport_size.1)
            .spacing(1)
            .push(Text::new(&format!("Hello sliders! {:.2}", self.curr_val)).width(25))
            .push(
                Column::new().height(3).push(
                    Slider::new(&mut self.slider_state, 0.0..=100., self.curr_val, |new| MyMessage::ChangeVal(new))
                ),
            )
            .into()
    }

    pub fn new(viewport_size: (u16, u16)) -> Self {
        MyState {
            viewport_size,
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
    let (view_y, view_x) = renderer.size();
    let mut state = MyState::new((view_x, view_y));
    loop {
        let cache = state.cache.clone();
        let root = state.view();
        let mut ui = iced::UserInterface::build(root, cache, &renderer);
        ui.draw(&mut renderer);
        if let Some(events) = renderer.handle() {
            let messages = ui.update(events.into_iter());
            drop(ui);
            state.handle_messages(messages);
        }
        renderer.flush();
    }
}
