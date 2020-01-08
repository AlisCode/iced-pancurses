use iced_native::Color;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// The representation of a pancurses color to be indexed by the ColorRegistry
pub struct PancursesColor {
    /// Foreground pancurses color
    foreground: i16,
    /// Background pancurses color
    backgroud: i16,
}

impl PancursesColor {
    pub fn new(fg: i16, bg: i16) -> Self {
        PancursesColor {
            foreground: fg,
            backgroud: bg,
        }
    }
}

#[derive(Debug)]
pub struct ColorRegistry {
    colors: HashMap<PancursesColor, i16>,
    idx: i16,
}

impl Default for ColorRegistry {
    fn default() -> Self {
        ColorRegistry {
            colors: Default::default(),
            idx: 1,
        }
    }
}

impl ColorRegistry {
    /// Gets the pancurses internal index of the PancursesColor.
    /// Initializes the color if it is not in the registry.
    pub fn get_idx(&mut self, color: PancursesColor) -> i16 {
        let idx = *self.colors.entry(color).or_insert(self.idx);
        pancurses::init_pair(idx, color.foreground, color.backgroud);
        self.idx += 1;
        idx
    }
}

/// Gets the closest pancurses-supported color matching the iced_native Color
pub(crate) fn get_closest_color(source: Color) -> i16 {
    let [r_src, g_src, b_src, _] = source.into_linear();
    vec![
        (Color::WHITE, pancurses::COLOR_WHITE),
        (Color::BLACK, pancurses::COLOR_BLACK),
        (
            Color {
                r: 0.,
                g: 0.,
                b: 1.,
                a: 1.,
            },
            pancurses::COLOR_BLUE,
        ),
        (
            Color {
                r: 0.,
                g: 1.,
                b: 0.,
                a: 1.,
            },
            pancurses::COLOR_GREEN,
        ),
        (
            Color {
                r: 0.,
                g: 1.,
                b: 1.,
                a: 1.,
            },
            pancurses::COLOR_CYAN,
        ),
        (
            Color {
                r: 1.,
                g: 0.,
                b: 0.,
                a: 1.,
            },
            pancurses::COLOR_RED,
        ),
        (
            Color {
                r: 1.,
                g: 0.,
                b: 1.,
                a: 1.,
            },
            pancurses::COLOR_MAGENTA,
        ),
        (
            Color {
                r: 1.,
                g: 1.,
                b: 0.,
                a: 1.,
            },
            pancurses::COLOR_YELLOW,
        ),
    ]
    .into_iter()
    .map(|(col, pancurses_color)| {
        let [r, g, b, _] = col.into_linear();
        let dist_r = r - r_src;
        let dist_g = g - g_src;
        let dist_b = b - b_src;
        (
            f32::sqrt(dist_r * dist_r + dist_g * dist_g + dist_b * dist_b),
            pancurses_color,
        )
    })
    .min_by(|(a, _), (b, _)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    .unwrap_or((Color::WHITE, pancurses::COLOR_WHITE))
    .1
}
