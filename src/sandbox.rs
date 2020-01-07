use crate::PancursesRenderer;
use iced_native::{Cache, Container, Element, Length, UserInterface};

pub trait Sandbox: Sized {
    type Message: std::fmt::Debug + Send + Clone;

    /// Initializes the Sanbox
    ///
    /// Should return the initial state of the sandbox
    fn new() -> Self;

    /// Handles the dispatch of a message and updates the state of the sandbox
    ///
    /// This function should define the update logic.
    /// All messages produced by user interaction will be handled here.
    fn update(&mut self, messages: Vec<Self::Message>);

    /// Request drawing the new state of the UI
    ///
    /// Returns the root element to display using the renderer
    fn view(&mut self) -> Element<'_, Self::Message, PancursesRenderer>;

    /// Launches the sandbox and takes ownership of the current thread.
    ///
    /// This should be the last thing you execute at the end of the entrypoint of
    /// your program.
    fn run()
    where
        Self: 'static,
    {
        // Creates the sandbox and its renderer
        let mut renderer = PancursesRenderer::default();
        let mut state = Self::new();

        let mut cache = Some(Cache::default());

        loop {
            renderer.flush();
            let size = renderer.size();
            // Consumes the cache and renders the UI to primitives
            let view: Element<'_, Self::Message, PancursesRenderer> = Container::new(state.view())
                .width(Length::Units(size.0))
                .height(Length::Units(size.1))
                .into();
            let mut ui = UserInterface::build(view, cache.take().unwrap(), &mut renderer);

            // Displays the new state of the sandbox using the renderer
            let primitives = ui.draw(&mut renderer);
            renderer.draw(primitives);

            // Polls pancurses events and apply them on the ui
            let messages = renderer
                .handle()
                .map(|events| ui.update(&renderer, None, events.into_iter()));

            // Stores back the cache
            cache = Some(ui.into_cache());

            // Applies updates on the state with given messages if any
            if let Some(messages) = messages {
                state.update(messages);
            }
        }
    }
}
