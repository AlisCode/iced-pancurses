use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::image;
use iced_native::{Font, HorizontalAlignment, Layout, VerticalAlignment};

// TODO: Properly support image rendering using w3img backend or fallback pixel buffer

impl image::Renderer for TerminalRenderer {
    fn dimensions(&self, _path: &image::Handle) -> (u32, u32) {
        (15, 5)
    }

    fn draw(&mut self, _path: image::Handle, layout: Layout) -> Primitive {
        let mut bounds = layout.bounds();
        bounds.x += 1.;
        bounds.y += 1.;
        let prim_text = <Self as iced_native::widget::text::Renderer>::draw(
            self,
            bounds,
            "Unsupported",
            1,
            Font::Default,
            None,
            HorizontalAlignment::Center,
            VerticalAlignment::Center,
        );
        Primitive::Group(vec![Primitive::BoxDisplay(layout.bounds()), prim_text])
    }
}
