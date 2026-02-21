use std::collections::HashMap;

use makepad_widgets::{WebSocket, makepad_micro_serde::*};

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub timestamp: String,
    pub files: Vec<String>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct ChatInfo {
    pub id: i64,
    pub name: Option<String>,
    pub chat_type: String,
    pub created_at: String,
}

#[derive(Default, Debug)]
pub enum Screen {
    #[default]
    Auth,
    Dialog,
    NewChatInit,
}

#[derive(Default)]
pub struct State {
    pub username: String,
    pub chat_info: HashMap<i64, ChatInfo>,
    pub msg_history: HashMap<i64, Vec<ChatMessage>>,
    pub open_chat_id: Option<i64>,
    pub token: String,
    pub socket: Option<WebSocket>,
    pub screen: Screen,
}

impl State {
    pub fn new() -> Self {
        State {
            username: String::new(),
            chat_info: HashMap::new(),
            msg_history: HashMap::new(),
            open_chat_id: None,
            token: String::new(),
            socket: None,
            screen: Screen::default(),
        }
    }

    pub fn get_chats_number(&self) -> usize {
        self.chat_info.len()
    }

    pub fn get_message_number(&self) -> usize {
        if let Some(chat_id) = self.open_chat_id {
            let msgs = self.msg_history.get(&chat_id);
            match msgs {
                Some(m) => m.len(),
                None => 0,
            }
        } else {
            0
        }
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        let chat_id = msg.chat_id;
        self.msg_history.entry(chat_id).or_insert(vec![]).push(msg);
    }
}
