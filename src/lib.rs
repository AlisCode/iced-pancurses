//! Terminl Iced Renderer implementation.
//! Allows to create TUI application using iced as the GUI framework

//mod application;
//mod colors;
mod primitive;
mod renderer;
mod sandbox;
//mod subscription;

// Conveniently reexports common structs that the user might want to use in their application.
//pub use application::Application;
pub use renderer::TerminalRenderer;
pub use sandbox::Sandbox;
