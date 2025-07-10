use crate::Component;

use super::prelude::*;

#[derive(Debug, Default)]
pub struct Panel;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    EscPressed,
    // Focusing the widget [tab, shift-tab]
    NextWidget,
    PrevWidget,
    // Focusing the control inside a widget [alt-tab, shift-alt-tab]
    NextControl,
    PrevControl,
    // Interaction with controls [space / enter, control-space / control-enter]
    Interact,
    Open,
}

impl<'a> Component<'a, Message> for Panel {
    fn update(&mut self, message: Message) -> Task<Message> {
        use window::{close, get_latest};
        use Message::*;
        match message {
            EscPressed => get_latest().and_then(close),
            NextWidget => todo!(),
            PrevWidget => todo!(),
            NextControl => todo!(),
            PrevControl => todo!(),
            Interact => todo!(),
            Open => todo!(),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use event::{Event::Keyboard, Status::Ignored};
        use keyboard::{key::Named::*, Event::KeyPressed, Key::Named, Modifiers};

        const SHIFT_ALT: Modifiers = Modifiers::SHIFT.union(Modifiers::ALT);
        const EMPTY: Modifiers = Modifiers::empty();
        const ALT: Modifiers = Modifiers::ALT;
        const SHIFT: Modifiers = Modifiers::SHIFT;

        event::listen_with(|event, status, _| {
            if let (Keyboard(KeyPressed { key, modifiers, .. }), Ignored) = (event, status) {
                match (key, modifiers) {
                    // Esc
                    (Named(Escape), EMPTY) => Some(Message::EscPressed),
                    // Alt-Tab & Shift-Alt-Tab
                    (Named(Tab), SHIFT_ALT) => Some(Message::PrevControl),
                    (Named(Tab), ALT) => Some(Message::NextControl),
                    // Tab & Shift-Tab
                    (Named(Tab), SHIFT) => Some(Message::PrevWidget),
                    (Named(Tab), EMPTY) => Some(Message::NextWidget),
                    // Space & Enter
                    (Named(Space), EMPTY) => Some(Message::Interact),
                    (Named(Enter), EMPTY) => Some(Message::Open),
                    _ => None,
                }
            } else {
                None
            }
        })
    }

    fn new() -> (Task<Message>, Self) {
        (Task::none(), Self::default())
    }
}
