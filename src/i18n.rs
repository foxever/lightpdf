use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    English,
    Chinese,
    Spanish,
}

impl Default for Language {
    fn default() -> Self {
        #[cfg(target_os = "macos")]
        {
            if let Ok(lang) = std::env::var("LANG") {
                if lang.starts_with("zh") {
                    return Language::Chinese;
                } else if lang.starts_with("es") {
                    return Language::Spanish;
                }
            }
        }
        #[cfg(not(target_os = "macos"))]
        {
            if let Ok(lang) = std::env::var("LANG") {
                if lang.starts_with("zh") {
                    return Language::Chinese;
                } else if lang.starts_with("es") {
                    return Language::Spanish;
                }
            }
        }
        Language::English
    }
}

pub struct I18n {
    language: Language,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        Self { language }
    }

    pub fn default() -> Self {
        Self::new(Language::default())
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn t(&self, key: &str) -> String {
        match self.language {
            Language::English => self.t_en(key),
            Language::Chinese => self.t_zh(key),
            Language::Spanish => self.t_es(key),
        }
    }

    fn t_en(&self, key: &str) -> String {
        match key {
            "app_name" => "LightPDF".to_string(),
            "menu_file" => "File".to_string(),
            "menu_open" => "Open...".to_string(),
            "menu_quit" => "Quit".to_string(),
            "menu_view" => "View".to_string(),
            "menu_zoom_in" => "Zoom In".to_string(),
            "menu_zoom_out" => "Zoom Out".to_string(),
            "menu_reset_zoom" => "Actual Size".to_string(),
            "menu_next_page" => "Next Page".to_string(),
            "menu_prev_page" => "Previous Page".to_string(),
            "menu_theme" => "Theme".to_string(),
            "menu_theme_light" => "Light".to_string(),
            "menu_theme_dark" => "Dark".to_string(),
            "menu_language" => "Language".to_string(),
            "menu_language_en" => "English".to_string(),
            "menu_language_zh" => "中文".to_string(),
            "menu_language_es" => "Español".to_string(),
            "toolbar_open" => "Open PDF".to_string(),
            "toolbar_prev" => "Previous".to_string(),
            "toolbar_next" => "Next".to_string(),
            "toolbar_zoom_out" => "Zoom Out".to_string(),
            "toolbar_zoom_in" => "Zoom In".to_string(),
            "status_ready" => "Ready".to_string(),
            "status_page" => "Page".to_string(),
            "status_zoom" => "Zoom".to_string(),
            "pdf_drag_hint" => "Drag and drop a PDF file here".to_string(),
            "pdf_or_shortcut" => "or press Ctrl+O / Cmd+O to open a file".to_string(),
            "pdf_loading" => "Loading...".to_string(),
            "pdf_no_outline" => "No outline".to_string(),
            _ => key.to_string(),
        }
    }

    fn t_zh(&self, key: &str) -> String {
        match key {
            "app_name" => "LightPDF".to_string(),
            "menu_file" => "文件".to_string(),
            "menu_open" => "打开...".to_string(),
            "menu_quit" => "退出".to_string(),
            "menu_view" => "视图".to_string(),
            "menu_zoom_in" => "放大".to_string(),
            "menu_zoom_out" => "缩小".to_string(),
            "menu_reset_zoom" => "实际大小".to_string(),
            "menu_next_page" => "下一页".to_string(),
            "menu_prev_page" => "上一页".to_string(),
            "menu_theme" => "主题".to_string(),
            "menu_theme_light" => "浅色".to_string(),
            "menu_theme_dark" => "深色".to_string(),
            "menu_language" => "语言".to_string(),
            "menu_language_en" => "English".to_string(),
            "menu_language_zh" => "中文".to_string(),
            "menu_language_es" => "Español".to_string(),
            "toolbar_open" => "打开 PDF".to_string(),
            "toolbar_prev" => "上一页".to_string(),
            "toolbar_next" => "下一页".to_string(),
            "toolbar_zoom_out" => "缩小".to_string(),
            "toolbar_zoom_in" => "放大".to_string(),
            "status_ready" => "就绪".to_string(),
            "status_page" => "页码".to_string(),
            "status_zoom" => "缩放".to_string(),
            "pdf_drag_hint" => "拖放 PDF 文件到此处".to_string(),
            "pdf_or_shortcut" => "或按 Ctrl+O / Cmd+O 打开文件".to_string(),
            "pdf_loading" => "正在加载...".to_string(),
            "pdf_no_outline" => "暂无目录".to_string(),
            _ => key.to_string(),
        }
    }

    fn t_es(&self, key: &str) -> String {
        match key {
            "app_name" => "LightPDF".to_string(),
            "menu_file" => "Archivo".to_string(),
            "menu_open" => "Abrir...".to_string(),
            "menu_quit" => "Salir".to_string(),
            "menu_view" => "Ver".to_string(),
            "menu_zoom_in" => "Acercar".to_string(),
            "menu_zoom_out" => "Alejar".to_string(),
            "menu_reset_zoom" => "Tamaño real".to_string(),
            "menu_next_page" => "Página siguiente".to_string(),
            "menu_prev_page" => "Página anterior".to_string(),
            "menu_theme" => "Tema".to_string(),
            "menu_theme_light" => "Claro".to_string(),
            "menu_theme_dark" => "Oscuro".to_string(),
            "menu_language" => "Idioma".to_string(),
            "menu_language_en" => "English".to_string(),
            "menu_language_zh" => "中文".to_string(),
            "menu_language_es" => "Español".to_string(),
            "toolbar_open" => "Abrir PDF".to_string(),
            "toolbar_prev" => "Anterior".to_string(),
            "toolbar_next" => "Siguiente".to_string(),
            "toolbar_zoom_out" => "Alejar".to_string(),
            "toolbar_zoom_in" => "Acercar".to_string(),
            "status_ready" => "Listo".to_string(),
            "status_page" => "Página".to_string(),
            "status_zoom" => "Zoom".to_string(),
            "pdf_drag_hint" => "Arrastra y suelta un archivo PDF aquí".to_string(),
            "pdf_or_shortcut" => "o presiona Ctrl+O / Cmd+O para abrir un archivo".to_string(),
            "pdf_loading" => "Cargando...".to_string(),
            "pdf_no_outline" => "Sin esquema".to_string(),
            _ => key.to_string(),
        }
    }
}
