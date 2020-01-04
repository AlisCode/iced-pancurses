use iced_native::widget::scrollable::State;
use iced_native::{Cache, Column, Scrollable, Text, Row, Length};
use iced_pancurses::PancursesRenderer;

fn main() {
    let mut state = State::new();
    let mut renderer = PancursesRenderer::default();
    let root: Column<(), PancursesRenderer> = Column::new()
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
