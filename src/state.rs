#[derive(Default, Clone)]
pub struct State {
    pub username: String,
    pub msg_history: Vec<String>,
}

impl State {
    pub const fn new() -> Self {
        State {
            username: String::new(),
            msg_history: Vec::new(),
        }
    }

    pub fn get_msg_number(&self) -> usize {
        self.msg_history.len()
    }

    pub fn add_message(&mut self, text: &String) {
        self.msg_history.push(text.clone());
    }
}
