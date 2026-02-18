use crate::makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    pub MessageListPage = <View> {
        height: Fill
        width: Fill
        flow: Right
        padding: 0
        spacing: 0.

        contacts = <RoundedView> {
            width: 350.
            height: Fill
            show_bg: true
            draw_bg: {
                color: (THEME_COLOR_INSET)
                border_radius: (THEME_CORNER_RADIUS)
            }
            padding: <THEME_MSPACE_3> { top: 0., right: (THEME_SPACE_2) }
            margin: <THEME_MSPACE_V_2> { }

            flow: Down,
            spacing: (THEME_SPACE_2)
            scroll_bars: <ScrollBars> {show_scroll_x: false, show_scroll_y: true}
        }

        dialog = <View> {
            width: Fill
            height: Fill
            flow: Down
            spacing: (THEME_SPACE_2)
            padding: <THEME_MSPACE_3> { right: (THEME_SPACE_2 * 3) }
            margin: <THEME_MSPACE_V_2> {}
            scroll_bars: <ScrollBars> {show_scroll_x: false, show_scroll_y: true}
        }
    }
}
