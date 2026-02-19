use makepad_widgets::{WebSocket, makepad_micro_serde::*};

// #[derive(Clone, Debug, Default, DeJson, SerJson)]
// pub struct ChatMessageSend {
//     pub chat_id: i64,
//     pub content: String,
// }

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub timestamp: String,
    pub files: Vec<String>,
}

#[derive(Default)]
pub struct State {
    pub username: String,
    pub msg_history: Vec<ChatMessage>,
    pub token: String,
    pub socket: Option<WebSocket>,
}

impl State {
    pub const fn new() -> Self {
        State {
            username: String::new(),
            msg_history: Vec::new(),
            token: String::new(),
            socket: None,
        }
    }

    pub fn get_msg_number(&self) -> usize {
        self.msg_history.len()
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        self.msg_history.push(msg);
    }
}
