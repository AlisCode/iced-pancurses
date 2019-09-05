use std::collections::HashMap;
use std::ops::Index;

pub struct ColorRegistry {
    colors: HashMap<String, u32>,
    color_idx: u32,
}

impl ColorRegistry {
    pub fn add(&mut self, key: String, (fg, bg): (i16, i16)) {
        pancurses::init_pair(self.color_idx as i16, fg, bg);
        self.colors.insert(key, self.color_idx);
        self.color_idx += 1;
    }
}

impl Default for ColorRegistry {
    fn default() -> Self {
        let mut registry = ColorRegistry {
            colors: Default::default(),
            color_idx: 1,
        };

        registry.add("red".into(), (pancurses::COLOR_RED, -1));
        registry.add("blue".into(), (pancurses::COLOR_BLUE, -1));
        registry.add("green".into(), (pancurses::COLOR_GREEN, -1));
        registry.add("white".into(), (pancurses::COLOR_WHITE, -1));
        registry.add("magenta".into(), (pancurses::COLOR_MAGENTA, -1));
        registry.add("cyan".into(), (pancurses::COLOR_CYAN, -1));
        registry.add("yellow".into(), (pancurses::COLOR_YELLOW, -1));
        registry.add("black".into(), (pancurses::COLOR_BLACK, -1));

        registry
    }
}

impl Index<&str> for ColorRegistry {
    type Output = u32;

    fn index(&self, index: &str) -> &u32 {
        &self.colors[index]
    }
}
