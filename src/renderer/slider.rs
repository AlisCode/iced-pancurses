use crate::primitive::Primitive;
use crate::PancursesRenderer;

use iced_native::widget::slider;
use iced_native::{Point, Rectangle};

use std::ops::RangeInclusive;

impl slider::Renderer for PancursesRenderer {
    fn height(&self) -> u32 {
        1
    }

    fn draw(
        &mut self,
        bounds: Rectangle,
        _cursor_position: Point,
        range: RangeInclusive<f32>,
        value: f32,
        _is_dragging: bool,
    ) -> Primitive {
        let (range_start, range_end) = range.into_inner();
        let marker_offset =
            bounds.width * ((value - range_start) / (range_end - range_start).max(1.0));

        Primitive::Group(vec![
            Primitive::BoxDisplay(bounds),
            Primitive::Char(bounds.x as i32 + marker_offset as i32, bounds.y as i32, 'x'),
        ])
    }
}
