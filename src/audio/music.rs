use super::mpris;
use crate::prelude::*;

#[derive(Debug, Default)]
pub struct Music {
    state: State,
}

#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    NoPlayer,
    Player {
        data: mpris::PlayerData,
        status: mpris::PlayStatus,
    },
}

#[derive(Debug, Clone)]
pub enum Message {
    Done,
    // Internal events
    PlayPause,
    Next,
    Prev,
    // External events
    StatusChanged(mpris::PlayStatus),
    SongChanged(mpris::PlayerData),
    PlayerChanged, // On playerChanged reload subscription to listen_player

    ConnectionError,
}

impl<'a> Component<'a, Message> for Music {
    fn update(&mut self, message: Message) -> Task<Message> {
        use Message::*;
        use State::*;
        match &mut self.state {
            NoPlayer => match message {
                PlayerChanged(data, status) => self.state = Player { data, status },
                _ => (),
            },

            Player { data, status } => match message {
                StatusChanged(new) => *status = new,
                SongChanged(new_data) => *data = new_data,
                PlayerChanged => (),
                PlayPause => return Task::perform(mpris::play_pause(*status), |_| Done),
                Next => return Task::perform(mpris::next(), |_| Done),
                Prev => return Task::perform(mpris::prev(), |_| Done),
                Done => (),
                ConnectionError => self.state = NoPlayer,
            },
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.state {
            State::NoPlayer => widget::column![widget::text("Nothing Playing")],
            State::Player { data, status } => {
                widget::column![
                    // Info
                    widget::text(format!("{} - {}", data.title, data.source)),
                    // Controls
                    widget::row![
                        widget::button("Prev").on_press(Message::Prev),
                        widget::button(widget::text(status.icon())).on_press(Message::PlayPause),
                        widget::button("Next").on_press(Message::Next),
                    ]
                ]
            }
        }
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(mpris::listen)
    }
}
