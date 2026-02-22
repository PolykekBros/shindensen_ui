use crate::makepad_widgets::*;
use crate::state::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::ui::*;

    pub ChatList = {{ChatList}} {
        flow: Down
        chat_list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            auto_tail: true
            BottomSpace = <View> {height: 100.}

            chat = <CachedView> {
                user_chat = <Chats> {}
            }
        }
        new_chat_btn = <Buttons> {
            width: Fill, height: Fit
            text: "Add new chat"
        }
    }
}

#[derive(Live, LiveHook, Widget)]
struct ChatList {
    #[deref]
    view: View,
}

impl ChatList {
    pub fn load_msg_history(&mut self, cx: &mut Cx, state: &mut State) {
        if let Some(chat_id) = state.open_chat_id {
            state.client.get_history(cx, chat_id);
            log!("Requesting message history for chat: {}", chat_id);
        } else {
            log!("Warning: Attempted to load history without an open chat.");
        }
    }
}

impl Widget for ChatList {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get::<State>().expect("State not found.");
                let chat_ids: Vec<i64> = state.chat_info.keys().copied().collect();
                list.set_item_range(cx, 0, chat_ids.len());
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = live_id!(chat);
                    let item = list.item(cx, item_id, template);
                    if let Some(chat_id) = chat_ids.get(item_id) {
                        let chat_name = if let Some(info) = state.chat_info.get(chat_id) {
                            info.name.clone().unwrap_or_else(|| chat_id.to_string())
                        } else {
                            chat_id.to_string()
                        };
                        item.label(id!(user_chat.target_usr.text))
                            .set_text(cx, &chat_name);

                        if let Some(msgs) = state.msg_history.get(chat_id) {
                            if let Some(last_msg) = msgs.last() {
                                item.label(id!(user_chat.last_msg.text))
                                    .set_text(cx, last_msg.content.as_deref().unwrap_or(""));
                            }
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
                if let ViewAction::FingerUp(fe) = action.as_widget_action().cast()
                    && body_view.area().rect(cx).contains(fe.abs)
                {
                    let state = scope.data.get_mut::<State>().expect("State not found.");
                    let chat_ids: Vec<i64> = state.chat_info.keys().copied().collect();
                    if let Some(selected_id) = chat_ids.get(item_id) {
                        state.open_chat_id = Some(*selected_id);
                        self.load_msg_history(cx, state);
                        log!("Opened chat: {}", *selected_id);
                        cx.redraw_all();
                    }
                }
            }
        }

        let state = scope.data.get_mut::<State>().expect("State not found.");
        let new_chat_btn = self.button(id!(new_chat_btn));
        if new_chat_btn.clicked(&actions) {
            state.screen = Screen::NewChatInit;
        }
    }
}
