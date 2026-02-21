use crate::state::*;
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

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
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

    fn new_chat_init(&mut self, _cx: &mut Cx) {
        let chat_name = self.ui.text_input(id!(chat_name)).text();
        if !chat_name.is_empty() {
            log!("Created new chat!")
        }
        self.state.screen = Screen::Dialog;
    }

    pub fn load_chats(&mut self, cx: &mut Cx) {
        if self.state.token.is_empty() {
            log!("Warning: Attempted to load chats without a token.");
            return;
        }
        let mut request = HttpRequest::new(format!("{API_URL}/chats"), HttpMethod::GET);
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        request.set_header(
            "Authorization".to_string(),
            format!("Bearer {}", self.state.token),
        );
        request.is_streaming = true;
        log!("Requesting chats list for user: {}", self.state.username);
        cx.http_request(live_id!(GetChats), request);
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
        for event in responses {
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    if response.status_code != 200 && response.status_code != 0 {
                        error!("Server Error: Status {}", response.status_code);
                        continue;
                    }
                }
                NetworkResponse::HttpStreamResponse(response) => {
                    if response.status_code != 200 && response.status_code != 0 {
                        error!("API response: {response:?}");
                    }
                    let data = response.get_string_body().unwrap();

                    match event.request_id {
                        live_id!(AuthRequest) => {
                            if let Ok(auth_data) = AuthResponse::deserialize_json(&data) {
                                self.state.token = auth_data.token;
                                self.load_chats(cx);
                                log!(
                                    "Authenticated as: {}, token is: {}",
                                    self.state.username,
                                    self.state.token
                                );
                            } else {
                                error!("Failed to parse AuthResponse: {}", data);
                            }
                        }

                        live_id!(GetHistory) => match MsgHistory::deserialize_json(&data) {
                            Ok(history) => {
                                let chat_id = history[0].chat_id;
                                self.state.msg_history.insert(chat_id, history);
                                self.ui.redraw(cx);
                                log!("Msg_history is imported for chat: {:?}", chat_id);
                            }
                            Err(e) => {
                                error!("Deserialzing msg history response: {e:?}");
                            }
                        },

                        live_id!(GetChats) => match ChatsList::deserialize_json(&data) {
                            Ok(chats) => {
                                chats.into_iter().for_each(|chat| {
                                    self.state.chat_info.insert(chat.id, chat);
                                });
                                self.ui.redraw(cx);
                                log!(
                                    "Chats are imported, number of chats is: {:?}",
                                    self.state.get_chats_number()
                                );
                            }
                            Err(e) => {
                                error!("Deserialzing chats response: {e:?}: {data}");
                            }
                        },
                        _ => (),
                    }
                }

                NetworkResponse::HttpRequestError(err) => {
                    error!("Network Request Failed: {:?}", err);
                }

                _ => (),
            }
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut scope);
        });
        if self.ui.button(id!(create)).clicked(&actions) {
            self.new_chat_init(cx);
        }
        if let Some(_) = self.ui.text_input(id!(chat_name)).returned(&actions) {
            self.new_chat_init(cx);
        }
        self.apply_visibility(cx);

        if let Some(mut ws) = self.state.socket.take() {
            let mut is_closed = false;
            while let Ok(msg) = ws.try_recv() {
                log!("received shit from webscocket: {msg:?}");
                match msg {
                    WebSocketMessage::String(s) => match ChatMessage::deserialize_json(&s) {
                        Ok(msg) => {
                            self.state.add_message(msg);
                            self.ui.redraw(cx);
                        }
                        Err(e) => {
                            error!("{e:?}");
                        }
                    },
                    WebSocketMessage::Error(e) => error!("WS Error: {}", e),
                    WebSocketMessage::Closed => {
                        is_closed = true;
                        log!("WS Disconnected");
                    }
                    _ => (),
                }
            }
            if !is_closed {
                self.state.socket = Some(ws);
            }
        }
    }
}

#[derive(DeJson, Debug)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(DeJson, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson)]
pub struct MsgHistoryResponse {
    pub files: Vec<String>,
}

type MsgHistory = Vec<ChatMessage>;
type ChatsList = Vec<ChatInfo>;

app_main!(App);
