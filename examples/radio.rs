use iced::{Cache, Column, Element, Text, Radio};
use iced_pancurses::PancursesRenderer;

pub struct MyState {
    viewport_size: (u16, u16),
    pub selected_color: ExampleColor,
    pub cache: Cache,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExampleColor {
    White,
    Yellow,
    Blue, 
    Red
}

impl ExampleColor {
    pub fn str_rep(&self) -> &'static str {
        match self {
            ExampleColor::White => "white",
            ExampleColor::Yellow => "yellow",
            ExampleColor::Blue => "blue",
            ExampleColor::Red => "red",
        }
    }
}

impl MyState {

    pub fn view(&mut self) -> Element<MyMessage, PancursesRenderer> {
        let col = self.selected_color.str_rep();
        Column::new()
            .max_width(self.viewport_size.0)
            .max_height(self.viewport_size.1)
            .spacing(1)
            .push(Text::new("Colored text").color(col))
            .push(
                Column::new()
                .push(
                    Radio::new(
                        ExampleColor::White,
                        "White",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::White),
                    )
                )
                .push(
                    Radio::new(
                        ExampleColor::Yellow,
                        "Yellow",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Yellow),
                    )
                )
                .push(
                    Radio::new(
                        ExampleColor::Blue,
                        "Blue",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Blue),
                    )
                )
                .push(
                    Radio::new(
                        ExampleColor::Red,
                        "Red",
                        Some(self.selected_color),
                        |_| MyMessage::SelectColor(ExampleColor::Red),
                    )
                )
            )
            .into()
    }

    pub fn new(viewport_size: (u16, u16)) -> Self {
        MyState {
            viewport_size,
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