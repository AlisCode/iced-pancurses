use iced_native::{
    Align, Button, Color, Column, Command,
    Container, Element, Image, Length, Row, Text, button, image,
};
use iced_pancurses::{Application, PancursesRenderer};

pub fn main() {
    Pokedex::run()
}

#[derive(Debug)]
enum Pokedex {
    Loading,
    Loaded {
        pokemon: Pokemon,
        search: button::State,
    },
    Errored {
        error: Error,
        try_again: button::State,
    },
}

#[derive(Debug, Clone)]
enum Message {
    PokemonFound(Result<Pokemon, Error>),
    Search,
}

impl Application for Pokedex {
    type Message = Message;

    fn new() -> (Pokedex, Command<Message>) {
        (
            Pokedex::Loading,
            Command::perform(Pokemon::search(), Message::PokemonFound),
        )
    }

    fn update(&mut self, messages: Vec<Message>) -> Vec<Command<Message>> {
        messages
            .into_iter()
            .map(|message| {
                match message {
                    Message::PokemonFound(Ok(pokemon)) => {
                        *self = Pokedex::Loaded {
                            pokemon,
                            search: button::State::new(),
                        };
        
                        Command::none()
                    }
                    Message::PokemonFound(Err(error)) => {
                        *self = Pokedex::Errored {
                            error,
                            try_again: button::State::new(),
                        };
        
                        Command::none()
                    }
                    Message::Search => match self {
                        Pokedex::Loading => Command::none(),
                        _ => {
                            *self = Pokedex::Loading;
        
                            Command::perform(Pokemon::search(), Message::PokemonFound)
                        }
                    },
                }
            })
        .collect()
    }

    fn view(&mut self) -> Element<Message, PancursesRenderer> {
        let content = match self {
            Pokedex::Loading => Column::new().width(Length::Shrink).push(
                Text::new("Searching for Pokémon...")
                    .width(Length::Shrink)
            ),
            Pokedex::Loaded { pokemon, search } => Column::new()
                .max_width(70)
                .spacing(1)
                .align_items(Align::End)
                .push(pokemon.view())
                .push(
                    button(search, "Keep searching!").on_press(Message::Search),
                ),
            Pokedex::Errored { try_again, .. } => Column::new()
                .width(Length::Shrink)
                .spacing(1)
                .align_items(Align::End)
                .push(
                    Text::new("Whoops! Something went wrong...")
                        .width(Length::Shrink)
                )
                .push(button(try_again, "Try again").on_press(Message::Search)),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
struct Pokemon {
    number: u16,
    name: String,
    description: String,
    image: image::Handle,
}

impl Pokemon {
    const TOTAL: u16 = 807;

    fn view(&self) -> Element<Message, PancursesRenderer> {
        Row::new()
            .spacing(1)
            .align_items(Align::Center)
            .push(Image::new(self.image.clone()))
            .push(
                Column::new()
                    .spacing(1)
                    .push(
                        Row::new()
                            .align_items(Align::Center)
                            .spacing(1)
                            .push(Text::new(&self.name))
                            .push(
                                Text::new(format!("#{}", self.number))
                                    .width(Length::Shrink)
                                    .color([1., 0., 0.]),
                            ),
                    )
                    .push(Text::new(&self.description)),
            )
            .into()
    }

    async fn search() -> Result<Pokemon, Error> {
        use rand::Rng;
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct Entry {
            id: u32,
            name: String,
            flavor_text_entries: Vec<FlavorText>,
        }

        #[derive(Debug, Deserialize)]
        struct FlavorText {
            flavor_text: String,
            language: Language,
        }

        #[derive(Debug, Deserialize)]
        struct Language {
            name: String,
        }

        let id = {
            let mut rng = rand::thread_rng();

            rng.gen_range(0, Pokemon::TOTAL)
        };

        let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
        let sprite = format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/{}.png", id);

        let (entry, sprite): (Entry, _) = futures::future::try_join(
            surf::get(&url).recv_json(),
            surf::get(&sprite).recv_bytes(),
        )
        .await?;

        let description = entry
            .flavor_text_entries
            .iter()
            .filter(|text| text.language.name == "en")
            .next()
            .ok_or(Error::LanguageError)?;

        Ok(Pokemon {
            number: id,
            name: entry.name.to_uppercase(),
            description: description
                .flavor_text
                .chars()
                .map(|c| if c.is_control() { ' ' } else { c })
                .collect(),
            image: image::Handle::from_memory(sprite),
        })
    }
}

#[derive(Debug, Clone)]
enum Error {
    APIError,
    LanguageError,
}

impl From<surf::Exception> for Error {
    fn from(exception: surf::Exception) -> Error {
        dbg!(&exception);

        Error::APIError
    }
}

fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message, PancursesRenderer> {
    Button::new(state, Text::new(text).color(Color::WHITE))
        .background(Color::from_rgb(0.11, 0.42, 0.87))
        .padding(1)
}
