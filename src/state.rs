use crate::shindensen_client::{ChatInfo, ChatMessage, ShinDensenClient, UserInfoResponse};
use std::collections::HashMap;
use makepad_widgets::Cx;

#[derive(Default, Debug, PartialEq, Clone)]
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
    pub current_user_id: Option<i64>,
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
            current_user_id: None,
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

    pub fn fetch_user(&mut self, cx: &mut Cx, user_id: i64) {
        if !self.user_info.contains_key(&user_id) && !self.pending_user_fetches.contains(&user_id) {
            self.pending_user_fetches.insert(user_id);
            self.client.user_get_by_id(cx, user_id);
        }
    }

    pub fn get_chat_name(&self, chat_id: i64) -> String {
        let chat = match self.chat_info.get(&chat_id) {
            Some(c) => c,
            None => {
                return chat_id.to_string();
            }
        };

        if let Some(name) = &chat.name {
            if !name.is_empty() {
                return name.clone();
            }
        }

        let mut other_participants = Vec::new();
        for &p_id in &chat.participants {
            if let Some(my_id) = self.current_user_id {
                if p_id == my_id {
                    continue;
                }
            }
            if let Some(user) = self.user_info.get(&p_id) {
                other_participants.push(user.display_name.clone().unwrap_or_else(|| user.username.clone()));
            } else {
                other_participants.push(format!("U{}", p_id));
            }
        }

        if other_participants.is_empty() {
            if let Some(my_id) = self.current_user_id {
                if chat.participants.contains(&my_id) {
                    return "Saved Messages".to_string();
                }
            }
            return chat_id.to_string();
        }

        if chat.chat_type == "direct" {
            return other_participants[0].clone();
        }

        // For groups without name, show first 3 participants
        let count = other_participants.len();
        if count > 3 {
             format!("{}, {}, {}...", other_participants[0], other_participants[1], other_participants[2])
        } else {
             other_participants.join(", ")
        }
    }
}
