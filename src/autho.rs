use crate::state::*;
use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.LoginForm = #(LoginForm::register_widget(vm)) {
        SolidView {
            width: Fill
            height: Fill
            flow: Down
            align: Align { x: 0.5, y: 0.5 }
            show_bg: true
            draw_bg +: {
                color: #26242b
            }
            SDLabel{
                text: "Enter your nickname:"
                draw_text +: {
                    text_style +: {
                        font_size: 12.0
                    }
                }
            }
            nickname := SDTextInput{
                empty_text: "Today my name is ..."
            }
            enter := SDButton{
                text: "Start"
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
struct LoginForm {
    #[deref]
    view: View,
}

impl LoginForm {
    fn set_user(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let state = scope.data.get_mut::<State>().expect("State not found.");
        let input = self.text_input(cx, ids!(nickname));
        let nick = input.text().trim().to_owned();
        if !nick.is_empty() {
            input.set_text(cx, "");
            state.username = nick.to_owned();
            state.client.authorize(cx, nick);
        }
    }
}

impl Widget for LoginForm {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.view.button(cx, ids!(enter)).clicked(&actions)
            || self
                .view
                .text_input(cx, ids!(nickname))
                .returned(&actions)
                .is_some()
        {
            self.set_user(cx, scope);
        }
        cx.extend_actions(actions);
    }
}
