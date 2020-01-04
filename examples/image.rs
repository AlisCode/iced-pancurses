use iced_native::{Cache, Column, Image, Text};
use iced_pancurses::PancursesRenderer;

fn main() {
    let mut renderer = PancursesRenderer::default();
    let root: Column<(), PancursesRenderer> = Column::new()
        .spacing(1)
        .push(Text::new("Hello image !"))
        .push(Image::new("resources/ferris.png"));
    let cache = Cache::default();
    let ui = iced_native::UserInterface::build(root, cache, &mut renderer);
    loop {
        let primitives = ui.draw(&mut renderer);
        renderer.draw(primitives);
        let _event = renderer.handle();
    }
}
