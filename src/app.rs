use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;
    use crate::dialog::*;

    TestLabel = <Label> {
        width: Fit
        height: Fit
        margin: { top: 10.0, right: 6.0, bottom: 10.0, left: 6.0 }
        padding: { top: 5.0, right: 3.0, bottom: 5.0, left: 3.0 }
        align: { x: 0.0, y: 0.5 }
        flow: RightWrap
        hover_actions_enabled: false
        text: "Welcome to not a chat!"

        draw_text: {
            color_dither: 1.0
            gradient_fill_horizontal: 0.0
            font_scale: 1.0

            color:  #fff
            color_2:#f00

            text_style: {
                font_size: 16.0
                line_spacing: 1.2
            }
        }
    }

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
                    flow: Down,
                    spacing: 0.,
                    margin: 0.,
                    <DialogPage> {}
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::layout::live_design(cx);
        crate::dialog::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
