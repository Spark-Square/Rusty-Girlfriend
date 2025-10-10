use std::rc::Rc;
use yew::prelude::*;
use crate::types::{ChatMessage, Sender};

#[derive(Clone, PartialEq, Default)]
pub struct ChatHistory {
    pub messages: Vec<ChatMessage>,
}

#[derive(Clone, PartialEq)]
pub enum ChatAction {
    AddMessage { sender: Sender, text: String },
    //Set(Vec<ChatMessage>),
    //Clear,
}

impl Reducible for ChatHistory {
    type Action = ChatAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut next = (*self).clone();

        match action {
            ChatAction::AddMessage { sender, text } => {
                next.messages.push(ChatMessage { sender, text });
            }
            //ChatAction::Set(msgs) => {
            //    next.messages = msgs;
            //}
            //ChatAction::Clear => {
            //    next.messages.clear();
            //}
        }

        next.into()
    }
}
