use makepad_widgets::*;
use crate::state::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.layout.*

    mod.dialog = {}

    mod.dialog.NewsFeed = #(NewsFeed::register_widget(vm)) {
        list := PortalList {
            scroll_bar: ScrollBar {}
            auto_tail: true
            BottomSpace := View {height: 100.0}

            post := CachedView {
                flow: Down
                user_msg := mod.ui.Post {}
            }
        }
    }

    mod.dialog.DialogPage = #(DialogPage::register_widget(vm)) mod.layout.MessageListPage {
        contacts +: {
            mod.dialog_list.ChatList {}
        }
        dialog +: {
            news_feed := mod.dialog.NewsFeed {}
            input_bar := View {
                width: Fill
                height: Fit
                flow: Right
                View {
                    width: Fill
                    height: 150.0
                    scroll_bars := ScrollBars {}
                    msg := mod.ui.InputField {
                            width: Fill
                            empty_text: "Type a message..."
                    }
                }
                send := mod.ui.Button {text: "Send"}
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
struct DialogPage {
    #[deref]
    view: View,
}

impl DialogPage {
    pub fn send_message_ws(&mut self, scope: &mut Scope, cx: &mut Cx) {
        let state = scope.data.get_mut::<State>().expect("State not found.");
        let input = self.text_input(cx, ids!(dialog.input_bar.msg));
        let text = input.text();
        input.set_text(cx, "");
        if let Some(chat_id) = state.open_chat_id {
            log!("Sending message to chat_id: {}", chat_id);
            state.client.send_message(cx, chat_id, text);
            self.view(cx, ids!(news_feed)).redraw(cx);
        } else {
            log!("Error: dialog is not opened!")
        }
    }
}

impl Widget for DialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });
        let send_btn = self.button(cx, ids!(dialog.input_bar.send));
        if send_btn.clicked(&actions) || self.text_input(cx, ids!(dialog.input_bar.msg)).returned(&actions).is_some() {
            let text = self.text_input(cx, ids!(dialog.input_bar.msg)).text();
            if !text.is_empty() {
                self.send_message_ws(scope, cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

#[derive(Script, ScriptHook, Widget)]
struct NewsFeed {
    #[deref]
    view: View,
}

impl Widget for NewsFeed {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get::<State>().expect("State not found.");
                let msg_count = state.get_message_number();
                list.set_item_range(cx, 0, msg_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id < msg_count {
                        let item = list.item(cx, item_id, id!(post));
                        if let Some(chat_id) = state.open_chat_id
                            && let Some(messages) = state.msg_history.get(&chat_id)
                            && let Some(msg) = messages.get(item_id)
                        {
                            let sender_name =
                                if let Some(user) = state.user_info.get(&msg.sender_id) {
                                    user.display_name
                                        .clone()
                                        .unwrap_or_else(|| user.username.clone())
                                } else {
                                    msg.sender_id.to_string()
                                };
                            item.label(cx, ids!(user_msg.body.username.text))
                                .set_text(cx, &sender_name);
                            item.markdown(cx, ids!(user_msg.body.content.text))
                                .set_text(cx, msg.content.as_deref().unwrap_or(""));
                        }
                        item.draw_all_unscoped(cx);
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}
