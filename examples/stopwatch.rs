use iced_native::widget::button;
use iced_native::{
    Align, Background, Button, Color, Column, Command, Container, Element, HorizontalAlignment,
    Length, Row, Subscription, Text,
};
use iced_pancurses::{Application, PancursesRenderer};
use std::time::{Duration, Instant};

pub fn main() {
    Stopwatch::run()
}

struct Stopwatch {
    duration: Duration,
    state: State,
    toggle: button::State,
    reset: button::State,
}

enum State {
    Idle,
    Ticking { last_tick: Instant },
}

#[derive(Debug, Clone)]
enum Message {
    Toggle,
    Reset,
    Tick(Instant),
}

impl Application for Stopwatch {
    type Message = Message;

    fn new() -> (Stopwatch, Command<Message>) {
        (
            Stopwatch {
                duration: Duration::default(),
                state: State::Idle,
                toggle: button::State::new(),
                reset: button::State::new(),
            },
            Command::none(),
        )
    }

    fn update(&mut self, messages: Vec<Message>) -> Vec<Command<Message>> {
        messages.into_iter().for_each(|message| match message {
            Message::Toggle => match self.state {
                State::Idle => {
                    self.state = State::Ticking {
                        last_tick: Instant::now(),
                    };
                }
                State::Ticking { .. } => {
                    self.state = State::Idle;
                }
            },
            Message::Tick(now) => match &mut self.state {
                State::Ticking { last_tick } => {
                    self.duration += now - *last_tick;
                    *last_tick = now;
                }
                _ => {}
            },
            Message::Reset => {
                self.duration = Duration::default();
            }
        });
        vec![
            Command::none()
        ]
    }

    fn subscription(&self) -> Subscription<Message> {
        match self.state {
            State::Idle => Subscription::none(),
            State::Ticking { .. } => time::every(Duration::from_millis(10)).map(Message::Tick),
        }
    }

    fn view(&mut self) -> Element<Message, PancursesRenderer> {
        const MINUTE: u64 = 60;
        const HOUR: u64 = 60 * MINUTE;

        let seconds = self.duration.as_secs();

        let duration = Text::new(format!(
            "{:0>2}:{:0>2}:{:0>2}.{:0>2}",
            seconds / HOUR,
            (seconds % HOUR) / MINUTE,
            seconds % MINUTE,
            self.duration.subsec_millis() / 10,
        ))
        .width(Length::Shrink);

        let button = |state, label, color: [f32; 3]| {
            Button::new(
                state,
                Text::new(label)
                    .color(Color::WHITE)
                    .horizontal_alignment(HorizontalAlignment::Center),
            )
            .min_width(10)
            .background(Background::Color(color.into()))
            .padding(1)
        };

        let toggle_button = {
            let (label, color) = match self.state {
                State::Idle => ("Start", [0.11, 0.42, 0.87]),
                State::Ticking { .. } => ("Stop", [0.9, 0.4, 0.4]),
            };

            button(&mut self.toggle, label, color).on_press(Message::Toggle)
        };

        let reset_button =
            button(&mut self.reset, "Reset", [0.7, 0.7, 0.7]).on_press(Message::Reset);

        let controls = Row::new()
            .width(Length::Shrink)
            .spacing(2)
            .push(toggle_button)
            .push(reset_button);

        let content = Column::new()
            .width(Length::Shrink)
            .align_items(Align::Center)
            .spacing(2)
            .push(duration)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod time {
    use iced_native::Subscription;
    pub fn every(duration: std::time::Duration) -> Subscription<std::time::Instant> {
        Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H, I> for Every
    where
        H: std::hash::Hasher,
    {
        type Output = std::time::Instant;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(self: Box<Self>, _input: I) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| std::time::Instant::now())
                .boxed()
        }
    }
}
