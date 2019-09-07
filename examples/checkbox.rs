use iced::{Cache, Column, Checkbox, Text};
use iced_pancurses::PancursesRenderer;

fn main() {
    let mut renderer = PancursesRenderer::default();
    let (view_y, view_x) = renderer.size();
    let root: Column<(), PancursesRenderer> = Column::new()
        .width(view_x)
        .height(view_y)
        .spacing(1)
        .push(Checkbox::new(
            false,
            "Test checkbox",
            |_| { }
        ));
    let cache = Cache::default();
    let ui = iced::UserInterface::build(root, cache, &renderer);

    ui.draw(&mut renderer);
    loop {
        let _event = renderer.handle();
        renderer.flush();
        ui.draw(&mut renderer);
    }
}
