use iced_native::{Cache, Color, Column, HorizontalAlignment, Text};
use iced_pancurses::PancursesRenderer;

fn main() {
    let mut renderer = PancursesRenderer::default();
    let (view_y, view_x) = renderer.size();
    let root: Column<(), PancursesRenderer> = Column::new()
        .max_width(view_x)
        .max_height(view_y)
        .spacing(1)
        .push(
            Text::new("Hello pancurses!\nThis is a toy renderer")
                .color(Color {
                    r: 0.,
                    g: 0.,
                    b: 1.,
                    a: 1.,
                })
                .horizontal_alignment(HorizontalAlignment::Center),
        )
        .push(Text::new("Other text"));
    let cache = Cache::default();
    let ui = iced_native::UserInterface::build(root, cache, &mut renderer);
    loop {
        //renderer.flush();
        let primitives = ui.draw(&mut renderer);
        renderer.draw(primitives);
        let _event = renderer.handle();
    }
}
