use gpui::*;
use std::sync::Arc;
use crate::app::state::AppState;

pub fn render(state: &Arc<AppState>, _cx: &mut WindowContext) -> impl IntoElement {
    let (page_info, zoom_info) = if let Some(doc) = state.current_doc.lock().unwrap().as_ref() {
        (
            format!("{} / {}", doc.current_page + 1, doc.page_count),
            format!("{:.0}%", doc.zoom * 100.0)
        )
    } else {
        ("-".to_string(), "100%".to_string())
    };
    
    div()
        .h(px(28.0))
        .w_full()
        .flex()
        .flex_row()
        .items_center()
        .px_4()
        .bg(rgb(0x007acc))
        .child(
            div()
                .flex()
                .flex_row()
                .gap_4()
                .child(format!("页码: {}", page_info))
                .child(format!("缩放: {}", zoom_info))
        )
        .child(div().flex_1())
        .child("LightPDF v0.1.0")
}
