use makepad_widgets::makepad_micro_serde::*;

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Default, Clone)]
pub struct State {
    pub username: String,
    pub msg_history: Vec<ChatMessage>,
    pub token: String,
}

impl State {
    pub const fn new() -> Self {
        State {
            username: String::new(),
            msg_history: Vec::new(),
            token: String::new(),
        }
    }

    pub fn get_msg_number(&self) -> usize {
        self.msg_history.len()
    }

    pub fn add_message(&mut self, user: String, text: &String) {
        self.msg_history.push(ChatMessage {
            role: user,
            content: text.clone(),
        });
    }
}
