use makepad_micro_serde::*;
use makepad_widgets::*;
use std::collections::HashMap;

pub struct ShinDensenClient {
    api_url: String,
    ws_url: String,
    stream_chunks: HashMap<LiveId, Vec<u8>>,
    socket: Option<WebSocket>,
    token: Option<String>,
}

#[derive(SerJson, Debug)]
pub struct AuthRequestPayload {
    pub username: String,
}

#[derive(SerJson, Debug)]
pub struct ChatMessagePayload {
    pub chat_id: i64,
    pub content: String,
    pub files: Vec<String>,
}

#[derive(DeJson, Debug)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct UserInfoResponse {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub image_id: Option<i64>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: String,
    pub timestamp: String,
    pub files: Vec<String>,
}

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct ChatInfo {
    pub id: i64,
    pub name: Option<String>,
    pub chat_type: String,
    pub created_at: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ShinDensenClientAction {
    Authenticated,
    NewMessage(ChatMessage),
    Chats(Vec<ChatInfo>),
    History(Vec<ChatMessage>),
    Token(String),
    UserInfo(UserInfoResponse),
    Error(String),
    NetworkError(String),
    #[default]
    None,
}

impl Default for ShinDensenClient {
    fn default() -> Self {
        Self {
            api_url: String::new(),
            ws_url: String::new(),
            stream_chunks: HashMap::new(),
            socket: None,
            token: None,
        }
    }
}

impl ShinDensenClient {
    pub fn new(api_url: String, ws_url: String) -> Self {
        Self {
            api_url,
            ws_url,
            stream_chunks: HashMap::new(),
            socket: None,
            token: None,
        }
    }

    fn send_request<T: SerJson>(
        &self,
        cx: &mut Cx,
        suffix: &str,
        payload: Option<T>,
        live_id: LiveId,
    ) {
        let method = if payload.is_some() {
            HttpMethod::POST
        } else {
            HttpMethod::GET
        };
        let mut request = HttpRequest::new(format!("{}/{suffix}", self.api_url), method);
        if let Some(token) = &self.token {
            request.set_header("Authorization".to_string(), format!("Bearer {}", token));
        }
        if let Some(payload) = payload {
            request.set_header("Content-Type".to_string(), "application/json".to_string());
            request.set_body(payload.serialize_json().as_bytes().to_vec());
        }
        request.is_streaming = true;
        cx.http_request(live_id, request);
    }

    fn open_socket(&mut self, _cx: &mut Cx) {
        let mut request = HttpRequest::new(self.ws_url.clone(), HttpMethod::GET);
        if let Some(token) = &self.token {
            request.set_header("Authorization".to_string(), format!("Bearer {}", token));
        }
        self.socket = Some(WebSocket::open(request));
    }

    pub fn authorize(&mut self, cx: &mut Cx, user: String) {
        let payload = AuthRequestPayload { username: user };
        self.send_request(cx, "login", Some(payload), live_id!(AuthRequest));
    }

    pub fn get_chats(&self, cx: &mut Cx) {
        self.send_request::<String>(cx, "chats", None, live_id!(GetChats));
    }

    pub fn get_history(&self, cx: &mut Cx, chat_id: i64) {
        self.send_request::<String>(
            cx,
            &format!("chats/{}/messages", chat_id),
            None,
            live_id!(GetHistory),
        );
    }

    pub fn user_search(&self, cx: &mut Cx, username: String) {
        self.send_request::<String>(cx, &format!("users/{}", username), None, live_id!(UserInfo));
    }

    pub fn send_message(&mut self, chat_id: i64, text: String) {
        if let Some(socket) = &mut self.socket {
            let payload = ChatMessagePayload {
                chat_id,
                content: text,
                files: vec![],
            };
            let _ = socket.send_string(payload.serialize_json());
        }
    }

    pub fn handle_signal(&mut self, cx: &mut Cx) {
        if let Some(socket) = &mut self.socket {
            while let Ok(msg) = socket.try_recv() {
                match msg {
                    WebSocketMessage::String(data) => match ChatMessage::deserialize_json(&data) {
                        Ok(msg) => cx.action(ShinDensenClientAction::NewMessage(msg)),
                        Err(e) => cx.action(ShinDensenClientAction::Error(format!(
                            "Parsing WebSocketMessage: {e:?}"
                        ))),
                    },
                    WebSocketMessage::Closed => {
                        cx.action(ShinDensenClientAction::NetworkError(
                            "WebSocket Closed".to_string(),
                        ));
                        self.open_socket(cx);
                        break;
                    }
                    WebSocketMessage::Error(e) => {
                        cx.action(ShinDensenClientAction::NetworkError(format!("WebSocket error: {e:?}")));
                        self.open_socket(cx);
                        break;
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for event in responses {
            match &event.response {
                NetworkResponse::HttpStreamResponse(response) => {
                    let buffer = self.stream_chunks.entry(event.request_id).or_default();
                    if let Some(body) = response.get_body() {
                        buffer.extend(body);
                    }
                }
                NetworkResponse::HttpStreamComplete(response) => {
                    if let Some(body) = response.get_body() {
                        self.stream_chunks
                            .entry(event.request_id)
                            .or_default()
                            .extend(body);
                    }
                    let buffer = self.stream_chunks.remove(&event.request_id).unwrap();
                    if response.status_code != 200 {
                        cx.action(ShinDensenClientAction::Error(format!(
                            "Server returned error code {} for request {}",
                            response.status_code,
                            event.request_id
                        )));
                        return;
                    }
                    let data = match String::from_utf8(buffer) {
                        Ok(s) => s,
                        Err(e) => {
                            cx.action(ShinDensenClientAction::Error(format!(
                                "Failed to convert response body to UTF-8: {e:?}"
                            )));
                            return;
                        }
                    };
                    match event.request_id {
                        live_id!(AuthRequest) => match AuthResponse::deserialize_json(&data) {
                            Ok(data) => {
                                self.token = Some(data.token);
                                self.open_socket(cx);
                                cx.action(ShinDensenClientAction::Authenticated);
                            }
                            Err(e) => {
                                cx.action(ShinDensenClientAction::Error(format!(
                                    "Parsing AuthRequest: {e:?}"
                                )));
                            }
                        },
                        live_id!(GetChats) => match Vec::<ChatInfo>::deserialize_json(&data) {
                            Ok(chats) => {
                                cx.action(ShinDensenClientAction::Chats(chats));
                            }
                            Err(e) => {
                                cx.action(ShinDensenClientAction::Error(format!(
                                    "Parsing GetChats: {e:?}"
                                )));
                            }
                        },
                        live_id!(GetHistory) => match Vec::<ChatMessage>::deserialize_json(&data) {
                            Ok(history) => {
                                cx.action(ShinDensenClientAction::History(history));
                            }
                            Err(e) => {
                                cx.action(ShinDensenClientAction::Error(format!(
                                    "Parsing GetHistory: {e:?}"
                                )));
                            }
                        },
                        live_id!(UserInfo) => match UserInfoResponse::deserialize_json(&data) {
                            Ok(info) => {
                                cx.action(ShinDensenClientAction::UserInfo(info));
                            }
                            Err(e) => {
                                cx.action(ShinDensenClientAction::Error(format!(
                                    "Parsing UserInfo: {e:?}"
                                )));
                            }
                        },
                        _ => {}
                    }
                }
                NetworkResponse::HttpRequestError(e) => {
                    cx.action(ShinDensenClientAction::NetworkError(format!(
                        "HttpRequestError for {}: {:?}",
                        event.request_id, e
                    )));
                }
                _ => {}
            }
        }
    }
}
