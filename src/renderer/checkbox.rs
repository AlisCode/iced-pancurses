use crate::primitive::Primitive;
use crate::PancursesRenderer;

use iced_native::widget::checkbox::Renderer as CheckboxRenderer;
use iced_native::Rectangle;

impl CheckboxRenderer for PancursesRenderer {
    fn default_size(&self) -> u32 {
        1
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        is_checked: bool,
        _is_mouse_over: bool,
        label: Primitive,
    ) -> Primitive {
        let boxchar = if is_checked { 'x' } else { 'o' };
        Primitive::Group(vec![
            Primitive::Char(bounds.x as i32, bounds.y as i32, boxchar),
            label,
        ])
    }
}
