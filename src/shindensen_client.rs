#![allow(clippy::question_mark)]
use makepad_micro_serde::*;
use makepad_widgets::*;

#[derive(Default)]
pub struct ShinDensenClient {
    api_url: String,
    ws_url: String,
    socket: Option<WebSocket>,
    token: Option<String>,
}

#[derive(SerJson, Debug)]
pub struct AuthRequestPayload {
    pub username: String,
}

#[derive(SerJson, Debug)]
pub struct FilePayload {
    pub _type: String,
    pub url: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub size_bytes: i64,
}

#[derive(SerJson, Debug)]
pub struct ChatMessagePayload {
    pub chat_id: i64,
    pub content: Option<String>,
    pub files: Option<Vec<FilePayload>>,
}

#[derive(SerJson, Debug)]
pub struct InitiateChatPayload {
    pub target_username: String,
}

#[derive(DeJson, Debug)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Clone, DeJson, Debug, PartialEq)]
pub struct InitiateChatResponse {
    pub chat_id: i64,
    pub status: String,
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
pub struct FileMetadata {
    pub id: i64,
    pub _type: String,
    pub url: String,
    pub filename: String,
    pub mime_type: Option<String>,
    pub size_bytes: i64,
    pub created_at: String,
}

#[derive(Clone, Debug, Default, DeJson, SerJson, PartialEq)]
pub struct ChatMessage {
    pub id: i64,
    pub chat_id: i64,
    pub sender_id: i64,
    pub content: Option<String>,
    pub timestamp: String,
    pub files: Vec<FileMetadata>,
}

#[derive(Clone, DeJson, Debug, PartialEq)]
pub struct GetHistoryResponse {
    pub chat_id: i64,
    pub messages: Vec<ChatMessage>,
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
    History(GetHistoryResponse),
    Token(String),
    UserSearchResponse(UserInfoResponse),
    UserInfo(UserInfoResponse),
    UserNotFound,
    InitiateChat(InitiateChatResponse),
    Error(String),
    NetworkError(String),
    #[default]
    None,
}

impl ShinDensenClient {
    pub fn new(api_url: String, ws_url: String) -> Self {
        Self {
            api_url,
            ws_url,
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

    pub fn user_get_by_id(&self, cx: &mut Cx, user_id: i64) {
        self.send_request::<String>(
            cx,
            &format!("users/{}", user_id),
            None,
            live_id!(GetUserInfo),
        );
    }

    pub fn initiate_chat(&self, cx: &mut Cx, target_username: String) {
        let payload = InitiateChatPayload { target_username };
        self.send_request(cx, "chats/initiate", Some(payload), live_id!(InitiateChat));
    }

    pub fn send_message(&mut self, cx: &mut Cx, chat_id: i64, text: String) {
        match &mut self.socket {
            Some(socket) => {
                let payload = ChatMessagePayload {
                    chat_id,
                    content: Some(text),
                    files: Some(vec![]),
                };
                if let Err(e) = socket.send_string(payload.serialize_json()) {
                    cx.action(ShinDensenClientAction::NetworkError(format!(
                        "WebSocket send error: {e:?}"
                    )));
                }
            }
            None => cx.action(ShinDensenClientAction::Error(
                "Socket is NOT open, cannot send message!".to_string(),
            )),
        }
    }

    pub fn handle_signal(&mut self, cx: &mut Cx) {
        if let Some(socket) = &mut self.socket {
            while let Ok(msg) = socket.try_recv() {
                match msg {
                    WebSocketMessage::Opened => {
                        log!("WebSocket connection opened successfully");
                    }
                    WebSocketMessage::String(data) => match ChatMessage::deserialize_json(&data) {
                        Ok(msg) => {
                            cx.action(ShinDensenClientAction::NewMessage(msg));
                        }
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
                        cx.action(ShinDensenClientAction::NetworkError(format!(
                            "WebSocket error: {e:?}"
                        )));
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
                NetworkResponse::HttpResponse(response) => {
                    let data = response.get_string_body().unwrap_or_default();
                    self.handle_response(cx, event.request_id, response.status_code, data);
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

    pub fn handle_response(&mut self, cx: &mut Cx, request_id: LiveId, status: u16, data: String) {
        if (request_id == live_id!(UserInfo) || request_id == live_id!(GetUserInfo))
            && status == 404
        {
            cx.action(ShinDensenClientAction::UserNotFound);
            return;
        }

        if status != 200 && status != 0 {
            cx.action(ShinDensenClientAction::Error(format!(
                "Server returned error code {} for request {}",
                status, request_id
            )));
            return;
        }

        match request_id {
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
            live_id!(GetHistory) => match GetHistoryResponse::deserialize_json(&data) {
                Ok(res) => {
                    cx.action(ShinDensenClientAction::History(res));
                }
                Err(e) => {
                    cx.action(ShinDensenClientAction::Error(format!(
                        "Parsing GetHistory: {e:?}"
                    )));
                }
            },
            live_id!(UserInfo) => match UserInfoResponse::deserialize_json(&data) {
                Ok(info) => {
                    cx.action(ShinDensenClientAction::UserSearchResponse(info));
                }
                Err(e) => {
                    cx.action(ShinDensenClientAction::Error(format!(
                        "Parsing UserInfo: {e:?}"
                    )));
                }
            },
            live_id!(GetUserInfo) => match UserInfoResponse::deserialize_json(&data) {
                Ok(info) => {
                    cx.action(ShinDensenClientAction::UserInfo(info));
                }
                Err(e) => {
                    cx.action(ShinDensenClientAction::Error(format!(
                        "Parsing GetUserInfo: {e:?}"
                    )));
                }
            },
            live_id!(InitiateChat) => match InitiateChatResponse::deserialize_json(&data) {
                Ok(res) => {
                    cx.action(ShinDensenClientAction::InitiateChat(res));
                }
                Err(e) => {
                    cx.action(ShinDensenClientAction::Error(format!(
                        "Parsing InitiateChat: {e:?}"
                    )));
                }
            },
            _ => {}
        }
    }
}
