use crate::shindensen_client::{ChatInfo, ChatMessage, ShinDensenClient, UserInfoResponse};
use std::collections::HashMap;

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
    pub user_info: HashMap<i64, UserInfoResponse>,
    pub pending_user_fetches: std::collections::HashSet<i64>,
    pub open_chat_id: Option<i64>,
    pub screen: Screen,
    pub client: ShinDensenClient,
}

impl State {
    pub fn new(api_url: String, ws_url: String) -> Self {
        State {
            username: String::new(),
            chat_info: HashMap::new(),
            msg_history: HashMap::new(),
            user_info: HashMap::new(),
            pending_user_fetches: std::collections::HashSet::new(),
            open_chat_id: None,
            screen: Screen::default(),
            client: ShinDensenClient::new(api_url, ws_url),
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
