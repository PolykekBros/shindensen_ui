use crate::state::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::ui::*;

    pub NewChat = {{NewChat}} {
        flow: Down,
        align: { x: 0.5, y: 0.5 },
        <TextLabel> {
            text: "Enter new chat name:"
            draw_text:{
                text_style: {
                    font_size: 12.0
                }
            }
        }
        chat_name = <InputField> {
            empty_text: "New chat name"
        }
        <View> {
            width: Fill, height: Fit
            align: { x: 0.5, y: 0.0 }
            spacing: 10.0
            flow: Right
            back = <Buttons> {
                text: "Return"
            }
            create = <Buttons> {
                text: "Create"
            }
        }
        error_label = <AlertField> { alert_text = { text: "User not found!" }}
    }
}

#[derive(Live, LiveHook, Widget)]
struct NewChat {
    #[deref]
    view: View,
}

impl NewChat {
    fn user_search(&mut self, cx: &mut Cx, state: &mut State) {
        let user = self.text_input(id!(chat_name)).text();
        if !user.is_empty() {
            state.client.user_search(cx, user);
        }
    }
}

impl Widget for NewChat {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });
        let state = scope.data.get_mut::<State>().expect("State not found.");
        let chat_search = self.text_input(id!(chat_name)).text();
        let btn_create = self.button(id!(create));
        let btn_back = self.button(id!(back));
        if !chat_search.is_empty() {
            if btn_create.clicked(&actions) {
                self.user_search(cx, state);
            }
            if self.text_input(id!(chat_name)).returned(&actions).is_some() {
                self.user_search(cx, state);
            }
        }

        if btn_back.clicked(&actions) {
            state.screen = Screen::Dialog;
            self.text_input(id!(chat_name)).set_text(cx, "");
            self.widget(id!(error_label)).set_visible(cx, false);
        }
        // if self.text_input(id!(chat_name)).returned(&actions).is_some() {
        //     self.user_search(cx, state);
        // } TODO: Make go back on esc button click
    }
}
