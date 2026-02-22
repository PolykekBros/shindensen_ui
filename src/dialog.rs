use crate::makepad_widgets::*;
use crate::state::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::ui::*;
    use crate::dialog_list::*;

    NewsFeed = {{NewsFeed}} {
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            auto_tail: true
            BottomSpace = <View> {height: 100.}

            post = <CachedView> {
                flow: Down,
                user_msg = <Post> {}
            }
        }
    }

    pub DialogPage = {{DialogPage}} <MessageListPage> {
        contacts = {
            <ChatList> {}
            // <Markdown> { body: dep("crate://self/resources/dialog.md") }
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

        let text = self.text_input(id!(msg)).text();
        self.text_input(id!(msg)).set_text(cx, "");

        if let Some(chat_id) = state.open_chat_id {
            state.client.send_message(chat_id, text);
            self.view(id!(news_feed)).redraw(cx);
        } else {
            log!("Error: dialog is not opened!")
        }
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
                let state = scope.data.get::<State>().expect("State not found.");
                let msg_count = state.get_message_number();
                list.set_item_range(cx, 0, msg_count);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = live_id!(post);
                    let item = list.item(cx, item_id, template);
                    if let Some(chat_id) = state.open_chat_id
                        && let Some(messages) = state.msg_history.get(&chat_id)
                        && let Some(msg) = messages.get(item_id)
                    {
                        item.label(id!(username.text))
                            .set_text(cx, &msg.sender_id.to_string());
                        item.label(id!(content.text))
                            .set_text(cx, msg.content.as_deref().unwrap_or(""));
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
