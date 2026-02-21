use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub InputField = <TextInput> {
        width: 500.0
        height: Fit
        margin: { top: 6.0, right: 6.0, bottom: 6.0, left: 6.0 }
        label_align: { x: 0.0, y: 0.5 }

        text: ""
        empty_text: ""
        is_password: false
        is_numeric_only: false
        is_read_only: false
        blink_speed: 0.5

        draw_bg: {
            color_dither: 1.0

            border_radius: 2.5
            border_size: 0.75

            color: #00000040
            color_hover: #00000040
            color_down: #00000066
            color_focus: #00000040
            color_disabled: #00000000
            color_empty: #00000040

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

            color: #0000FFFF
            color_hover: #0000FFFF
            color_down: #FF00FFFF
            color_focus: #0000FFFF
            color_disabled: #00000000
            color_empty: #00000000
        }

        draw_cursor: {
            border_radius: 1.25
            color: #FFFFFFFF
        }
    }

    pub Buttons = <Button> {
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

            color: #FFFFFF1A
            color_hover: #FFFFFF26
            color_down: #00000026
            color_focus: #FFFFFF1A
            color_disabled: #FFFFFF0D

            border_color_2: #00000066
            border_color_2_hover: #00000066
            border_color_2_down: #FFFFFF40
            border_color_2_focus: #000000BF
            border_color_2_disabled: #333333FF
        }
    }

    pub TextLabel = <Label> {
        width: Fit
        height: Fit
        margin: { top: 10.0, right: 6.0, bottom: 10.0, left: 6.0 }
        padding: { top: 5.0, right: 3.0, bottom: 5.0, left: 3.0 }
        flow: RightWrap
        hover_actions_enabled: false
        text: "Welcome to ShinDensen!"

        draw_text: {
            font_scale: 1.0

            color:  #fff

            text_style: {
                font_size: 16.0
                line_spacing: 1.2
            }
        }
    }

    pub AlertField = <RoundedView> {
        width: Fit
        height: Fit
        visible: true,
        draw_bg: {
            color: #FF2400
            border_size: 1.5
            border_color: #2d2c40
        }
        alert_text = <TextLabel> { }
    }

    pub Post = <View> {
        width: Fill
        height: Fit
        padding: { top: 0., bottom: 0. }
        margin: { top: 10.0, bottom: 10. }

        body = <RoundedView> {
            width: Fill
            height: Fit
            flow: Down
            draw_bg: {
                color: #3f497e
                border_size: 1.5
                border_color: #2d2c40
            }

            username = <RoundedView> {
                width: Fill
                height: Fit
                padding: { top: 5., right: 10.,bottom: 5., left: 10. }
                show_bg: true
                draw_bg: {
                    color: #323456
                    border_size: 1.5
                    border_color: #2d2c40
                }
                text = <H4> { text: "" }
            }
            content = <View> {
                width: Fill
                height: Fit
                padding: { top: 10., right: 10.,bottom: 10., left: 10. }
                text = <P> { text: "" }
            }
        }
    }

    pub Chats = <View> {
        width: Fill
        height: Fit
        cursor: Hand
        padding: { top: 0., bottom: 0. }
        margin: { top: 10.0, bottom: 10. }

        body = <RoundedView> {
            width: Fill
            height: Fit
            flow: Down
            draw_bg: {
                color: #3f497e
                border_size: 1.5
                border_color: #2d2c40
            }

            target_usr = <RoundedView> {
                width: Fill
                height: Fit
                padding: { top: 5., right: 10.,bottom: 5., left: 10. }
                show_bg: true
                draw_bg: {
                    color: #323456
                    border_size: 1.5
                    border_color: #2d2c40
                }
                text = <H4> { text: "" }
            }
            last_msg = <View> {
                width: Fill
                height: Fit
                padding: { top: 10., right: 10.,bottom: 10., left: 10. }
                text = <P> { text: "" }
            }
        }
    }

}
