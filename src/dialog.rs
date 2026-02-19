use crate::makepad_widgets::*;
use crate::state::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::ui::*;

    NewsFeed = {{NewsFeed}} {
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            auto_tail: true
            BottomSpace = <View> {height: 100.}

            post = <CachedView> {
                flow: Down,
                user_msg = <Post> {}
                <Hr> {}
            }
        }
    }

    pub DialogPage = {{DialogPage}} <MessageListPage> {
        contacts = {
            <Markdown> { body: dep("crate://self/resources/dialog.md") }
        }
        dialog = {
            news_feed = <NewsFeed> {}
            <View> {
                width: Fill
                height: Fit
                flow: Right
                <View> {
                    width: Fill
                    height: 150.0
                    scroll_bars: <ScrollBars> {}
                    msg = <InputField> {
                            width: Fill
                            empty_text: "Type a message..."
                    }
                }
                send = <Buttons> {text: "Send"}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
struct DialogPage {
    #[deref]
    layout: View,
}

impl DialogPage {
    pub fn send_message_ws(&mut self, scope: &mut Scope, cx: &mut Cx) {
        let state = scope.data.get_mut::<State>().expect("State not found.");

        if state.token.is_empty() {
            error!("Cannot send message: No auth token.");
            return;
        }

        let text = self.text_input(id!(msg)).text();
        self.text_input(id!(msg)).set_text(cx, "");

        if state.socket.is_none() {
            let url = "ws://localhost:3000/ws".to_string();
            let mut request = HttpRequest::new(url, HttpMethod::GET);
            request.set_header(
                "Authorization".to_string(),
                format!("Bearer {}", state.token),
            );
            state.socket = Some(WebSocket::open(request));
            log!("Opening WebSocket...")
        }

        if let Some(ws) = &mut state.socket {
            let payload = format!(r#"{{"chat_id": 1, "content": "{}", "files": []}}"#, text);
            if let Err(e) = ws.send_string(payload) {
                error!("Failed to send WS message: {:?}", e);
            } else {
                log!("Sent WS message: {}", text);
            }
        }

        self.view(id!(news_feed)).redraw(cx);
    }
}

impl Widget for DialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.layout.handle_event(cx, event, scope);
        });

        if self.button(id!(send)).clicked(&actions)
            || self.text_input(id!(msg)).returned(&actions).is_some()
        {
            let text = self.text_input(id!(msg)).text();
            if !text.is_empty() {
                self.send_message_ws(scope, cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.layout.draw_walk(cx, scope, walk)
    }
}

#[derive(Live, LiveHook, Widget)]
struct NewsFeed {
    #[deref]
    view: View,
}

impl Widget for NewsFeed {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().expect("State not found.");
                let msg_count = state.get_msg_number();
                list.set_item_range(cx, 0, msg_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = live_id!(post);
                    let item = list.item(cx, item_id, template);
                    if let Some(msg) = state.msg_history.get(item_id) {
                        item.label(id!(user_msg.username.text))
                            .set_text(cx, &msg.chat_id.to_string());
                        item.label(id!(content.text)).set_text(cx, &msg.content);
                    }
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
