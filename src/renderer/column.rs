use crate::{primitive::Primitive, TerminalRenderer};
use iced_native::{column, Element, Layout, Point};

impl<W: std::io::Write> column::Renderer for TerminalRenderer<W> {
    fn draw<Message>(
        &mut self,
        content: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        Primitive::Group(
            content
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| child.draw(self, layout, cursor_position))
                .collect(),
        )
    }
}
