use iced::{Column, Cache, Text};
use iced_pancurses::renderer::PancursesRenderer;

fn main() {
    let mut renderer = PancursesRenderer::default();
    let (view_y, view_x) = renderer.size();
    let root: Column<(), PancursesRenderer> = 
        Column::new()
        .width(view_x)
        .height(view_y)
        .spacing(1)
        .push(Text::new("Hello pancurses!\nThis is a toy renderer").color("blue"))
        .push(Text::new("Other text").color("red"));
    let cache = Cache::default(); 
    let ui = iced::UserInterface::build(root, cache, &renderer);  

    ui.draw(&mut renderer);
    loop {
        let _event = renderer.handle();
        renderer.flush();
        ui.draw(&mut renderer);
    }
}