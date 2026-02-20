use gpui::*;
use std::sync::Arc;
use crate::app::state::AppState;

pub fn render(state: &Arc<AppState>, _cx: &mut WindowContext) -> impl IntoElement {
    let has_doc = state.current_doc.lock().unwrap().is_some();
    
    div()
        .flex_1()
        .h_full()
        .bg(rgb(0x1e1e1e))
        .flex()
        .items_center()
        .justify_center()
        .child(
            if has_doc {
                div()
                    .child("PDF 内容区域")
                    .into_any_element()
            } else {
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0x888888))
                            .child("拖放 PDF 文件到此处打开")
                    )
                    .child(
                        div()
                            .text_color(rgb(0x666666))
                            .child("或使用 Ctrl+O (Cmd+O) 打开文件")
                    )
                    .into_any_element()
            }
        )
}
