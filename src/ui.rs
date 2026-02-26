use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.ui = {}
    mod.ui.header_font = { font_size: 16.0, line_spacing: 1.2 }
    mod.ui.body_font = { font_size: 13.0, line_spacing: 1.2 }

    mod.ui.InputField = TextInput {
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
            instance radius: 4.0
            instance border_width: 1.0
            instance border_color: #3a3a5a
        }

        draw_text +: {
            font_scale: 1.0
            text_style: { font_size: 13.0, line_spacing: 1.2 }
            color: #dcdcdc
        }

        draw_selection +: {
            color: #3d5afe80
        }

        draw_cursor +: {
            color: #ffffff
        }
    }

    mod.ui.Button = Button {
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
            text_style: {
                font_size: 11.0
                line_spacing: 1.2
            }
        }

        draw_bg +: {
            color: #3f497e
            instance border_width: 0.0
            instance radius: 4.0
            
            fn pixel(self) -> vec4 {
                let hover = self.hover;
                let pressed = self.pressed;
                let color = mix(
                    mix(#3f497e, #4f5ba0, hover),
                    #2d3560,
                    pressed
                );
                return fill(color);
            }
        }
    }

    mod.ui.TextLabel = Label {
        width: Fit
        height: Fit
        margin: Inset { top: 10.0, right: 10.0, bottom: 10.0, left: 10.0 }
        text: "Label"

        draw_text +: {
            font_scale: 1.0
            color: #ffffff
            text_style: {
                font_size: 13.0
                line_spacing: 1.2
            }
        }
    }

    mod.ui.AlertField = RoundedView {
        width: Fit
        height: Fit
        visible: false
        padding: Inset { top: 8.0, right: 12.0, bottom: 8.0, left: 12.0 }
        margin: Inset { top: 5.0, right: 0.0, bottom: 0.0, left: 0.0 }
        draw_bg +: {
            color: #ff444422
            instance border_width: 1.0
            instance border_color: #ff4444
            instance radius: 4.0
        }
        alert_text := mod.ui.TextLabel {
            margin: Inset { top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
            draw_text: { color: #ff4444, text_style: { font_size: 11.0 } }
        }
    }

    mod.ui.Post = View {
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
                instance radius: 8.0
            }

            username := View {
                width: Fill
                height: Fit
                padding: Inset { top: 8.0, right: 12.0, bottom: 4.0, left: 12.0 }
                text := Label { 
                    draw_text: { 
                        color: #8888cc, 
                        text_style: { font_size: 11.0 } 
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
                    draw_text: { 
                        color: #ffffff, 
                        text_style: { font_size: 13.0 } 
                    } 
                }
            }
        }
    }

    mod.ui.ChatItem = View {
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
                instance radius: 4.0
                
                fn pixel(self) -> vec4 {
                    return fill(mix(#00000000, #ffffff11, self.hover));
                }
            }

            target_usr := Label { 
                draw_text: { 
                    color: #ffffff, 
                    text_style: { font_size: 13.0 } 
                } 
            }
            last_msg := Label { 
                draw_text: { 
                    color: #999999, 
                    text_style: { font_size: 11.0 } 
                } 
            }
        }
    }
}
