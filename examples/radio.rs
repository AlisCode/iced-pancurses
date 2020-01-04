use iced_native::{Cache, Color, Column, Element, Radio, Text};
use iced_pancurses::PancursesRenderer;

pub struct MyState {
    pub selected_color: ExampleColor,
    pub cache: Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleColor {
    White,
    Yellow,
    Blue,
    Red,
}

impl ExampleColor {
    pub fn as_iced_color(&self) -> Color {
        match self {
            ExampleColor::White => Color::WHITE,
            ExampleColor::Yellow => Color {
                r: 1.,
                g: 1.,
                b: 0.,
                a: 1.,
            },
            ExampleColor::Blue => Color {
                r: 0.,
                g: 0.,
                b: 1.,
                a: 1.,
            },
            ExampleColor::Red => Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        }
    }
}

impl MyState {
    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        Column::new()
            .spacing(1)
            .push(Text::new("Colored text").color(self.selected_color.as_iced_color()))
            .push(
                Column::new()
                    .push(Radio::new(
                        ExampleColor::White,
                        "White",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::White),
                    ))
                    .push(Radio::new(
                        ExampleColor::Yellow,
                        "Yellow",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Yellow),
                    ))
                    .push(Radio::new(
                        ExampleColor::Blue,
                        "Blue",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Blue),
                    ))
                    .push(Radio::new(
                        ExampleColor::Red,
                        "Red",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Red),
                    )),
            )
            .into()
    }

    pub fn new() -> Self {
        MyState {
            cache: Cache::default(),
            selected_color: ExampleColor::White,
        }
    }

    pub fn handle_messages(&mut self, messages: Vec<MyMessage>) {
        messages.into_iter().for_each(|m| match m {
            MyMessage::SelectColor(c) => self.selected_color = c,
        });
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MyMessage {
    SelectColor(ExampleColor),
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
