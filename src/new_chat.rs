use crate::app::API_URL;
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
        error_label = <AlertField> { alert_text = { text: "User not found!" }}
        create = <Buttons> {
            text: "Create"
        }
    }
}

#[derive(Live, LiveHook, Widget)]
struct NewChat {
    #[deref]
    view: View,
}

impl NewChat {
    fn user_search(&mut self, cx: &mut Cx) {
        let user = self.text_input(id!(chat_name)).text();
        if !user.is_empty() {
            let mut request = HttpRequest::new(format!("{API_URL}/users/{user}"), HttpMethod::GET);
            request.set_header("Content-Type".to_string(), "application/json".to_string());
            request.is_streaming = true;

            cx.http_request(live_id!(UserInfo), request);
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
        let chat_search = self.text_input(id!(chat_name)).text();
        let btn_create = self.button(id!(create));
        if !chat_search.is_empty() {
            if btn_create.clicked(&actions) {
                self.user_search(cx);
            }
            if let Some(_) = self.text_input(id!(chat_name)).returned(&actions) {
                self.user_search(cx);
            }
        }
    }
}
