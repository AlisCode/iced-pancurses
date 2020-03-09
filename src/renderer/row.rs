use crate::{primitive::Primitive, TerminalRenderer};
use iced_native::{row, Element, Layout, Point};

impl row::Renderer for TerminalRenderer {
    fn draw<Message>(
        &mut self,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Self::Output {
        Primitive::Group(
            children
                .iter()
                .zip(layout.children())
                .map(|(child, layout)| child.draw(self, layout, cursor_position))
                .collect(),
        )
    }
}
