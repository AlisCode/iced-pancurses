use iced_native::{Color, Rectangle};

pub enum Primitive {
    Char(i32, i32, char),
    BoxDisplay(Rectangle),
    Empty,
    Group(Vec<Primitive>),
    Text(Vec<String>, Rectangle, Color),
}

impl Primitive {
    pub fn with_offset(self, offset: i32) -> Primitive {
        match self {
            Primitive::BoxDisplay(mut bounds) => {
                bounds.y -= offset as f32;
                Primitive::BoxDisplay(bounds)
            }
            Primitive::Char(x, y, content) => Primitive::Char(x, y - offset, content),
            Primitive::Text(content, mut bounds, color) => {
                bounds.y -= offset as f32;
                Primitive::Text(content, bounds, color)
            }
            Primitive::Group(primitives) => Primitive::Group(
                primitives
                    .into_iter()
                    .map(|p| p.with_offset(offset))
                    .collect(),
            ),
            _ => self,
        }
    }
}
