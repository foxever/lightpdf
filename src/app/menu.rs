use gpui::*;
use crate::i18n::Language;

actions!(lightpdf, [
    OpenFile,
    CloseTab,
    SaveAs,
    Print,
    Quit,
    ZoomIn,
    ZoomOut,
    ResetZoom,
    FitWidth,
    FitPage,
    RotateClockwise,
    RotateCounterClockwise,
    FullScreen,
    ToggleSidebar,
    PrevPage,
    NextPage,
    FirstPage,
    LastPage,
    GoToPage,
    AddToFavorites,
    ToggleTheme,
    About
]);

pub fn create_menus(language: Language) -> Vec<Menu> {
    match language {
        Language::English => create_english_menus(),
        Language::Chinese => create_chinese_menus(),
        Language::Spanish => create_spanish_menus(),
    }
}

fn create_english_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("Open...", OpenFile),
                MenuItem::separator(),
                MenuItem::action("Close Tab", CloseTab),
                MenuItem::action("Save As...", SaveAs),
                MenuItem::separator(),
                MenuItem::action("Print...", Print),
                MenuItem::separator(),
                MenuItem::action("Quit", Quit),
            ],
        },
        Menu {
            name: "View".into(),
            items: vec![
                MenuItem::action("Zoom In", ZoomIn),
                MenuItem::action("Zoom Out", ZoomOut),
                MenuItem::action("Actual Size", ResetZoom),
                MenuItem::separator(),
                MenuItem::action("Fit Width", FitWidth),
                MenuItem::action("Fit Page", FitPage),
                MenuItem::separator(),
                MenuItem::action("Rotate Clockwise", RotateClockwise),
                MenuItem::action("Rotate Counter-Clockwise", RotateCounterClockwise),
                MenuItem::separator(),
                MenuItem::action("Full Screen", FullScreen),
                MenuItem::action("Toggle Sidebar", ToggleSidebar),
            ],
        },
        Menu {
            name: "Go".into(),
            items: vec![
                MenuItem::action("Previous Page", PrevPage),
                MenuItem::action("Next Page", NextPage),
                MenuItem::separator(),
                MenuItem::action("First Page", FirstPage),
                MenuItem::action("Last Page", LastPage),
                MenuItem::separator(),
                MenuItem::action("Go to Page...", GoToPage),
            ],
        },
        Menu {
            name: "Favorites".into(),
            items: vec![
                MenuItem::action("Add to Favorites", AddToFavorites),
            ],
        },
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action("About LightPDF", About),
            ],
        },
    ]
}

fn create_chinese_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "文件".into(),
            items: vec![
                MenuItem::action("打开...", OpenFile),
                MenuItem::separator(),
                MenuItem::action("关闭标签", CloseTab),
                MenuItem::action("另存为...", SaveAs),
                MenuItem::separator(),
                MenuItem::action("打印...", Print),
                MenuItem::separator(),
                MenuItem::action("退出", Quit),
            ],
        },
        Menu {
            name: "视图".into(),
            items: vec![
                MenuItem::action("放大", ZoomIn),
                MenuItem::action("缩小", ZoomOut),
                MenuItem::action("实际大小", ResetZoom),
                MenuItem::separator(),
                MenuItem::action("适应宽度", FitWidth),
                MenuItem::action("适应页面", FitPage),
                MenuItem::separator(),
                MenuItem::action("顺时针旋转", RotateClockwise),
                MenuItem::action("逆时针旋转", RotateCounterClockwise),
                MenuItem::separator(),
                MenuItem::action("全屏", FullScreen),
                MenuItem::action("切换侧边栏", ToggleSidebar),
            ],
        },
        Menu {
            name: "导航".into(),
            items: vec![
                MenuItem::action("上一页", PrevPage),
                MenuItem::action("下一页", NextPage),
                MenuItem::separator(),
                MenuItem::action("第一页", FirstPage),
                MenuItem::action("最后一页", LastPage),
                MenuItem::separator(),
                MenuItem::action("跳转到页...", GoToPage),
            ],
        },
        Menu {
            name: "收藏".into(),
            items: vec![
                MenuItem::action("添加到收藏", AddToFavorites),
            ],
        },
        Menu {
            name: "帮助".into(),
            items: vec![
                MenuItem::action("关于 LightPDF", About),
            ],
        },
    ]
}

fn create_spanish_menus() -> Vec<Menu> {
    vec![
        Menu {
            name: "Archivo".into(),
            items: vec![
                MenuItem::action("Abrir...", OpenFile),
                MenuItem::separator(),
                MenuItem::action("Cerrar pestaña", CloseTab),
                MenuItem::action("Guardar como...", SaveAs),
                MenuItem::separator(),
                MenuItem::action("Imprimir...", Print),
                MenuItem::separator(),
                MenuItem::action("Salir", Quit),
            ],
        },
        Menu {
            name: "Ver".into(),
            items: vec![
                MenuItem::action("Ampliar", ZoomIn),
                MenuItem::action("Reducir", ZoomOut),
                MenuItem::action("Tamaño real", ResetZoom),
                MenuItem::separator(),
                MenuItem::action("Ajustar a ancho", FitWidth),
                MenuItem::action("Ajustar a página", FitPage),
                MenuItem::separator(),
                MenuItem::action("Girar en sentido horario", RotateClockwise),
                MenuItem::action("Girar en sentido antihorario", RotateCounterClockwise),
                MenuItem::separator(),
                MenuItem::action("Pantalla completa", FullScreen),
                MenuItem::action("Alternar barra lateral", ToggleSidebar),
            ],
        },
        Menu {
            name: "Ir".into(),
            items: vec![
                MenuItem::action("Página anterior", PrevPage),
                MenuItem::action("Página siguiente", NextPage),
                MenuItem::separator(),
                MenuItem::action("Primera página", FirstPage),
                MenuItem::action("Última página", LastPage),
                MenuItem::separator(),
                MenuItem::action("Ir a página...", GoToPage),
            ],
        },
        Menu {
            name: "Favoritos".into(),
            items: vec![
                MenuItem::action("Agregar a favoritos", AddToFavorites),
            ],
        },
        Menu {
            name: "Ayuda".into(),
            items: vec![
                MenuItem::action("Acerca de LightPDF", About),
            ],
        },
    ]
}
