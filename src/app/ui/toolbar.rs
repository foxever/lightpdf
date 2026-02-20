use gpui::*;
use std::sync::Arc;
use crate::app::state::AppState;

pub fn render(state: &Arc<AppState>, _cx: &mut WindowContext) -> impl IntoElement {
    div()
        .h(px(40.0))
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .px_4()
        .bg(rgb(0x3c3c3c))
        .border_b_1()
        .border_color(rgb(0x1e1e1e))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(toolbar_button("打开", |cx| {
                    // TODO: 打开文件对话框
                    println!("打开文件");
                }))
                .child(toolbar_button("上一页", |cx| {
                    println!("上一页");
                }))
                .child(toolbar_button("下一页", |cx| {
                    println!("下一页");
                }))
        )
        .child(div().flex_1())
        .child(
            div()
                .flex()
                .flex_row()
                .gap_2()
                .child(toolbar_button("放大", |cx| {
                    println!("放大");
                }))
                .child(toolbar_button("缩小", |cx| {
                    println!("缩小");
                }))
                .child(toolbar_button("适应宽度", |cx| {
                    println!("适应宽度");
                }))
        )
}

fn toolbar_button(label: &str, on_click: impl Fn(&mut WindowContext) + 'static) -> impl IntoElement {
    div()
        .px_3()
        .py_1()
        .bg(rgb(0x4a4a4a))
        .rounded_md()
        .cursor_pointer()
        .child(label.to_string())
        .on_click(move |_, cx| on_click(cx))
}
