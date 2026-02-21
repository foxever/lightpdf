use crate::theme::ThemeColors;
use gpui::prelude::FluentBuilder;
use gpui::*;
use image::RgbaImage;
use std::sync::Arc;

pub mod menu;
pub mod state;
pub mod tabs;

use state::AppState;
use tabs::Tab;

pub struct PdfReaderApp {
    pub state: Arc<AppState>,
    pub show_sidebar: bool,
    focus_handle: FocusHandle,
}

impl PdfReaderApp {
    pub fn new(state: Arc<AppState>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        window.activate_window();
        window.set_window_title("LingPDF");

        let focus_handle = cx.focus_handle();
        focus_handle.focus(window);

        Self {
            state,
            show_sidebar: false,
            focus_handle,
        }
    }

    pub fn fit_width(&mut self, cx: &mut Context<Self>) {
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

    pub fn fit_page(&mut self, cx: &mut Context<Self>) {
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

    pub fn open_file_dialog(&mut self, cx: &mut Context<Self>) {
        let options = PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("选择 PDF 文件".into()),
        };

        let receiver = cx.prompt_for_paths(options);

        cx.spawn(
            async move |this: WeakEntity<Self>, cx| match receiver.await {
                Ok(Ok(Some(paths))) => {
                    if let Some(path) = paths.into_iter().next() {
                        this.update(cx, |this: &mut Self, cx: &mut Context<Self>| {
                            this.open_file_in_new_tab(path, cx);
                        })
                        .ok();
                    }
                }
                _ => {}
            },
        )
        .detach();
    }

    pub fn render_current_tab_page(&mut self, tab_id: usize, _cx: &mut Context<Self>) {
        if let Some(tab) = self.state.tabs.get_tab(tab_id) {
            if let Some(ref pdf_doc) = tab.doc {
                let current_page = tab.current_page;
                let zoom = tab.zoom;
                let rotation = tab.rotation;

                match pdf_doc.render_page(current_page, zoom) {
                    Ok((data, pixmap_width, pixmap_height)) => {
                        let mut scaled_width = pixmap_width;
                        let mut scaled_height = pixmap_height;

                        let mut rgba_image = RgbaImage::from_raw(scaled_width, scaled_height, data);

                        if let Some(ref mut rgba) = rgba_image {
                            match rotation {
                                90 => {
                                    *rgba = image::imageops::rotate90(rgba);
                                    std::mem::swap(&mut scaled_width, &mut scaled_height);
                                }
                                180 => {
                                    *rgba = image::imageops::rotate180(rgba);
                                }
                                270 => {
                                    *rgba = image::imageops::rotate270(rgba);
                                    std::mem::swap(&mut scaled_width, &mut scaled_height);
                                }
                                _ => {}
                            }

                            let page_dimensions = Some((scaled_width, scaled_height));
                            let frame = image::Frame::new(rgba.clone());
                            let render_image = RenderImage::new([frame]);
                            let page_image = Some(Arc::new(render_image));

                            self.state.tabs.update_tab(tab_id, |tab| {
                                tab.page_dimensions = page_dimensions;
                                tab.page_image = page_image;
                            });
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to render page: {}", e);
                    }
                }
            }
        }
    }

    pub fn next_page(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            let _ = self.state.next_page();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn prev_page(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            let _ = self.state.prev_page();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn zoom_in(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.zoom_in();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn zoom_out(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.zoom_out();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn reset_zoom(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.reset_zoom();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn rotate_clockwise(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.rotate_clockwise();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn rotate_counter_clockwise(&mut self, cx: &mut Context<Self>) {
        if let Some(tab_id) = self.state.get_active_tab_id() {
            self.state.rotate_counter_clockwise();
            self.render_current_tab_page(tab_id, cx);
            cx.notify();
        }
    }

    pub fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let current_theme = self.state.get_theme();
        let new_theme = match current_theme {
            crate::theme::Theme::Light => crate::theme::Theme::Dark,
            crate::theme::Theme::Dark => crate::theme::Theme::Light,
        };
        self.state.set_theme(new_theme);
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
            .track_focus(&self.focus_handle)
            .child(self.render_combined_titlebar(&tabs, active_tab_id, colors, cx))
            .child(self.render_toolbar(active_tab_id.is_some(), colors, cx))
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .flex()
                    .flex_row()
                    .child(if self.show_sidebar && active_tab_id.is_some() {
                        self.render_sidebar(active_tab_id, colors, cx)
                            .into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .child(self.render_pdf_view(active_tab_id, colors, cx)),
            )
            .child(self.render_status_bar(active_tab_id, colors, cx))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                match event.keystroke.key.as_str() {
                    "left" => this.prev_page(cx),
                    "right" => this.next_page(cx),
                    _ => {}
                }
            }))
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
                    .when(is_active, |this| this.bg(colors.background))
                    .when(!is_active, |this| {
                        this.bg(colors.background_secondary)
                            .hover(|hover| hover.bg(colors.background_tertiary))
                    })
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(11.0))
                            .text_color(if is_active {
                                colors.text
                            } else {
                                colors.text_secondary
                            })
                            .text_ellipsis()
                            .child(tab.file_name()),
                    )
                    .when(tabs.len() > 1, |this| {
                        this.child(
                            div()
                                .p(px(2.0))
                                .text_size(px(10.0))
                                .text_color(colors.text_secondary)
                                .cursor_pointer()
                                .hover(|hover| {
                                    hover
                                        .text_color(colors.text)
                                        .bg(colors.background_tertiary)
                                        .rounded_sm()
                                })
                                .child("×")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let tab_id = tab.id;
                                        move |this, _event, _window, cx| {
                                            this.close_tab(tab_id, cx);
                                        }
                                    }),
                                ),
                        )
                    })
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener({
                            let tab_id = tab.id;
                            move |this, _event, _window, cx| {
                                this.switch_tab(tab_id, cx);
                            }
                        }),
                    ),
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
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, _window, cx| {
                            this.open_file_dialog(cx);
                        }),
                    ),
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
                    .child("LightPDF"),
            )
    }

    fn render_toolbar(
        &self,
        has_doc: bool,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = self.state.get_theme();
        let (theme_emoji, theme_color) = match theme {
            crate::theme::Theme::Light => ("🌙", colors.moon_color),
            crate::theme::Theme::Dark => ("☀️", colors.sun_color),
        };

        let language = self.state.get_language();
        let lang_flag = match language {
            crate::i18n::Language::English => "🇺🇸",
            crate::i18n::Language::Chinese => "🇨🇳",
            crate::i18n::Language::Spanish => "🇪🇸",
        };

        let sidebar_emoji = if self.show_sidebar { "📑" } else { "📖" };

        let scroll_mode = self.state.get_scroll_mode();
        let scroll_emoji = match scroll_mode {
            crate::app::state::ScrollMode::Page => "📄",
            crate::app::state::ScrollMode::Smooth => "📜",
        };

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
            .child(toolbar_btn(
                "📂",
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.open_file_dialog(cx);
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                "◀",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.prev_page(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "▶",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.next_page(cx);
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                "⏮",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    if let Some(tab_id) = this.state.get_active_tab_id() {
                        this.state.update_active_tab(|tab| {
                            tab.current_page = 0;
                        });
                        this.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                }),
            ))
            .child(toolbar_btn_enabled(
                "⏭",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    if let Some(tab_id) = this.state.get_active_tab_id() {
                        this.state.update_active_tab(|tab| {
                            tab.current_page = tab.page_count.saturating_sub(1);
                        });
                        this.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                "−",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.zoom_out(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "+",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.zoom_in(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "1:1",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.reset_zoom(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "↔",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.fit_width(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "□",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.fit_page(cx);
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                "↻",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.rotate_clockwise(cx);
                }),
            ))
            .child(toolbar_btn_enabled(
                "↺",
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.rotate_counter_clockwise(cx);
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                sidebar_emoji,
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    this.show_sidebar = !this.show_sidebar;
                    cx.notify();
                }),
            ))
            .child(div().w(px(4.0)))
            .child(toolbar_btn_enabled(
                scroll_emoji,
                has_doc,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    let current_mode = this.state.get_scroll_mode();
                    let next_mode = match current_mode {
                        crate::app::state::ScrollMode::Page => {
                            crate::app::state::ScrollMode::Smooth
                        }
                        crate::app::state::ScrollMode::Smooth => {
                            crate::app::state::ScrollMode::Page
                        }
                    };
                    this.state.set_scroll_mode(next_mode);
                    cx.notify();
                }),
            ))
            .child(div().flex_1())
            .child(toolbar_btn(
                lang_flag,
                colors,
                cx.listener(|this, _event, _window, cx| {
                    let current_lang = this.state.get_language();
                    let next_lang = match current_lang {
                        crate::i18n::Language::English => crate::i18n::Language::Chinese,
                        crate::i18n::Language::Chinese => crate::i18n::Language::Spanish,
                        crate::i18n::Language::Spanish => crate::i18n::Language::English,
                    };
                    this.state.set_language(next_lang);
                    cx.notify();
                }),
            ))
            .child(toolbar_btn_with_color(
                theme_emoji,
                colors,
                theme_color,
                cx.listener(|this, _event, _window, cx| {
                    this.toggle_theme(cx);
                }),
            ))
    }

    fn render_sidebar(
        &self,
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let outline = active_tab_id.and_then(|id| {
            self.state
                .tabs
                .get_tab(id)
                .and_then(|t| t.outline_items.clone())
        });
        let i18n = self.state.get_i18n();
        let has_doc = active_tab_id.is_some();

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
                            .child(if has_doc {
                                i18n.t("sidebar_outline")
                            } else {
                                i18n.t("sidebar_recent_files")
                            }),
                    ),
            )
            .child(div().flex_1().p_1().child(if has_doc {
                match outline {
                    Some(items) if !items.is_empty() => self
                        .render_outline_items(&items, colors, cx, 0)
                        .into_any_element(),
                    _ => self.render_page_list(colors, cx).into_any_element(),
                }
            } else {
                self.render_recent_files(colors, cx).into_any_element()
            }))
    }

    fn render_recent_files(&self, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let recent_files = self.state.get_recent_files();
        let i18n = self.state.get_i18n();

        if recent_files.is_empty() {
            return div()
                .text_size(px(10.0))
                .text_color(colors.text_secondary)
                .child(i18n.t("no_recent_files"))
                .into_any_element();
        }

        let mut container = div().flex().flex_col();

        for file_path in recent_files {
            let file_name = std::path::Path::new(&file_path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&file_path)
                .to_string();
            let path_clone = file_path.clone();

            container = container.child(
                div()
                    .px_2()
                    .py(px(4.0))
                    .cursor_pointer()
                    .hover(|this| this.bg(colors.background_tertiary))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(colors.text)
                            .child(file_name),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, _window, cx| {
                            let path = std::path::PathBuf::from(&path_clone);
                            if path.exists() {
                                this.open_file_in_new_tab(path, cx);
                            } else {
                                this.state.remove_from_recent(&path_clone);
                                cx.notify();
                            }
                        }),
                    ),
            );
        }

        container.into_any_element()
    }

    fn render_outline_items(
        &self,
        items: &[crate::pdf::OutlineItem],
        colors: ThemeColors,
        cx: &mut Context<Self>,
        level: usize,
    ) -> impl IntoElement {
        let mut container = div().flex().flex_col();

        for item in items {
            let page_num = item.page;
            container = container.child(
                div()
                    .px(px(level as f32 * 12.0 + 8.0))
                    .py(px(4.0))
                    .cursor_pointer()
                    .hover(|this| this.bg(colors.background_tertiary))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(colors.text)
                            .child(format!("{}", item.title)),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener({
                            let page = page_num;
                            move |this, _event, _window, cx| {
                                if let Some(tab_id) = this.state.get_active_tab_id() {
                                    let _ = this.state.navigate_to_page(page);
                                    this.render_current_tab_page(tab_id, cx);
                                    cx.notify();
                                }
                            }
                        }),
                    ),
            );

            if !item.children.is_empty() {
                container = container.child(self.render_outline_items(
                    &item.children,
                    colors,
                    cx,
                    level + 1,
                ));
            }
        }

        container
    }

    fn render_page_list(&self, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let i18n = self.state.get_i18n();
        let page_count = self
            .state
            .get_active_tab_id()
            .and_then(|id| self.state.tabs.get_tab(id))
            .map(|t| t.page_count)
            .unwrap_or(0);

        if page_count == 0 {
            return div()
                .text_size(px(10.0))
                .text_color(colors.text_secondary)
                .child(i18n.t("pdf_no_outline"))
                .into_any_element();
        }

        let mut container = div().flex().flex_col();

        for page_num in 0..page_count {
            let page_num_clone = page_num;
            container = container.child(
                div()
                    .px_2()
                    .py(px(4.0))
                    .cursor_pointer()
                    .hover(|this| this.bg(colors.background_tertiary))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(colors.text)
                            .child(format!("{} {}", i18n.t("page_label"), page_num_clone + 1)),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _event, _window, cx| {
                            if let Some(tab_id) = this.state.get_active_tab_id() {
                                let _ = this.state.navigate_to_page(page_num_clone);
                                this.render_current_tab_page(tab_id, cx);
                                cx.notify();
                            }
                        }),
                    ),
            );
        }

        container.into_any_element()
    }

    fn render_pdf_view(
        &self,
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let i18n = self.state.get_i18n();
        let scroll_mode = self.state.get_scroll_mode();

        if active_tab_id.is_none() {
            return div()
                .flex_1()
                .h_full()
                .bg(colors.pdf_view)
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div().flex().flex_col().items_center().gap_3().child(
                        div()
                            .text_size(px(14.0))
                            .text_color(colors.text_secondary)
                            .child(i18n.t("welcome_message")),
                    ),
                )
                .into_any_element();
        }

        if let Some(tab_id) = active_tab_id {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                if let Some(image) = &tab.page_image {
                    let (width, height) = tab.page_dimensions.unwrap_or((800, 600));
                    let render_image = image.clone();

                    match scroll_mode {
                        crate::app::state::ScrollMode::Page => {
                            return div()
                                .flex_1()
                                .overflow_hidden()
                                .bg(colors.pdf_view)
                                .flex()
                                .flex_row()
                                .on_scroll_wheel(cx.listener(
                                    |this, event: &ScrollWheelEvent, _window, cx| match event.delta
                                    {
                                        ScrollDelta::Pixels(delta) => {
                                            if delta.y > px(10.0) {
                                                this.next_page(cx);
                                            } else if delta.y < px(-10.0) {
                                                this.prev_page(cx);
                                            }
                                        }
                                        ScrollDelta::Lines(delta) => {
                                            if delta.y > 0.5 {
                                                this.next_page(cx);
                                            } else if delta.y < -0.5 {
                                                this.prev_page(cx);
                                            }
                                        }
                                    },
                                ))
                                .children([
                                    div().flex_1().h_full().cursor_pointer().on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.prev_page(cx);
                                        }),
                                    ),
                                    div()
                                        .flex_1()
                                        .h_full()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            img(render_image.clone())
                                                .block()
                                                .max_w(px(width as f32))
                                                .max_h(px(height as f32)),
                                        ),
                                    div().flex_1().h_full().cursor_pointer().on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _event, _window, cx| {
                                            this.next_page(cx);
                                        }),
                                    ),
                                ])
                                .into_any_element();
                        }
                        crate::app::state::ScrollMode::Smooth => {
                            return div()
                                .flex_1()
                                .overflow_hidden()
                                .bg(colors.pdf_view)
                                .child(
                                    div()
                                        .flex_1()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .p_4()
                                        .gap_4()
                                        .child(
                                            img(render_image.clone())
                                                .block()
                                                .max_w(px(width as f32)),
                                        ),
                                )
                                .into_any_element();
                        }
                    }
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
                    .child("正在加载..."),
            )
            .into_any_element()
    }

    fn render_status_bar(
        &self,
        active_tab_id: Option<usize>,
        colors: ThemeColors,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let i18n = self.state.get_i18n();
        let (current_page, page_count, zoom_info, file_name) = if let Some(tab_id) = active_tab_id {
            if let Some(tab) = self.state.tabs.get_tab(tab_id) {
                let file_name = tab.file_name();
                (
                    tab.current_page + 1,
                    tab.page_count,
                    format!("{:.0}%", tab.zoom * 100.0),
                    file_name,
                )
            } else {
                (0, 0, String::new(), String::new())
            }
        } else {
            (0, 0, String::new(), String::new())
        };

        let has_doc = page_count > 0;
        let current_page_clone = current_page;
        let page_count_clone = page_count;

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
                    .child(if has_doc {
                        file_name
                    } else {
                        i18n.t("status_ready")
                    }),
            )
            .child(div().flex_1())
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(colors.text)
                    .child(if has_doc {
                        format!("{} / {}", current_page_clone, page_count_clone)
                    } else {
                        String::new()
                    }),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(colors.text)
                    .child(if has_doc { zoom_info } else { String::new() }),
            )
    }
}

fn toolbar_btn<F>(label: &str, colors: ThemeColors, on_click: F) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    toolbar_btn_with_color(label, colors, colors.text, on_click)
}

fn toolbar_btn_with_color<F>(
    label: &str,
    colors: ThemeColors,
    text_color: gpui::Rgba,
    on_click: F,
) -> impl IntoElement
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
                .text_color(text_color)
                .child(label.to_string()),
        )
        .on_mouse_down(MouseButton::Left, on_click)
}

fn toolbar_btn_enabled<F>(
    label: &str,
    enabled: bool,
    colors: ThemeColors,
    on_click: F,
) -> impl IntoElement
where
    F: Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
{
    let base = div()
        .px_2()
        .py(px(2.0))
        .rounded_sm()
        .child(div().text_size(px(12.0)).child(label.to_string()));

    if enabled {
        base.bg(colors.background_tertiary)
            .text_color(colors.text)
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, on_click)
    } else {
        base.bg(colors.background_secondary)
            .text_color(colors.text_secondary)
    }
}
