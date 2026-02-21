#![allow(clippy::question_mark)]
use crate::state::*;
use crate::shindensen_client::*;
use makepad_draw::MatchEvent;
use makepad_micro_serde::*;
use makepad_widgets::*;

pub const API_URL: &str = "http://localhost:3000";
pub const WS_URL: &str = "ws://localhost:3000";

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::dialog::*;
    use crate::ui::*;
    use crate::autho::*;
    use crate::new_chat::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                caption_bar = {
                    visible: true,
                    margin: {left: -100},
                    caption_label = { label = {text: "ShinDensen"} }
                },
                body = <View> {
                    width: Fill, height: Fill,
                    flow: Overlay,
                    spacing: 0.,
                    margin: 0.,
                    auth_page = <LoginForm> {
                        width: Fill, height: Fill,
                        visible: true
                    },
                    dialog_page = <DialogPage> {
                        width: Fill, height: Fill,
                        visible: false
                    }
                    new_chat = <NewChat> {
                        width: Fill, height: Fill,
                        visible: false
                    }
                }
            }
        }
    }
}

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.state = State::new(API_URL.into(), WS_URL.into());
    }
}

impl App {
    fn apply_visibility(&mut self, cx: &mut Cx) {
        match self.state.screen {
            Screen::Auth => {
                self.ui.widget(id!(auth_page)).set_visible(cx, true);
                self.ui.widget(id!(dialog_page)).set_visible(cx, false);
                self.ui.widget(id!(new_chat)).set_visible(cx, false);
            }
            Screen::Dialog => {
                self.ui.widget(id!(auth_page)).set_visible(cx, false);
                self.ui.widget(id!(dialog_page)).set_visible(cx, true);
                self.ui.widget(id!(new_chat)).set_visible(cx, false);
            }
            Screen::NewChatInit => {
                self.ui.widget(id!(auth_page)).set_visible(cx, false);
                self.ui.widget(id!(dialog_page)).set_visible(cx, false);
                self.ui.widget(id!(new_chat)).set_visible(cx, true);
            }
        }
        self.ui.redraw(cx);
    }

    pub fn load_chats(&mut self, cx: &mut Cx) {
        self.state.client.get_chats(cx);
    }

    fn new_chat_init(&mut self, cx: &mut Cx) {
        self.state.screen = Screen::Dialog;
        self.ui.text_input(id!(chat_name)).set_text(cx, "");
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::layout::live_design(cx);
        crate::dialog::live_design(cx);
        crate::ui::live_design(cx);
        crate::autho::live_design(cx);
        crate::dialog_list::live_design(cx);
        crate::new_chat::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        self.state.client.handle_network_responses(cx, responses);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            match action.cast() {
                ShinDensenClientAction::Authenticated => {
                    self.load_chats(cx);
                    log!("Authenticated successfully");
                }
                ShinDensenClientAction::NewMessage(msg) => {
                    self.state.add_message(msg);
                    self.ui.redraw(cx);
                }
                ShinDensenClientAction::Chats(chats) => {
                    for chat in chats {
                        self.state.chat_info.insert(chat.id, chat);
                    }
                    self.ui.redraw(cx);
                    log!("Chats loaded: {}", self.state.chat_info.len());
                }
                ShinDensenClientAction::History(history) => {
                    if !history.is_empty() {
                        let chat_id = history[0].chat_id;
                        self.state.msg_history.insert(chat_id, history);
                        self.ui.redraw(cx);
                        log!("History loaded for chat: {}", chat_id);
                    }
                }
                ShinDensenClientAction::Token(_) => {
                    // Token is handled internally by client, but we could store it if needed
                }
                ShinDensenClientAction::UserInfo(info) => {
                    self.new_chat_init(cx);
                    log!("Started chat with: {}", info.username);
                }
                ShinDensenClientAction::Error(e) => {
                    error!("Client Error: {}", e);
                }
                ShinDensenClientAction::NetworkError(e) => {
                    error!("Network Error: {}", e);
                }
                ShinDensenClientAction::None => (),
            }
        }
    }

    fn handle_signal(&mut self, cx: &mut Cx) {
        self.state.client.handle_signal(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        self.ui.handle_event(cx, event, &mut scope);
        self.apply_visibility(cx);
    }
}

#[derive(DeJson, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

app_main!(App);
