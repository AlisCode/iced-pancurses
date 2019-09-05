//! Pancurses iced Renderer implementation.
//! Allows to create TUI application using iced as the GUI framework

mod colors;
mod renderer;

// Conveniently reexports common structs that the user might want to use in their application.
pub use colors::ColorRegistry;
pub use renderer::PancursesRenderer;
