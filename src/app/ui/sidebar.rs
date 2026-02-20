use gpui::*;
use std::sync::Arc;
use crate::app::state::AppState;

pub fn render(state: &Arc<AppState>, _cx: &mut WindowContext) -> impl IntoElement {
    div()
        .w(px(250.0))
        .h_full()
        .flex()
        .flex_col()
        .bg(rgb(0x252525))
        .border_r_1()
        .border_color(rgb(0x1e1e1e))
        .child(
            div()
                .h(px(35.0))
                .w_full()
                .flex()
                .items_center()
                .px_3()
                .bg(rgb(0x2d2d2d))
                .border_b_1()
                .border_color(rgb(0x1e1e1e))
                .child("目录")
        )
        .child(
            div()
                .flex_1()
                .p_2()
                .child("暂无目录信息")
        )
}
