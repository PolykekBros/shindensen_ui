use crate::state::*;
use makepad_draw::MatchEvent;
use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::dialog::*;
    use crate::ui::*;
    use crate::autho::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                caption_bar = {
                    visible: true,
                    margin: {left: -100},
                    caption_label = { label = {text: "ShinDensen"} }
                },
                body = <View> {
                    width: Fill, height: Fill,
                    flow: Overlay,
                    spacing: 0.,
                    margin: 0.,
                    auth_page = <LoginForm> {
                        width: Fill, height: Fill,
                        visible: true
                    },
                    dialog_page = <DialogPage> {
                        width: Fill, height: Fill,
                        visible: false
                    }
                }
            }
        }
    }
}

#[derive(Default, Debug)]
enum Screen {
    #[default]
    Auth,
    Dialog,
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
    #[rust]
    screen: Screen,
}

impl App {
    fn apply_visibility(&mut self, cx: &mut Cx) {
        match self.screen {
            Screen::Auth => {
                self.ui.view(id!(auth_page)).set_visible(cx, true);
                self.ui.view(id!(dialog_page)).set_visible(cx, false);
            }
            Screen::Dialog => {
                self.ui.view(id!(auth_page)).set_visible(cx, false);
                self.ui.view(id!(dialog_page)).set_visible(cx, true);
            }
        }
        // self.ui.redraw(cx);
        cx.redraw_all();
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::layout::live_design(cx);
        crate::dialog::live_design(cx);
        crate::ui::live_design(cx);
        crate::autho::live_design(cx);
    }
}

impl MatchEvent for App {}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut scope);
        });

        if self.ui.button(id!(enter)).clicked(&actions) {
            self.screen = Screen::Dialog;
            let nick = self.ui.text_input(id!(nickname)).text();
            self.ui.text_input(id!(nickname)).set_text(cx, "");
            log!("Nickname now is: {}", nick);
        }
        self.apply_visibility(cx);
    }
}

app_main!(App);
