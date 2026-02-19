use crate::state::*;
use makepad_draw::MatchEvent;
use makepad_micro_serde::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::dialog::*;
    use crate::ui::*;
    use crate::autho::*;

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
                }
            }
        }
    }
}

#[derive(Default, Debug)]
enum Screen {
    #[default]
    Auth,
    Dialog,
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
    #[rust]
    screen: Screen,
}

impl App {
    fn apply_visibility(&mut self, cx: &mut Cx) {
        match self.screen {
            Screen::Auth => {
                self.ui.widget(id!(auth_page)).set_visible(cx, true);
                self.ui.widget(id!(dialog_page)).set_visible(cx, false);
            }
            Screen::Dialog => {
                self.ui.widget(id!(auth_page)).set_visible(cx, false);
                self.ui.widget(id!(dialog_page)).set_visible(cx, true);
            }
        }
        self.ui.redraw(cx);
    }

    fn set_user(&mut self, cx: &mut Cx) {
        let nick = self.ui.text_input(id!(nickname)).text();
        if !nick.is_empty() {
            self.screen = Screen::Dialog;
            self.ui.text_input(id!(nickname)).set_text(cx, "");
            log!("Nickname now is: {}", nick);
            self.state.username = nick;
        }
    }

    fn update_history_with_chunk(&mut self, cx: &mut Cx, chunk: String) {
        if let Some(last_msg) = self.state.msg_history.last_mut() {
            if last_msg.role == "assistant" {
                last_msg.content.push_str(&chunk);
            } else {
                self.state.msg_history.push(ChatMessage {
                    role: "assistant".to_string(),
                    content: chunk,
                });
            }
        } else {
            self.state.msg_history.push(ChatMessage {
                role: "assistant".to_string(),
                content: chunk,
            });
        }
        self.ui.redraw(cx);
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::layout::live_design(cx);
        crate::dialog::live_design(cx);
        crate::ui::live_design(cx);
        crate::autho::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for event in responses {
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    if response.status_code != 200 {
                        error!("Server Error: Status {}", response.status_code);
                        continue;
                    }

                    match event.request_id {
                        live_id!(AuthRequest) => {
                            if let Ok(auth_data) = response.get_json_body::<AuthResponse>() {
                                self.state.username = auth_data.username;
                                self.state.token = auth_data.token;
                                log!("Authenticated as: {}", self.state.username);
                            }
                        }

                        live_id!(GetHistory) => {
                            if let Ok(server_data) = response.get_json_body::<ServerResponse>() {
                                if let Some(history) = server_data.history {
                                    self.state.msg_history = history;
                                    self.ui.redraw(cx);
                                }
                            }
                        }
                        _ => (),
                    }
                }

                NetworkResponse::HttpStreamResponse(response) => {
                    let data = response.get_string_body().unwrap();

                    for line in data.split("\n\n") {
                        if let Some(payload) = line.strip_prefix("data: ") {
                            let payload = payload.trim();
                            if payload == "[DONE]" {
                                continue;
                            }

                            if let Ok(stream_chunk) = ServerResponse::deserialize_json(payload) {
                                if let Some(delta) = stream_chunk.delta {
                                    if let Some(new_text) = delta.content {
                                        self.update_history_with_chunk(cx, new_text);
                                    }
                                }
                            }
                        }
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

        if self.ui.button(id!(enter)).clicked(&actions) {
            self.set_user(cx);
        }
        if let Some(_) = self.ui.text_input(id!(nickname)).returned(&actions) {
            self.set_user(cx);
        }
        self.apply_visibility(cx);
    }
}

#[derive(DeJson, Debug)]
pub struct AuthResponse {
    pub username: String,
    pub token: String,
}

#[derive(DeJson, Debug)]
pub struct ServerResponse {
    pub content: Option<String>,
    pub history: Option<Vec<ChatMessage>>,
    pub delta: Option<Delta>,
}

#[derive(DeJson, Debug)]
pub struct Delta {
    pub content: Option<String>,
}

app_main!(App);
