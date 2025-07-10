use crate::Component;

use super::prelude::*;
use chrono::prelude::*;

#[derive(Debug, Default)]
pub struct Clock(DateTime<Local>);

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Time(DateTime<Local>),
}

impl<'a> Component<'a, Message> for Clock {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Time(time) => self.0 = time,
        };
        Task::none()
    }

    fn view(&'a self) -> Element<'a, Message> {
        let time = self.0.format("%H:%M").to_string();
        let date = self.0.format("%a %e %b %Y").to_string();

        widget::container(
            widget::column![
                widget::text(time),
                widget::text(date), //
            ]
            .align_x(Alignment::Center),
        )
        .padding(10)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        use std::time::Duration;

        time::every(Duration::from_secs(1)).map(|_| Message::Time(Local::now()))
    }
}
