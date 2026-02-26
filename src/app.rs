use crate::shindensen_client::*;
use crate::state::*;
use makepad_widgets::*;
use makepad_micro_serde::*;

pub const API_URL: &str = "http://127.0.0.1:3000";
pub const WS_URL: &str = "ws://127.0.0.1:3000/ws";

script_mod! {
    use mod.prelude.widgets.*
    use mod.layout.*
    use mod.dialog_list.*
    use mod.dialog.*
    use mod.autho.*
    use mod.new_chat.*

    startup() do #(App::script_component(vm)) {
        ui: Root {
            main_window := Window {
                window +: { title: "ShinDensen" }
                body +: {
                    width: Fill, height: Fill,
                    flow: Overlay,
                    spacing: 0.0,
                    margin: Inset { top: 0.0, left: 0.0, right: 0.0, bottom: 0.0},
                    auth_page := mod.autho.LoginForm {
                        width: Fill, height: Fill,
                        visible: true
                    }
                    dialog_page := mod.dialog.DialogPage {
                        width: Fill, height: Fill,
                        visible: false
                    }
                    new_chat := mod.new_chat.NewChat {
                        width: Fill, height: Fill,
                        visible: false
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl App {
    fn run(vm: &mut ScriptVm) -> Self {
        crate::makepad_widgets::script_mod(vm);
        crate::ui::script_mod(vm);
        crate::layout::script_mod(vm);
        crate::dialog_list::script_mod(vm);
        crate::dialog::script_mod(vm);
        crate::autho::script_mod(vm);
        crate::new_chat::script_mod(vm);

        let mut app = App::from_script_mod(vm, self::script_mod);
        app.state = State::new(API_URL.into(), WS_URL.into());
        app
    }
}

impl App {
    fn apply_visibility(&mut self, cx: &mut Cx) {
        match self.state.screen {
            Screen::Auth => {
                self.ui.widget(cx, ids!(main_window.body.auth_page)).set_visible(cx, true);
                self.ui.widget(cx, ids!(main_window.body.dialog_page)).set_visible(cx, false);
                self.ui.widget(cx, ids!(main_window.body.new_chat)).set_visible(cx, false);
            }
            Screen::Dialog => {
                self.ui.widget(cx, ids!(main_window.body.auth_page)).set_visible(cx, false);
                self.ui.widget(cx, ids!(main_window.body.dialog_page)).set_visible(cx, true);
                self.ui.widget(cx, ids!(main_window.body.new_chat)).set_visible(cx, false);
            }
            Screen::NewChatInit => {
                self.ui.widget(cx, ids!(main_window.body.auth_page)).set_visible(cx, false);
                self.ui.widget(cx, ids!(main_window.body.dialog_page)).set_visible(cx, false);
                self.ui.widget(cx, ids!(main_window.body.new_chat)).set_visible(cx, true);
            }
        }
        self.ui.redraw(cx);
    }

    pub fn load_chats(&mut self, cx: &mut Cx) {
        self.state.client.get_chats(cx);
    }

    fn new_chat_init(&mut self, cx: &mut Cx) {
        self.state.screen = Screen::Dialog;
        self.ui.text_input(cx, ids!(main_window.body.new_chat.chat_name)).set_text(cx, "");
    }
}



impl MatchEvent for App {
    fn handle_draw_2d(&mut self, cx: &mut Cx2d) {
        self.ui.draw_all(cx, &mut Scope::with_data(&mut self.state));
    }

    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        self.state.client.handle_network_responses(cx, responses);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        for action in actions {
            match action.cast() {
                ShinDensenClientAction::Authenticated => {
                    self.state
                        .client
                        .user_search(cx, self.state.username.clone());
                    self.load_chats(cx);
                    log!("Authenticated successfully");
                }
                ShinDensenClientAction::NewMessage(msg) => {
                    let sender_id = msg.sender_id;
                    self.state.add_message(msg);
                    self.state.fetch_user(cx, sender_id);
                    self.ui.redraw(cx);
                }
                ShinDensenClientAction::Chats(chats) => {
                    for chat in chats {
                        for &p_id in &chat.participants {
                            self.state.fetch_user(cx, p_id);
                        }
                        let chat_id = chat.id;
                        self.state.chat_info.insert(chat_id, chat);
                        self.state.client.get_history(cx, chat_id);
                    }
                    self.ui.redraw(cx);
                    log!("Chats loaded: {}", self.state.chat_info.len());
                }
                ShinDensenClientAction::History(res) => {
                    for msg in &res.messages {
                        self.state.fetch_user(cx, msg.sender_id);
                    }
                    self.state.msg_history.insert(res.chat_id, res.messages.clone());
                    log!(
                        "History loaded for chat: {}: {} messages",
                        res.chat_id,
                        res.messages.len()
                    );
                    cx.redraw_all();
                }
                ShinDensenClientAction::Token(_) => {
                    // Token is handled internally by client, but we could store it if needed
                }
                ShinDensenClientAction::UserSearchResponse(users) => {
                    if let Some(info) = users.iter().find(|u| u.username == self.state.username) {
                        self.state.current_user_id = Some(info.id);
                        self.state.user_info.insert(info.id, info.clone());
                        log!("Current user ID identified: {}", info.id);
                    }

                    if let Some(info) = users.first() {
                        self.state.user_info.insert(info.id, info.clone());
                        // If we were initiating a chat (from new_chat screen)
                        if self.state.screen == Screen::NewChatInit {
                            self.state.client.initiate_chat(cx, info.id);
                            self.ui
                                .widget(cx, ids!(main_window.body.new_chat.error_label))
                                .set_visible(cx, false);
                            log!("User found: {}, initiating chat...", info.username);
                        }
                    } else if self.state.screen == Screen::NewChatInit {
                        self.ui
                            .widget(cx, ids!(main_window.body.new_chat.error_label))
                            .set_visible(cx, true);
                    }
                    self.ui.redraw(cx);
                }
                ShinDensenClientAction::UserInfo(info) => {
                    self.state.pending_user_fetches.remove(&info.id);
                    self.state.user_info.insert(info.id, info);
                    self.ui.redraw(cx);
                }
                ShinDensenClientAction::UserNotFound => {
                    error!("User not found");
                }
                ShinDensenClientAction::InitiateChat(res) => {
                    self.state.open_chat_id = Some(res.chat_id);
                    self.state.client.get_history(cx, res.chat_id);
                    self.state.client.get_chats(cx);
                    self.new_chat_init(cx);
                    log!(
                        "Chat initiated/found: id {}, status: {}",
                        res.chat_id,
                        res.status
                    );
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
        self.ui.handle_event(cx, event, &mut Scope::with_data(&mut self.state));
        self.apply_visibility(cx);
    }
}

#[derive(DeJson, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

app_main!(App);
