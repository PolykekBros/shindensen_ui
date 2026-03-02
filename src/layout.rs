use makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets.*

    mod.widgets.MessageListPage = View{
        height: Fill
        width: Fill
        flow: Right
        padding: 0.0
        spacing: 0.0

        contacts := SolidView{
            width: 350.0
            height: Fill
            show_bg: true
            draw_bg +: {
                color: instance(#3f497e)
                border_radius: 5.0
            }
            padding: Inset{ top: 0.0, right: 10.0, bottom: 0.0, left: 0.0 }
            margin: Inset{ top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }

            flow: Down
            spacing: 10.0
            scroll_bars := ScrollBars{show_scroll_x: false, show_scroll_y: true}
        }

        dialog := SolidView {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg +: {
                color: #26242b
                border_radius: 5.0
            }
            flow: Down
            spacing: 10.0
            padding: Inset{ top: 0.0, right: 30.0, bottom: 0.0, left: 0.0 }
            margin: Inset{ top: 0.0, right: 0.0, bottom: 0.0, left: 0.0 }
            scroll_bars := ScrollBars{show_scroll_x: false, show_scroll_y: true}
        }
    }
}
