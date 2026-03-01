use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.SDTextInput = TextInput{
        width: Fill
        height: Fit
        margin: Inset { top: 8.0, right: 8.0, bottom: 8.0, left: 8.0 }
        padding: Inset { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 }
        label_align: Align { x: 0.0, y: 0.5 }

        empty_text: ""
        is_password: false
        is_numeric_only: false
        is_read_only: false

        draw_bg +: {
            color: #1a1a2e
            border_radius: 4.0
            border_size: 1.0
            border_color: #3a3a5a
        }

        draw_text +: {
            font_scale: 1.0
            text_style +: { font_size: 13.0, line_spacing: 1.2 }
            color: #dcdcdc
        }

        draw_selection +: {
            color: #3d5afe80
        }

        draw_cursor +: {
            color: #ffffff
        }
    }

    mod.widgets.SDButton = Button {
        width: Fit
        height: Fit
        padding: Inset { top: 8.0, right: 16.0, bottom: 8.0, left: 16.0 }
        spacing: 5.0
        flow: Right

        grab_key_focus: true
        visible: true
        text: "Button"

        draw_text +: {
            color: #ffffff
            text_style +: {
                font_size: 11.0
                line_spacing: 1.2
            }
        }

        draw_bg +: {
            color: #3f497e
            color_hover: #4f5ba0
            color_down: #2d3560
            border_size: 0.0
            border_radius: 4.0
        }
    }

    let SDLabel = Label {
        width: Fit
        height: Fit
        margin: Inset { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 }
        text: "Label"

        draw_text +: {
            font_scale: 1.0
            color: #ffffff
            text_style +: {
                font_size: 13.0
                line_spacing: 1.2
            }
        }
    }

    mod.widgets.AlertField = RoundedView {
        width: Fit
        height: Fit
        visible: false
        padding: Inset { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 }
        margin: Inset { top: 5.0, right: 0.0, bottom: 0.0, left: 0.0 }
        draw_bg +: {
            color: #ff444422
            border_size: 1.0
            border_color: #ff4444
            border_radius: 4.0
        }
        alert_text := SDLabel {
            margin: Inset { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
            draw_text +: { color: #ff4444, text_style +: { font_size: 11.0 } }
        }
    }

    mod.widgets.Post = View {
        width: Fill
        height: Fit
        padding: Inset { top: 5.0, right: 5.0, bottom: 5.0, left: 5.0 }

        body := RoundedView {
            width: Fill
            height: Fit
            flow: Down
            spacing: 0.0

            draw_bg +: {
                color: #2a2a3a
                border_radius: 8.0
            }

            username := View {
                width: Fill
                height: Fit
                padding: Inset { top: 8.0, right: 12.0, bottom: 4.0, left: 12.0 }
                text := SDLabel {
                    draw_text +: {
                        color: #8888cc,
                        text_style +: { font_size: 11.0 }
                    }
                }
            }
            content := View {
                width: Fill
                height: Fit
                padding: Inset { top: 4.0, right: 12.0, bottom: 12.0, left: 12.0 }
                text := Markdown {
                    width: Fill
                    height: Fit
                    body: ""
                    draw_text +: {
                        color: #ffffff,
                        text_style +: { font_size: 13.0 }
                    }
                }
            }
        }
    }

    mod.widgets.ChatItem = View {
        width: Fill
        height: Fit
        padding: Inset { top: 2.0, right: 10.0, bottom: 2.0, left: 10.0 }

        body := RoundedView {
            width: Fill
            height: Fit
            flow: Down
            padding: Inset { top: 10.0, right: 12.0, bottom: 10.0, left: 12.0 }
            spacing: 4.0

            draw_bg +: {
                color: #00000000
                color_hover: #ffffff11
                border_radius: 4.0
            }

            target_usr := SDLabel {
                draw_text +: {
                    color: #ffffff,
                    text_style +: { font_size: 13.0 }
                }
            }
            last_msg := SDLabel {
                draw_text +: {
                    color: #999999,
                    text_style +: { font_size: 11.0 }
                }
            }
        }
    }

    mod.widgets.SDLabel = SDLabel
}
