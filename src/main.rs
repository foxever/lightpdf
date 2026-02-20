use std::sync::Arc;

mod app;
mod pdf;
mod platform;
mod utils;
mod i18n;
mod theme;

use app::PdfReaderApp;
use gpui::{
    App, Application, Menu, MenuItem, SystemMenuType, actions,
    prelude::*,
};

actions!(lightpdf, [Quit, ToggleTheme]);

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).cloned();

    Application::new().run(move |cx: &mut App| {
        cx.activate(true);
        cx.on_action(quit);

        cx.set_menus(vec![
            Menu {
                name: "LightPDF".into(),
                items: vec![
                    MenuItem::os_submenu("Services", SystemMenuType::Services),
                    MenuItem::separator(),
                    MenuItem::action("Quit", Quit),
                ],
            },
        ]);

        let app_state = Arc::new(app::state::AppState::new());
        let file_path_clone = file_path.clone();

        cx.open_window(
            gpui::WindowOptions {
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("LightPDF".into()),
                    ..Default::default()
                }),
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
                            app.open_file(path, cx);
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

fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}
