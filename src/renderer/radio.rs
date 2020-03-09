use crate::primitive::Primitive;
use crate::TerminalRenderer;
use iced_native::widget::radio::Renderer as RadioRenderer;
use iced_native::Rectangle;

impl RadioRenderer for TerminalRenderer {
    fn default_size(&self) -> u32 {
        1
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_selected: bool,
        _is_mouse_over: bool,
        label: Primitive,
    ) -> Primitive {
        let radiochar = if is_selected { 'x' } else { 'o' };
        Primitive::Group(vec![
            Primitive::Char(bounds.x as i32, bounds.y as i32, radiochar),
            label,
        ])
    }
}
