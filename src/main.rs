#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::Arc;

mod app;
mod pdf;
mod platform;
mod utils;
mod i18n;
mod theme;

use app::PdfReaderApp;
use app::menu::*;
use gpui::{
    App, Application, Menu, MenuItem, SystemMenuType,
    prelude::*,
};

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
        
        let mut full_menus = vec![
            Menu {
                name: "LightPDF".into(),
                items: vec![
                    MenuItem::os_submenu("Services", SystemMenuType::Services),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
            },
        ];
        full_menus.extend(menus);
        
        cx.set_menus(full_menus);
        
        let file_path_clone = file_path.clone();

        #[cfg(target_os = "macos")]
        let titlebar_options = {
            gpui::TitlebarOptions {
                title: Some("LightPDF".into()),
                appears_transparent: true,
                ..Default::default()
            }
        };

        #[cfg(not(target_os = "macos"))]
        let titlebar_options = {
            gpui::TitlebarOptions {
                title: Some("LightPDF".into()),
                ..Default::default()
            }
        };

        cx.open_window(
            gpui::WindowOptions {
                titlebar: Some(titlebar_options),
                window_bounds: Some(gpui::WindowBounds::Windowed(
                    gpui::Bounds::centered(
                        None,
                        gpui::Size::new(gpui::px(1200.0), gpui::px(800.0)),
                        cx,
                    )
                )),
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
    });
}
