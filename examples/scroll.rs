use iced_native::widget::scrollable::State;
use iced_native::{Cache, Column, Length, Row, Scrollable, Text};
use iced_pancurses::TerminalRenderer;

fn main() {
    let mut state = State::new();
    let mut renderer = TerminalRenderer::default();
    let root: Column<(), TerminalRenderer> = Column::new()
        .spacing(1)
        .push(Text::new("Hello scrolling !"))
        .push(
            Scrollable::new(&mut state).push(
                Column::new()
                    .spacing(1)
                    .push(Text::new("Scroll !"))
                    .push(Row::new().height(Length::Units(5)))
                    .push(Text::new("Scroll !"))
                    .push(Text::new("Scroll !")),
            ),
        );
    let cache = Cache::default();
    let ui = iced_native::UserInterface::build(root, cache, &mut renderer);
    loop {
        let primitives = ui.draw(&mut renderer);
        renderer.draw(primitives);
        let _event = renderer.handle();
    }
}
