use super::prelude::*;

#[derive(Debug, Default)]
pub struct Audio {
    music: music::Music,
    // volume: volume::Volume,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Volume(volume::Message),
    Music(music::Message),
}
use Message::*;
impl<'a> Component<'a, Message> for Audio {
    fn new() -> (Task<Message>, Self) {
        let (music_task, music) = music::Music::new();
        // let (volume_task, volume) = volume::Volume::new();
        (
            Task::batch(vec![
                music_task.map(Music),
                // volume_task.map(Volume), //
            ]),
            Self {
                music,
                //volume
            },
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Volume(message) => self.volume.update(message).map(Volume),
            Music(message) => self.music.update(message).map(Music),
        }
    }

    fn view(&self) -> Element<Message> {
        widget::column![
            // Volume
            // self.volume.view().map(Volume),
            // Player
            self.music.view().map(Music),
        ]
        .into()
    }
}

// mod volume {
//     use crate::prelude::*;

//     #[derive(Debug, Default)]
//     pub struct Volume {}

//     #[derive(Debug, Clone, Copy)]
//     pub enum Message {}

//     impl<'a> Component<'a, Message> for Volume {
//         fn view(&self) -> Element<Message> {
//             widget::row![
//                 // Volume
//                 widget::button("-").on_press(ChangeVolume(-0.05)),
//                 widget::slider(0..=100, (self.get_volume() * 100.) as u8, |v| SetVolume(
//                     (v as f64) / 100.
//                 )),
//                 widget::button("+").on_press(ChangeVolume(0.05)),
//             ]
//             .into()
//         }
//         fn update(&mut self, message: Message) -> Task<Message> {
//             todo!()
//         }
//     }
// }

// mod pipewire_workers {
//     use iced::{
//         futures::{
//             channel::mpsc::{self, Receiver, Sender},
//             SinkExt, Stream, StreamExt,
//         },
//         stream,
//     };

//     pub enum WorkerMessage {
//         ChangeVolume(f32),
//     }

//     pub fn pipewire_worker() -> Sender<WorkerMessage> {
//         // TODO write a pipewire worker
//         todo!()
//     }

//     pub fn pipewire_listener() -> impl Stream<Item = super::volume::Message> {
//         stream::channel(100, |mut output| async move {
//             // TODO It seems as monitoring the volume is harder than landing on the moon, therefore im going to pass for now
//         })
//     }
// }

mod mpris;
mod music;
