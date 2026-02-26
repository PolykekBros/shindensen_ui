use crate::state::*;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.layout.*

    mod.new_chat = {}

    mod.new_chat.NewChat = #(NewChat::register_widget(vm)) {
        flow: Down
        align: Align { x: 0.5, y: 0.5 }
        mod.ui.TextLabel {
            text: "Enter new chat name:"
            draw_text +: {
                text_style +: {
                    font_size: 12.0
                }
            }
        }
        chat_name := mod.ui.InputField {
            empty_text: "New chat name"
        }
        View {
            width: Fill, height: Fit
            align: Align { x: 0.5, y: 0.0 }
            spacing: 10.0
            flow: Right
            back := mod.ui.Button {
                text: "Return"
            }
            create := mod.ui.Button {
                text: "Create"
            }
        }
        error_label := mod.ui.AlertField { alert_text +: { text: "User not found!" }}
    }
}


#[derive(Script, ScriptHook, Widget)]
struct NewChat {
    #[deref]
    view: View,
}

impl NewChat {
    fn user_search(&mut self, cx: &mut Cx, state: &mut State) {
        let user = self.text_input(cx, ids!(chat_name)).text();
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
        let input = self.text_input(cx, ids!(chat_name));
        let chat_search = input.text();
        let btn_create = self.button(cx, ids!(create));
        let btn_back = self.button(cx, ids!(back));
        if !chat_search.is_empty() {
            if btn_create.clicked(&actions) || input.returned(&actions).is_some() {
                self.user_search(cx, state);
            }
        }

        if btn_back.clicked(&actions) {
            state.screen = Screen::Dialog;
            input.set_text(cx, "");
            self.widget(cx, ids!(error_label)).set_visible(cx, false);
        }
    }
}
