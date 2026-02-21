use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::Arc;
use image::RgbaImage;
use crate::theme::ThemeColors;

pub mod state;
pub mod tabs;
pub mod menu;

use state::AppState;
use tabs::Tab;
use menu::*;

pub struct PdfReaderApp {
    state: Arc<AppState>,
    show_sidebar: bool,
    show_language_menu: bool,
    window: Option<WeakEntity<Self>>,
}

impl PdfReaderApp {
    pub fn new(state: Arc<AppState>, window: &mut Window, _cx: &mut Context<Self>) -> Self {
        window.activate_window();
        window.set_window_title("LightPDF");
        
        Self {
            state,
            show_sidebar: false,
            show_language_menu: false,
            window: None,
        }
    }

    fn fit_width(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                if let Some(ref pdf_doc) = tab.doc {
                    let current_page = tab.current_page;
                    if let Ok((width, _)) = pdf_doc.get_page_size(current_page) {
                        let target_width = 800.0;
                        let zoom = target_width / width;
                        self.state.update_active_tab(|tab| {
                            tab.zoom = zoom.clamp(0.5, 3.0);
                        });
                        self.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                }
            }
        }
    }

    fn fit_page(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                if let Some(ref pdf_doc) = tab.doc {
                    let current_page = tab.current_page;
                    if let Ok((width, height)) = pdf_doc.get_page_size(current_page) {
                        let target_width = 600.0;
                        let target_height = 800.0;
                        let zoom_width = target_width / width;
                        let zoom_height = target_height / height;
                        let zoom = zoom_width.min(zoom_height);
                        self.state.update_active_tab(|tab| {
                            tab.zoom = zoom.clamp(0.5, 3.0);
                        });
                        self.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                }
            }
        }
    }

    pub fn open_file_in_new_tab(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        let path_str = path.to_string_lossy().to_string();
        log::info!("Opening file in new tab: {}", path_str);

        match self.state.open_file_new_tab(path) {
            Ok(tab_id) => {
                log::info!("File opened in tab {}", tab_id);
                self.show_sidebar = true;
                self.render_current_tab_page(tab_id, cx);
                cx.notify();
            }
            Err(e) => {
                log::error!("Failed to open PDF: {}", e);
            }
        }
    }

    pub fn close_tab(&mut self, tab_id: usize, cx: &mut Context<Self>) {
        self.state.close_tab(tab_id);
        cx.notify();
    }

    pub fn switch_tab(&mut self, tab_id: usize, cx: &mut Context<Self>) {
        self.state.set_active_tab(tab_id);
        self.render_current_tab_page(tab_id, cx);
        cx.notify();
    }

    fn open_file_dialog(&mut self, cx: &mut Context<Self>) {
        let options = PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("选择 PDF 文件".into()),
        };

        let receiver = cx.prompt_for_paths(options);
        
        cx.spawn(async move |this: WeakEntity<Self>, mut cx| {
            match receiver.await {
                Ok(Ok(Some(paths))) => {
                    if let Some(path) = paths.into_iter().next() {
                        this.update(cx, |this: &mut Self, cx: &mut Context<Self>| {
                            this.open_file_in_new_tab(path, cx);
                        }).ok();
                    }
                }
                _ => {}
            }
        }).detach();
    }

    fn render_current_tab_page(&mut self, tab_id: usize, _cx: &mut Context<Self>) {
        if let Some(tab) = self.state.tabs.get_tab(tab_id) {
            if let Some(ref pdf_doc) = tab.doc {
                let current_page = tab.current_page;
                let zoom = tab.zoom;
                let rotation = tab.rotation;

                match pdf_doc.render_page(current_page, zoom) {
                    Ok((data, pixmap_width, pixmap_height)) => {
                        let mut scaled_width = pixmap_width;
                        let mut scaled_height = pixmap_height;
                        
                        let mut rgba_image = RgbaImage::from_raw(
                            scaled_width,
                            scaled_height,
                            data
                        );

                        if let Some(ref mut rgba) = rgba_image {
                            match rotation {
                                90 => {
                                    *rgba = image::imageops::rotate90(rgba);
                                    std::mem::swap(&mut scaled_width, &mut scaled_height);
                                },
                                180 => {
                                    *rgba = image::imageops::rotate180(rgba);
                                },
                                270 => {
                                    *rgba = image::imageops::rotate270(rgba);
                                    std::mem::swap(&mut scaled_width, &mut scaled_height);
                                },
                                _ => {},
                            }

                            let page_dimensions = Some((scaled_width, scaled_height));
                            let frame = image::Frame::new(rgba.clone());
                            let render_image = RenderImage::new([frame]);
                            let page_image = Some(Arc::new(render_image));
                            
                            self.state.tabs.update_tab(tab_id, |tab| {
                                tab.page_dimensions = page_dimensions;
                                tab.page_image = page_image;
                            });
                            
                            log::info!("Successfully rendered page for tab {}", tab_id);
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to render page: {}", e);
                    }
                }
            }
        }
    }

    fn next_page(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            let _ = self.state.next_page();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn prev_page(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            let _ = self.state.prev_page();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn zoom_in(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.zoom_in();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn zoom_out(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.zoom_out();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn reset_zoom(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.reset_zoom();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn rotate_clockwise(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.rotate_clockwise();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn rotate_counter_clockwise(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.rotate_counter_clockwise();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let current_theme = self.state.get_theme();
        let new_theme = match current_theme {
            crate::theme::Theme::Light => crate::theme::Theme::Dark,
            crate::theme::Theme::Dark => crate::theme::Theme::Light,
        };
        self.state.set_theme(new_theme);
        cx.notify();
    }

    fn toggle_language_menu(&mut self, _cx: &mut Context<Self>) {
        self.show_language_menu = !self.show_language_menu;
    }

    fn set_language(&mut self, lang: crate::i18n::Language, cx: &mut Context<Self>) {
        self.state.set_language(lang);
        self.show_language_menu = false;
        cx.notify();
    }
}

impl Render for PdfReaderApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.get_theme();
        let colors = ThemeColors::for_theme(theme);
        let tabs = self.state.get_all_tabs();
        let active_tab_id = self.state.get_active_tab_id();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.background)
            .child(self.render_combined_titlebar(&tabs, active_tab_id, colors, cx))
            .child(self.render_toolbar(active_tab_id.is_some(), colors, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .child(if self.show_sidebar && active_tab_id.is_some() {
                        self.render_sidebar(active_tab_id, colors, cx).into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .child(self.render_pdf_view(active_tab_id, colors, cx))
            )
            .child(self.render_status_bar(active_tab_id, colors, cx))
    }
}

impl PdfReaderApp {
    fn render_combined_titlebar(
        &self,
        tabs: &[Tab],
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut titlebar = div()
            .h(px(32.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .bg(colors.toolbar)
            .border_b_1()
            .border_color(colors.border)
            .px_2()
            .child(div().w(px(70.0)));

        for (_i, tab) in tabs.iter().enumerate() {
            let is_active = Some(tab.id) == active_tab_id;
            
            titlebar = titlebar.child(
                div()
                    .h(px(28.0))
                    .min_w(px(100.0))
                    .max_w(px(180.0))
                    .px_2()
                    .mx(px(1.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .cursor_pointer()
                    .rounded_sm()
                    .when(is_active, |this| {
                        this.bg(colors.background)
                    })
                    .when(!is_active, |this| {
                        this.bg(colors.background_secondary)
                            .hover(|hover| hover.bg(colors.background_tertiary))
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(11.0))
                            .text_color(if is_active { colors.text } else { colors.text_secondary })
                            .text_ellipsis()
                            .child(tab.file_name())
                    )
                    .when(tabs.len() > 1, |this| {
                        this.child(
                            div()
                                .p(px(2.0))
                                .text_size(px(10.0))
                                .text_color(colors.text_secondary)
                                .cursor_pointer()
                                .hover(|hover| hover.text_color(colors.text).bg(colors.background_tertiary).rounded_sm())
                                .child("×")
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let tab_id = tab.id;
                                    move |this, _event, _window, cx| {
                                        this.close_tab(tab_id, cx);
                                    }
                                }))
                        )
                    })
                    .on_mouse_down(MouseButton::Left, cx.listener({
                        let tab_id = tab.id;
                        move |this, _event, _window, cx| {
                            this.switch_tab(tab_id, cx);
                        }
                    }))
            );
        }

        titlebar
            .child(
                div()
                    .h(px(28.0))
                    .w(px(28.0))
                    .ml(px(2.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_pointer()
                    .hover(|hover| hover.bg(colors.background_secondary).rounded_sm())
                    .text_color(colors.text_secondary)
                    .child("+")
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.open_file_dialog(cx);
                    }))
            )
            .child(div().flex_1())
            .child(
                div()
                    .h(px(28.0))
                    .px_3()
                    .flex()
                    .items_center()
                    .text_size(px(11.0))
                    .text_color(colors.text_secondary)
                    .child("LightPDF")
            )
    }

    fn render_toolbar(&self, has_doc: bool, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = self.state.get_theme();
        let theme_emoji = match theme {
            crate::theme::Theme::Light => "☾",
            crate::theme::Theme::Dark => "☼",
        };

        let language = self.state.get_language();
        let lang_flag = match language {
            crate::i18n::Language::English => "🇺🇸",
            crate::i18n::Language::Chinese => "🇨🇳",
            crate::i18n::Language::Spanish => "🇪🇸",
        };

        let show_menu = self.show_language_menu;
        let sidebar_emoji = if self.show_sidebar { "📑" } else { "📖" };

        div()
            .h(px(32.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .gap_1()
            .bg(colors.toolbar)
            .border_b_1()
            .border_color(colors.border)
            .child(toolbar_btn("📂", colors, cx.listener(|this, _event, _window, cx| {
                this.open_file_dialog(cx);
            })))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled("◀", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.prev_page(cx);
            })))
            .child(toolbar_btn_enabled("▶", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.next_page(cx);
            })))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled("⏮", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                if let Some(tab_id) = this.state.get_active_tab_id() {
                    this.state.update_active_tab(|tab| {
                        tab.current_page = 0;
                    });
                    this.render_current_tab_page(tab_id, cx);
                    cx.notify();
                }
            })))
            .child(toolbar_btn_enabled("⏭", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                if let Some(tab_id) = this.state.get_active_tab_id() {
                    this.state.update_active_tab(|tab| {
                        tab.current_page = tab.page_count.saturating_sub(1);
                    });
                    this.render_current_tab_page(tab_id, cx);
                    cx.notify();
                }
            })))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled("−", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.zoom_out(cx);
            })))
            .child(toolbar_btn_enabled("+", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.zoom_in(cx);
            })))
            .child(toolbar_btn_enabled("1:1", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.reset_zoom(cx);
            })))
            .child(toolbar_btn_enabled("↔", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.fit_width(cx);
            })))
            .child(toolbar_btn_enabled("□", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.fit_page(cx);
            })))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled("↻", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.rotate_clockwise(cx);
            })))
            .child(toolbar_btn_enabled("↺", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.rotate_counter_clockwise(cx);
            })))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(sidebar_emoji, has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.show_sidebar = !this.show_sidebar;
                cx.notify();
            })))
            .child(div().flex_1())
            .child(div().relative()
                .child(toolbar_btn(lang_flag, colors, cx.listener(|this, _event, _window, cx| {
                    this.toggle_language_menu(cx);
                })))
                .when(show_menu, |this| {
                    this.child(
                        div()
                            .absolute()
                            .right_0()
                            .top(px(32.0))
                            .flex()
                            .flex_col()
                            .w(px(120.0))
                            .bg(colors.background)
                            .border_1()
                            .border_color(colors.border)
                            .rounded_sm()
                            .shadow_md()
                            .child(lang_menu_item("🇺🇸 English", language != crate::i18n::Language::English, colors, cx.listener(|this, _event, _window, cx| {
                                this.set_language(crate::i18n::Language::English, cx);
                            })))
                            .child(lang_menu_item("🇨🇳 中文", language != crate::i18n::Language::Chinese, colors, cx.listener(|this, _event, _window, cx| {
                                this.set_language(crate::i18n::Language::Chinese, cx);
                            })))
                            .child(lang_menu_item("🇪🇸 Español", language != crate::i18n::Language::Spanish, colors, cx.listener(|this, _event, _window, cx| {
                                this.set_language(crate::i18n::Language::Spanish, cx);
                            })))
                    )
                })
            )
            .child(toolbar_btn(theme_emoji, colors, cx.listener(|this, _event, _window, cx| {
                this.toggle_theme(cx);
            })))
    }

    fn render_sidebar(&self, active_tab_id: Option<usize>, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let outline = active_tab_id.and_then(|id| self.state.tabs.get_tab(id).and_then(|t| t.outline_items.clone()));

        div()
            .w(px(200.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(colors.background_secondary)
            .border_r_1()
            .border_color(colors.border)
            .child(
                div()
                    .h(px(24.0))
                    .w_full()
                    .flex()
                    .items_center()
                    .px_2()
                    .bg(colors.background)
                    .border_b_1()
                    .border_color(colors.border)
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(colors.text)
                            .child("目录")
                    )
            )
            .child(
                div()
                    .flex_1()
                    .p_1()
                    .child(match outline {
                        Some(items) if !items.is_empty() => {
                            self.render_outline_items(&items, colors, cx, 0).into_any_element()
                        },
                        _ => {
                            div()
                                .text_size(px(10.0))
                                .text_color(colors.text_secondary)
                                .child("暂无目录")
                                .into_any_element()
                        }
                    })
            )
    }

    fn render_outline_items(&self, items: &[crate::pdf::OutlineItem], colors: ThemeColors, cx: &mut Context<Self>, level: usize) -> impl IntoElement {
        let mut container = div().flex().flex_col();

        for item in items {
            let page_num = item.page;
            container = container
                .child(
                    div()
                        .px(px(level as f32 * 12.0 + 8.0))
                        .py(px(4.0))
                        .cursor_pointer()
                        .hover(|this| this.bg(colors.background_tertiary))
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(colors.text)
                                .child(format!("{}", item.title))
                        )
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let page = page_num;
                            move |this, _event, _window, cx| {
                                if let Some(tab_id) = this.state.get_active_tab_id() {
                                    let _ = this.state.navigate_to_page(page);
                                    this.render_current_tab_page(tab_id, cx);
                                    cx.notify();
                                }
                            }
                        }))
                );
            
            if !item.children.is_empty() {
                container = container.child(
                    self.render_outline_items(&item.children, colors, cx, level + 1)
                );
            }
        }

        container
    }

    fn render_pdf_view(&self, active_tab_id: Option<usize>, colors: ThemeColors, _cx: &mut Context<Self>) -> impl IntoElement {
        if active_tab_id.is_none() {
            return div()
                .flex_1()
                .h_full()
                .bg(colors.pdf_view)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .text_size(px(14.0))
                                .text_color(colors.text_secondary)
                                .child("打开 PDF 文件开始阅读")
                        )
                )
                .into_any_element();
        }

        if let Some(tab_id) = active_tab_id {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                if let Some(image) = &tab.page_image {
                    let (width, height) = tab.page_dimensions.unwrap_or((800, 600));
                    let render_image = image.clone();
                    
                    return div()
                        .flex_1()
                        .bg(colors.pdf_view)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            img(render_image.clone())
                                .w(px(width as f32))
                                .h(px(height as f32))
                        )
                        .into_any_element();
                }
            }
        }

        div()
            .flex_1()
            .h_full()
            .bg(colors.pdf_view)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(colors.text_secondary)
                    .child("正在加载...")
            )
            .into_any_element()
    }

    fn render_status_bar(&self, active_tab_id: Option<usize>, colors: ThemeColors, _cx: &mut Context<Self>) -> impl IntoElement {
        let (page_info, zoom_info, file_name) = if let Some(tab_id) = active_tab_id {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                let file_name = tab.file_name();
                (
                    format!("{} / {}", tab.current_page + 1, tab.page_count),
                    format!("{:.0}%", tab.zoom * 100.0),
                    file_name,
                )
            } else {
                (String::new(), String::new(), String::new())
            }
        } else {
            (String::new(), String::new(), String::new())
        };

        let has_doc = !page_info.is_empty();

        div()
            .h(px(20.0))
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .px_2()
            .gap_3()
            .bg(colors.status_bar)
            .border_t_1()
            .border_color(colors.border)
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(colors.text)
                    .child(if has_doc { file_name } else { "就绪".to_string() })
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(colors.text)
                    .child(if has_doc { page_info } else { String::new() })
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(colors.text)
                    .child(if has_doc { zoom_info } else { String::new() })
            )
    }
}

fn toolbar_btn<F>(label: &str, colors: ThemeColors, on_click: F) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    div()
        .px_2()
        .py(px(2.0))
        .bg(colors.background_tertiary)
        .rounded_sm()
        .cursor_pointer()
        .child(
            div()
                .text_size(px(12.0))
                .text_color(colors.text)
                .child(label.to_string())
        )
        .on_mouse_down(MouseButton::Left, on_click)
}

fn toolbar_btn_enabled<F>(label: &str, enabled: bool, colors: ThemeColors, on_click: F) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    let base = div()
        .px_2()
        .py(px(2.0))
        .rounded_sm()
        .child(
            div()
                .text_size(px(12.0))
                .child(label.to_string())
        );

    if enabled {
        base
            .bg(colors.background_tertiary)
            .text_color(colors.text)
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
    } else {
        base
            .bg(colors.background_secondary)
            .text_color(colors.text_secondary)
    }
}

fn lang_menu_item<F>(label: &str, enabled: bool, colors: ThemeColors, on_click: F) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    let base = div()
        .px_3()
        .py(px(6.0))
        .w_full()
        .cursor_pointer()
        .child(
            div()
                .text_size(px(12.0))
                .text_color(colors.text)
                .child(label.to_string())
        );

    if enabled {
        base
            .bg(colors.background)
            .hover(|this| this.bg(colors.background_tertiary))
            .on_mouse_down(MouseButton::Left, on_click)
    } else {
        base
            .bg(colors.background_tertiary)
    }
}
