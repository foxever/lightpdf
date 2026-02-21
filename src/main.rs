#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::Arc;

mod app;
mod i18n;
mod pdf;
mod platform;
mod theme;
mod utils;

use app::menu::{
    FirstPage, FitPage, FitWidth, FullScreen, LastPage, NextPage, PrevPage, Quit, ResetZoom,
    RotateClockwise, RotateCounterClockwise, ToggleSidebar, ToggleTheme, ZoomIn, ZoomOut,
};
use app::PdfReaderApp;
use gpui::{prelude::*, App, Application, Menu, MenuItem, SystemMenuType, WindowHandle};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).cloned();

    Application::new().run(move |cx: &mut App| {
        cx.activate(true);
        cx.on_action(|_: &Quit, cx: &mut App| {
            cx.quit();
        });

        let app_state = Arc::new(app::state::AppState::new());
        let language = app_state.get_language();

        let menus = app::menu::create_menus(language);

        let mut full_menus = vec![Menu {
            name: "LightPDF".into(),
            items: vec![
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        }];
        full_menus.extend(menus);

        cx.set_menus(full_menus);

        let titlebar_options = gpui::TitlebarOptions {
            title: Some("LightPDF".into()),
            #[cfg(target_os = "macos")]
            appears_transparent: true,
            ..Default::default()
        };

        let file_path_clone = file_path.clone();

        let window_handle: WindowHandle<PdfReaderApp> = cx
            .open_window(
                gpui::WindowOptions {
                    titlebar: Some(titlebar_options),
                    window_bounds: Some(gpui::WindowBounds::Windowed(gpui::Bounds::centered(
                        None,
                        gpui::Size::new(gpui::px(1200.0), gpui::px(800.0)),
                        cx,
                    ))),
                    ..Default::default()
                },
                move |window, cx| {
                    cx.new(move |cx| {
                        let mut app = PdfReaderApp::new(app_state.clone(), window, cx);

                        if let Some(path_str) = &file_path_clone {
                            let path = std::path::PathBuf::from(path_str);
                            if path.exists() {
                                app.open_file_in_new_tab(path, cx);
                            } else {
                                log::error!("File not found: {}", path_str);
                            }
                        }

                        app
                    })
                },
            )
            .unwrap();

        cx.on_action(move |_: &PrevPage, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.prev_page(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &NextPage, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.next_page(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &FirstPage, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    if let Some(tab_id) = app.state.get_active_tab_id() {
                        app.state.update_active_tab(|tab| {
                            tab.current_page = 0;
                        });
                        app.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                })
                .ok();
        });

        cx.on_action(move |_: &LastPage, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    if let Some(tab_id) = app.state.get_active_tab_id() {
                        app.state.update_active_tab(|tab| {
                            tab.current_page = tab.page_count.saturating_sub(1);
                        });
                        app.render_current_tab_page(tab_id, cx);
                        cx.notify();
                    }
                })
                .ok();
        });

        cx.on_action(move |_: &ZoomIn, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.zoom_in(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &ZoomOut, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.zoom_out(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &ResetZoom, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.reset_zoom(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &FitWidth, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.fit_width(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &FitPage, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.fit_page(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &RotateClockwise, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.rotate_clockwise(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &RotateCounterClockwise, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.rotate_counter_clockwise(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &ToggleSidebar, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.show_sidebar = !app.show_sidebar;
                    cx.notify();
                })
                .ok();
        });

        cx.on_action(move |_: &ToggleTheme, cx: &mut App| {
            window_handle
                .update(cx, |app: &mut PdfReaderApp, _window, cx| {
                    app.toggle_theme(cx);
                })
                .ok();
        });

        cx.on_action(move |_: &FullScreen, cx: &mut App| {
            window_handle
                .update(cx, |_app, window, _cx| {
                    window.toggle_fullscreen();
                })
                .ok();
        });
    });
}
