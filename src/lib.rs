//! Pancurses iced Renderer implementation.
//! Allows to create TUI application using iced as the GUI framework

mod colors;
mod primitive;
mod renderer;
mod sandbox;

// Conveniently reexports common structs that the user might want to use in their application.
pub use renderer::PancursesRenderer;
pub use sandbox::Sandbox;
