use crate::state::*;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
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

impl LoginForm {
    fn set_user(&mut self, cx: &mut Cx, scope: &mut Scope) {
        let state = scope.data.get_mut::<State>().expect("State not found.");
        let nick = self.text_input(id!(nickname)).text();
        if !nick.is_empty() {
            state.screen = Screen::Dialog;
            self.text_input(id!(nickname)).set_text(cx, "");
            log!("Nickname now is: {}", nick);
            state.username = nick.clone();
            state.client.authorize(cx, nick);
        }
    }
}

// impl MatchEvent for LoginForm {
//     fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
//         for event in responses {
//             match &event.response {
//                 NetworkResponse::HttpResponse(response) => {
//                     if response.status_code != 200 && response.status_code != 0 {
//                         error!("Server Error: Status {}", response.status_code);
//                         continue;
//                     }
//                 }
//                 NetworkResponse::HttpStreamResponse(response) => {
//                     if response.status_code != 200 && response.status_code != 0 {
//                         error!("API response: {response:?}");
//                     }
//                     let data = response.get_string_body().unwrap();
//                     if event.request_id == live_id!(AuthRequest) {
//                         if let Ok(auth_data) = AuthResponse::deserialize_json(&data) {
//                             let state = scope.data.get_mut::<State>().expect("State not found.");
//                             state.token = auth_data.token;
//                             self.load_chats(cx);
//                             log!(
//                                 "Authenticated as: {}, token is: {}",
//                                 state.username,
//                                 state.token
//                             );
//                         } else {
//                             error!("Failed to parse AuthResponse: {}", data);
//                         }
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }
// }

impl Widget for LoginForm {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let actions = cx.capture_actions(|cx| {
            self.view.handle_event(cx, event, scope);
        });

        if self.button(id!(enter)).clicked(&actions) {
            self.set_user(cx, scope);
        }
        if self.text_input(id!(nickname)).returned(&actions).is_some() {
            self.set_user(cx, scope);
        }
    }
}
