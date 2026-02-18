use crate::makepad_widgets::*;
use crate::state::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::ui::*;

    Post = <View> {
        width: Fill
        height: Fit
        padding: { top: 10., bottom: 10.}

        body = <RoundedView> {
            width: Fill
            height: Fit
            <View> {
                width: Fill
                height: Fit
                flow: Down
                username = <View> {
                    width: Fill
                    height: Fit
                    draw_bg: {
                        color: #000
                    }
                    text = <H4> { text: "" }
                }
                content = <View> {
                    width: Fill
                    height: Fit
                    draw_bg: {
                        color: #FFF
                    }
                    text = <P> { text: "" }
                }
            }
        }
    }

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
                height: 200.0,
                align: { x: 1.0, y: 1.0 },
                msg = <InputField> { empty_text: "Type a message..." }
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
    fn send_message(&mut self, scope: &mut Scope, cx: &mut Cx) {
        let state = scope.data.get_mut::<State>().expect("State not found.");
        let text = self.text_input(id!(msg)).text();
        self.text_input(id!(msg)).set_text(cx, "");
        self.view(id!(news_feed)).redraw(cx);
        state.add_message(&text);
        log!(
            "Send message: {}, number of messages {}",
            text,
            state.get_msg_number()
        );
    }
}

impl Widget for DialogPage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.layout.handle_event(cx, event, scope);
        });
        if self.button(id!(send)).clicked(&actions) {
            self.send_message(scope, cx);
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
                            .set_text(cx, &state.username);
                        item.label(id!(content.text)).set_text(cx, msg);
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
