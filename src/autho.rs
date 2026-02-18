use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::dialog::*;
    use crate::ui::*;

    pub LoginForm = {{LoginForm}} {
        flow: Down,
        align: { x: 0.5, y: 0.5 },
        <TextLabel> { }
        <TextLabel> {
            text: "Enter your nickname:"
            draw_text:{
                text_style: {
                    font_size: 12.0
                }
            }
        }
        nickname = <InputField> {
            empty_text: "Today my name is ..."
        }
        enter = <Buttons> {
            text: "Start"
        }
    }
}

#[derive(Live, LiveHook, Widget)]
struct LoginForm {
    #[deref]
    view: View,
}

impl Widget for LoginForm {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}
