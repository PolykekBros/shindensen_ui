use crate::app::API_URL;
use crate::makepad_widgets::*;
use crate::state::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::ui::*;

    pub ChatList = {{ChatList}} {

        chat_list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            auto_tail: true
            BottomSpace = <View> {height: 100.}

            chat = <CachedView> {
                user_chat = <Chats> {}
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
struct ChatList {
    #[deref]
    view: View,
}

impl ChatList {
    pub fn load_msg_history(&mut self, cx: &mut Cx, state: &State) {
        if state.token.is_empty() {
            log!("Warning: Attempted to load history without a token.");
            return;
        }
        let chat_id_str = state.open_chat_id.unwrap().to_string();
        let mut request = HttpRequest::new(
            format!("{API_URL}/chats/{chat_id_str}/messages"),
            HttpMethod::GET,
        );
        request.set_header("Content-Type".to_string(), "application/json".to_string());
        request.set_header(
            "Authorization".to_string(),
            format!("Bearer {}", state.token),
        );
        request.is_streaming = true;
        log!("Requesting message history for user: {}", state.username);
        cx.http_request(live_id!(GetHistory), request);
    }
}

impl Widget for ChatList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get::<State>().expect("State not found.");
                let chat_ids: Vec<i64> = state.chat_info.keys().map(|s| s.clone()).collect();
                list.set_item_range(cx, 0, chat_ids.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = live_id!(chat);
                    let item = list.item(cx, item_id, template);
                    if let Some(chat_id) = chat_ids.get(item_id) {
                        item.label(id!(target_usr.text))
                            .set_text(cx, &chat_id.to_string());
                        if let Some(msgs) = state.msg_history.get(chat_id) {
                            if let Some(last_msg) = msgs.last() {
                                item.label(id!(last_msg.text))
                                    .set_text(cx, &last_msg.content);
                            }
                            item.label(id!(target_usr.text))
                                .set_text(cx, &chat_id.to_string());
                        }
                    }
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });
        let portal_list = self.view.portal_list(id!(chat_list));
        for (item_id, _) in portal_list.items_with_actions(&actions) {
            let item_widget = portal_list.item(cx, item_id, live_id!(chat));
            let body_view = item_widget.view(id!(body));
            for action in &actions {
                if let ViewAction::FingerUp(fe) = action.as_widget_action().cast() {
                    if body_view.area().rect(cx).contains(fe.abs) {
                        let state = scope.data.get_mut::<State>().expect("State not found.");
                        let chat_ids: Vec<i64> = state.chat_info.keys().cloned().collect();
                        if let Some(selected_id) = chat_ids.get(item_id) {
                            state.open_chat_id = Some(*selected_id);
                            self.load_msg_history(cx, state);
                            log!("Opened chat: {}", *selected_id);
                            cx.redraw_all();
                        }
                    }
                }
            }
        }
    }
}
