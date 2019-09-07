use crate::renderer::PancursesRenderer;

use iced::widget::text::{HorizontalAlignment, Renderer as TextRenderer, VerticalAlignment};
use iced::{Node, Number, Size, Style};

impl TextRenderer<&str> for PancursesRenderer {
    fn node(&self, style: Style, content: &str, _size: Option<u16>) -> Node {
        let max_len = content
            .lines()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(1);
        let content: String = content.into();
        Node::with_measure(style, move |bounds| {
            let max_x = match bounds.width {
                Number::Defined(x) => x as u32,
                _ => max_len as u32,
            };
            let max_y = match bounds.height {
                Number::Defined(x) => x as u32,
                _ => u32::max_value(),
            };
            let layout = TextLayout::compute_layout(&content, max_x, max_y);
            Size {
                width: layout.0 as f32,
                height: layout.1 as f32,
            }
        })
    }

    fn draw(
        &mut self,
        bounds: iced::Rectangle,
        content: &str,
        _size: Option<u16>,
        color: Option<&str>,
        horizontal_alignment: HorizontalAlignment,
        _vertical_alignment: VerticalAlignment,
    ) {
        let wrapped_content = TextLayout::wrap(
            content,
            bounds.width as u32,
            bounds.height as u32,
            horizontal_alignment,
        );
        if let Some(col) = color {
            let col = self.color_registry[col];
            self.window.attrset(pancurses::COLOR_PAIR(col.into()));
        } else {
            let col = self.color_registry["white"];
            self.window.attrset(pancurses::COLOR_PAIR(col.into()));
        }
        self.window.refresh();

        let mut y = 0;
        wrapped_content.into_iter().for_each(|l| {
            self.window.mv(bounds.y as i32 + y as i32, bounds.x as i32);
            self.window.addstr(l);
            y += 1;
        });
    }
}

pub struct TextLayout;

impl TextLayout {
    /// Computes a correct layout size. This is the minimum size that the text component has to
    /// take in order to be displayed correctly.
    /// Wraps the text if it is bigger than the bounds.
    pub fn compute_layout(content: &str, max_x: u32, max_y: u32) -> (u32, u32) {
        // max_len should be length of the longest line in the content
        let mut max_len = 0;

        // Computes the number of lines after they've been wrapped
        let lines: u32 = content
            .lines()
            .map(|l| {
                let chars = l.chars().count() as u32;
                max_len = max_len.max(chars);
                let offset = if chars % max_x == 0 { 0 } else { 1 };
                (chars / max_x) + offset
            })
            .sum();
        (max_len.min(max_x), lines.min(max_y))
    }

    /// Compute lines as they should be displayed on the screen, given :
    /// * The bounds of the text box (max_x, max_y)
    /// * The Horizontal Alignement of a text
    pub fn wrap(content: &str, max_x: u32, max_y: u32, align: HorizontalAlignment) -> Vec<String> {
        let (wrapped_x, _) = TextLayout::compute_layout(content, max_x, max_y);
        content
            .lines()
            .flat_map(|l| {
                let len = l.chars().count() as u32;
                if len > wrapped_x {
                    l.as_bytes()
                        .chunks(wrapped_x as usize)
                        .map(|bytes| String::from_utf8(bytes.to_vec()).unwrap())
                        .collect()
                } else {
                    let diff = wrapped_x - len;
                    match align {
                        HorizontalAlignment::Left => {
                            let padding: String = (0..diff).map(|_| ' ').collect();
                            vec![format!("{}{}", l, padding)]
                        }
                        HorizontalAlignment::Center => {
                            let pad = diff / 2;
                            let padding: String = (0..pad).map(|_| ' ').collect();
                            let offset = if diff % 2 == 0 { "" } else { " " };
                            vec![format!("{}{}{}{}", offset, padding, l, padding)]
                        }
                        HorizontalAlignment::Right => {
                            let padding: String = (0..diff).map(|_| ' ').collect();
                            vec![format!("{}{}", padding, l)]
                        }
                    }
                }
            })
            .collect()
    }
}

#[cfg(test)]
pub mod tests {

    use super::TextLayout;
    use iced::widget::text::HorizontalAlignment;

    #[test]
    pub fn text_layout_compute_should_work() {
        let content = "First line\ntest!";
        // This particular text should look like this in a terminal:
        //
        // First line
        // test!
        //
        // This means that the size it should take on a (10, 2) or bigger is always (10, 2)
        assert_eq!(TextLayout::compute_layout(content, 10, 2), (10, 2));
        assert_eq!(TextLayout::compute_layout(content, 15, 3), (10, 2));
    }

    #[test]
    pub fn text_layout_compute_should_wrap() {
        let content = "First line\ntest!";
        // Lets test the behaviour on smaller layout, and make the text wrap.
        //
        // On a (5, 10) box, the text should wrap as follows:
        //
        // First
        // line
        // test!
        assert_eq!(TextLayout::compute_layout(content, 5, 10), (5, 3));

        // On a (4, 10) box, the text should wrap as follows:
        //
        // Firs
        // t li
        // ne
        // test
        // !
        assert_eq!(TextLayout::compute_layout(content, 4, 10), (4, 5));
    }

    #[test]
    pub fn text_layout_wrap_should_work() {
        let content = "First line\ntest!";

        // Lets try normal layoung with Left alignment
        assert_eq!(
            TextLayout::wrap(content, 10, 2, HorizontalAlignment::Left),
            vec!["First line", "test!     "]
        );

        // ... Center ...
        assert_eq!(
            TextLayout::wrap(content, 10, 2, HorizontalAlignment::Center),
            vec!["First line", "   test!  "]
        );

        // ... and Right
        assert_eq!(
            TextLayout::wrap(content, 10, 2, HorizontalAlignment::Right),
            vec!["First line", "     test!"]
        )
    }
}
