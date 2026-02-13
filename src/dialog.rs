use crate::makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    use crate::layout::*;

    Buttons = <Button> {
        width: 70
        height: 30
        padding: { top: 6.0, right: 6.0, bottom: 10.0, left: 6.0 }
        spacing: 3.0
        flow: Right

        enable_long_press: true
        grab_key_focus: true
        reset_hover_on_click: false
        visible: true
        text: "Button"

        label_walk: {
            width: Fit,
            height: Fit,
            margin: { top: 0.0, right: 3.0, bottom: 0.0, left: 3.0 }
        }

        icon_walk: {
            width: 15.6,
            height: Fit,
            margin: { top: 0.0, right: 3.0, bottom: 0.0, left: 3.0 }
        }

        draw_text: {
            color: #FFFFFFA6
            color_hover: #FFFFFFA6
            color_down: #FFFFFF40
            color_focus: #FFFFFFA6
            color_disabled: #FFFFFF26

            text_style: {
                font_size: 10.0
                line_spacing: 1.2
            }
        }

        draw_bg: {
            color_dither: 1.0

            border_size: 0.75
            border_radius: 2.5

            gradient_fill_horizontal: 0.0
            gradient_border_horizontal: 0.0

            color: #FFFFFF1A
            color_hover: #FFFFFF26
            color_down: #00000026
            color_focus: #FFFFFF1A
            color_disabled: #FFFFFF0D

            color_2: #FFFFFF0D
            color_2_hover: #FFFFFF26
            color_2_down: #00000040
            color_2_focus: #FFFFFF0D
            color_2_disabled: #FFFFFF0D

            border_color: #FFFFFF40
            border_color_hover: #959595FF
            border_color_down: #00000066
            border_color_focus: #959595FF
            border_color_disabled: #747474FF

            border_color_2: #00000066
            border_color_2_hover: #00000066
            border_color_2_down: #FFFFFF40
            border_color_2_focus: #000000BF
            border_color_2_disabled: #333333FF
        }
    }

    MessageInput = <TextInput> {
        width: 500.0
        height: Fit
        margin: { top: 6.0, right: 6.0, bottom: 6.0, left: 6.0 }
        label_align: { x: 0.0, y: 0.5 }

        text: ""
        empty_text: "Type a message..."
        is_password: false
        is_numeric_only: false
        is_read_only: false
        blink_speed: 0.5

        draw_bg: {
            color_dither: 1.0
            gradient_fill_horizontal: 0.0
            gradient_border_horizontal: 0.0

            border_radius: 2.5
            border_size: 0.75

            color: #00000040
            color_hover: #00000040
            color_down: #00000066
            color_focus: #00000040
            color_disabled: #00000000
            color_empty: #00000040

            color_2: #0000001A
            color_2_hover: #0000001A
            color_2_down: #00000040
            color_2_focus: #0000001A
            color_2_disabled: #00000000
            color_2_empty: #0000001A

            border_color: #00000066
            border_color_hover: #00000066
            border_color_down: #00000099
            border_color_focus: #0000FFFF
            border_color_disabled: #323232FF
            border_color_empty: #00000066

            border_color_2: #FFFFFF40
            border_color_2_hover: #959595FF
            border_color_2_down: #959595FF
            border_color_2_focus: #FF00FFFF
            border_color_2_disabled: #747474FF
            border_color_2_empty: #FFFFFF40
        }

        draw_text: {
            font_scale: 1.0
            text_style: { font_size: 14.0, line_spacing: 1.2 }

            color: #FFFFFFA6
            color_hover: #FFFFFFA6
            color_down: #FFFFFF80
            color_focus: #FFFFFFA6
            color_disabled: #FFFFFF26

            color_empty: #FFFFFF40
            color_empty_hover: #FFAA00FF
            color_empty_focus: #FF5C39FF
        }

        draw_selection: {
            border_radius: 1.25
            color_dither: 1.0
            gradient_fill_horizontal: 0.0

            color: #0000FFFF
            color_hover: #0000FFFF
            color_down: #FF00FFFF
            color_focus: #0000FFFF
            color_disabled: #00000000
            color_empty: #00000000

            color_2: #FF00FFFF
            color_2_hover: #FFAA00FF
            color_2_down: #FF00FFFF
            color_2_focus: #FFAA00FF
            color_2_disabled: #00000000
            color_2_empty: #00000000
        }

        draw_cursor: {
            border_radius: 1.25
            color: #FFFFFFFF
        }
    }

    Post = <View> {
        width: Fill
        height: Fit
        padding: { top: 10., bottom: 10.}

        body = <RoundedView> {
            width: Fill
            height: Fit
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

    NewsFeed = {{NewsFeed}} {
        list = <PortalList> {
            scroll_bar: <ScrollBar> {}
            TopSpace = <View> {height: 0.}
            BottomSpace = <View> {height: 100.}

            Post = <CachedView>{
                flow: Down,
                <Post> {}
                <Hr> {}
            }
        }
    }

    pub DialogPage = <MessageListPage> {
        contacts = {
            <Markdown> { body: dep("crate://self/resources/dialog.md") }
        }
        dialog = {
            news_feed = <NewsFeed> {}
            <View> {
                height: 200.0,
                align: { x: 1.0, y: 1.0 },
                msg = <MessageInput> {}
                send = <Buttons> {text: "Send"}
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
struct NewsFeed {
    #[deref]
    view: View,
}

impl Widget for NewsFeed {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.handle_event(cx, event, &mut Scope::empty());
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
