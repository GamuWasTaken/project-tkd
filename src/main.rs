use prelude::*;

fn main() -> iced::Result {
    use iced::window::{self, Level, Position};

    iced::application("TabMenu", App::update, App::view)
        .window(window::Settings {
            level: Level::AlwaysOnTop,
            position: Position::Centered,
            size: Size::new(600., 400.),
            resizable: false,
            transparent: true,
            ..Default::default()
        })
        .theme(|_| iced::Theme::Nord) // TODO Make a theme changer, just a dropdown with all the themes and update the theme on select
        .subscription(App::subscription)
        .run_with(App::new)
}

#[derive(Debug, Clone)]
pub enum Message {}

#[derive(Debug)]
pub struct App {
    // TODO Maybe call each component 'component' / 'widget' instead of the name of the module
}

use Message::*;
// TODO maybe create a macro 'with_children'? to fill the boilerplate
impl App {
    pub fn new() -> (Self, Task<Message>) {
        use Message::*;
        (Self {}, Task::batch(vec![]))
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![])
    }

    pub fn view(&self) -> Element<Message> {
        widget::row![widget::column![],].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }
}

mod prelude {
    pub use crate::Component;
    pub use iced::{event, keyboard, time, widget, window};
    pub use iced::{Alignment, Element, Length, Size, Subscription, Task};
}

pub trait Component<'a, Message: 'a>: Default {
    fn new() -> (Task<Message>, Self) {
        (Task::none(), Self::default())
    }
    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }
    fn view(&'a self) -> Element<'a, Message> {
        widget::container("Default impl").into()
    }
    fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

mod searcher {
    use iced::widget::{column, scrollable, text_input, Column};

    use super::*;
    // search bar
    // result tab

    #[derive(Default)]
    pub struct SearchBar {
        content: String,
    }

    #[derive(Default)]
    pub struct ResultPad<'a> {
        results: Vec<Element<'a, Message>>,
    }

    #[derive(Debug, Clone)]
    pub enum Message {
        ContentChanged(String),
    }

    impl Component<'_, Message> for SearchBar {
        fn view(&'_ self) -> Element<'_, Message> {
            text_input("Search...", &self.content)
                .on_input(Message::ContentChanged)
                .into()
        }
        fn update(&mut self, message: Message) -> Task<Message> {
            match message {
                Message::ContentChanged(content) => self.content = content,
            };

            Task::none()
        }
    }

    impl Component<'_, Message> for ResultPad<'_> {
        fn view(&'_ self) -> Element<'_, Message> {
            scrollable(column(self.results.iter().copied())).into()
        }
        fn update(&mut self, _message: Message) -> Task<Message> {}
    }
}
