use iced_native::{Cache, Checkbox, Column, Element, Text};
use iced_pancurses::PancursesRenderer;

pub struct MyState {
    viewport_size: (u32, u32),
    cache: Cache,
    checked_test_checkbox: bool,
    checked_test_other_checkbox: bool,
}

pub enum MyMessage {
    ToggleTestCheckbox,
    ToggleOtherCheckbox,
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        let text = match (self.checked_test_checkbox, self.checked_test_other_checkbox) {
            (true, true) => "Both checked!",
            (false, true) | (true, false) => "Only one checked",
            _ => "Zero checked",
        };
        Column::new()
            .spacing(1)
            .push(Text::new(text))
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
            .into()
    }

    pub fn new(viewport_size: (u32, u32)) -> Self {
        MyState {
            viewport_size,
            cache: Default::default(),
            checked_test_checkbox: false,
            checked_test_other_checkbox: false,
        }
    }

    pub fn handle_messages(&mut self, messages: Vec<MyMessage>) {
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
    let mut renderer = PancursesRenderer::default();
    let (view_y, view_x) = renderer.size();
    let mut state = MyState::new((view_x as u32, view_y as u32));
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
