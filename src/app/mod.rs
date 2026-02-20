use gpui::*;
use gpui::prelude::FluentBuilder;
use std::sync::Arc;
use image::RgbaImage;
use crate::theme::ThemeColors;

pub mod state;

use state::AppState;

pub struct PdfReaderApp {
    state: Arc<AppState>,
    show_sidebar: bool,
    current_page_image: Option<Arc<RenderImage>>,
    page_dimensions: Option<(u32, u32)>,
    show_language_menu: bool,
    outline_items: Option<Vec<crate::pdf::OutlineItem>>,
}

impl PdfReaderApp {
    pub fn new(state: Arc<AppState>, window: &mut Window, _cx: &mut Context<Self>) -> Self {
        window.activate_window();
        window.set_window_title("LightPDF");
        
        Self {
            state,
            show_sidebar: false,
            current_page_image: None,
            page_dimensions: None,
            show_language_menu: false,
            outline_items: None,
        }
    }

    pub fn open_file(&mut self, path: std::path::PathBuf, cx: &mut Context<Self>) {
        let path_str = path.to_string_lossy().to_string();
        log::info!("Opening file: {}", path_str);

        if let Err(e) = self.state.open_file(path) {
            log::error!("Failed to open PDF: {}", e);
        } else {
            log::info!("PDF opened successfully! Rendering current page...");
            
            // Load outline
            if let Some(pdf_guard) = self.state.get_pdf_doc() {
                if let Some(ref pdf_doc) = *pdf_guard {
                    match pdf_doc.get_outline() {
                        Ok(outline) => {
                            self.outline_items = Some(outline);
                        },
                        Err(e) => {
                            log::warn!("Failed to load outline: {}", e);
                            self.outline_items = None;
                        }
                    }
                }
            }
            
            self.render_current_page(cx);
            self.show_sidebar = true;
            cx.notify();
            log::info!("Rendering done!");
        }
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
                            this.open_file(path, cx);
                        }).ok();
                    }
                }
                Ok(Ok(None)) => {
                }
                Ok(Err(e)) => {
                    log::error!("Failed to select file: {}", e);
                }
                Err(e) => {
                    log::error!("Failed to open file dialog: {}", e);
                }
            }
        }).detach();
    }

    fn render_current_page(&mut self, _cx: &mut Context<Self>) {
        if let Some(pdf_guard) = self.state.get_pdf_doc() {
            if let Some(ref pdf_doc) = *pdf_guard {
                let current_page = self.state.current_doc.lock()
                    .unwrap()
                    .as_ref()
                    .map(|d| d.current_page)
                    .unwrap_or(0);

                let zoom = self.state.current_doc.lock()
                    .unwrap()
                    .as_ref()
                    .map(|d| d.zoom)
                    .unwrap_or(1.0);

                let rotation = self.state.current_doc.lock()
                    .unwrap()
                    .as_ref()
                    .map(|d| d.rotation)
                    .unwrap_or(0);

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

                            self.page_dimensions = Some((scaled_width, scaled_height));

                            log::info!("Rendering page: width={}, height={}, data len={}, expected len={}", 
                                scaled_width, scaled_height, rgba.len(), scaled_width * scaled_height * 4);

                            let frame = image::Frame::new(rgba.clone());
                            let render_image = RenderImage::new([frame]);
                            self.current_page_image = Some(Arc::new(render_image));
                            log::info!("Successfully created page image!");
                        } else {
                            log::error!("Failed to create RgbaImage");
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
        let _ = self.state.next_page();
        self.render_current_page(cx);
        cx.notify();
    }

    fn prev_page(&mut self, cx: &mut Context<Self>) {
        let _ = self.state.prev_page();
        self.render_current_page(cx);
        cx.notify();
    }

    fn zoom_in(&mut self, cx: &mut Context<Self>) {
        self.state.zoom_in();
        self.render_current_page(cx);
        cx.notify();
    }

    fn zoom_out(&mut self, cx: &mut Context<Self>) {
        self.state.zoom_out();
        self.render_current_page(cx);
        cx.notify();
    }

    fn reset_zoom(&mut self, cx: &mut Context<Self>) {
        self.state.reset_zoom();
        self.render_current_page(cx);
        cx.notify();
    }

    fn rotate_clockwise(&mut self, cx: &mut Context<Self>) {
        self.state.rotate_clockwise();
        self.render_current_page(cx);
        cx.notify();
    }

    fn rotate_counter_clockwise(&mut self, cx: &mut Context<Self>) {
        self.state.rotate_counter_clockwise();
        self.render_current_page(cx);
        cx.notify();
    }

    fn go_to_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let _ = self.state.navigate_to_page(page);
        self.render_current_page(cx);
        cx.notify();
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
        let has_doc = self.state.current_doc.lock().unwrap().is_some();
        let theme = self.state.get_theme();
        let colors = ThemeColors::for_theme(theme);

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(colors.background)
            .child(self.render_toolbar(colors, cx))
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .child(if self.show_sidebar && has_doc {
                        self.render_sidebar(colors, cx).into_any_element()
                    } else {
                        div().into_any_element()
                    })
                    .child(self.render_pdf_view(colors, cx))
            )
            .child(self.render_status_bar(colors, cx))
    }
}

impl PdfReaderApp {
    fn render_toolbar(&self, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let has_doc = self.state.current_doc.lock().unwrap().is_some();
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

        div()
            .h(px(28.0))
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
            .child(div().w(px(8.0)))
            .child(toolbar_btn_enabled("◀", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.prev_page(cx);
            })))
            .child(toolbar_btn_enabled("▶", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.next_page(cx);
            })))
            .child(div().w(px(8.0)))
            .child(toolbar_btn_enabled("−", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.zoom_out(cx);
            })))
            .child(toolbar_btn_enabled("+", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.zoom_in(cx);
            })))
            .child(div().w(px(8.0)))
            .child(toolbar_btn_enabled("↻", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.rotate_clockwise(cx);
            })))
            .child(toolbar_btn_enabled("↺", has_doc, colors, cx.listener(|this, _event, _window, cx| {
                this.rotate_counter_clockwise(cx);
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
                            .top(px(28.0))
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

    fn render_sidebar(&self, colors: ThemeColors, cx: &mut Context<Self>) -> impl IntoElement {
        let outline = &self.outline_items;

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
                            self.render_outline_items(items, colors, cx, 0).into_any_element()
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
                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                            this.go_to_page(page_num, cx);
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

    fn render_pdf_view(&self, colors: ThemeColors, _cx: &mut Context<Self>) -> impl IntoElement {
        let has_doc = self.state.current_doc.lock().unwrap().is_some();

        if !has_doc {
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
                                .child("拖放 PDF 文件到此处")
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(colors.text_secondary)
                                .child("或按 Ctrl+O / Cmd+O 打开文件")
                        )
                )
                .into_any_element();
        }

        if let Some(image) = &self.current_page_image {
            let (width, height) = self.page_dimensions.unwrap_or((800, 600));
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

    fn render_status_bar(&self, colors: ThemeColors, _cx: &mut Context<Self>) -> impl IntoElement {
        let (page_info, zoom_info, file_name) = if let Some(doc) = self.state.current_doc.lock().unwrap().as_ref() {
            let file_name = doc.path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default();
            (
                format!("{} / {}", doc.current_page + 1, doc.page_count),
                format!("{:.0}%", doc.zoom * 100.0),
                file_name
            )
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
