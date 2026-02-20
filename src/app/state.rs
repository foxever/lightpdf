use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use crate::pdf::{PdfDocument, loader::PdfLoader};
use crate::theme::Theme;
use crate::i18n::Language;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub recent_files: Vec<String>,
    pub default_zoom: f32,
    pub theme: Theme,
    pub language: Language,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            recent_files: Vec::new(),
            default_zoom: 1.0,
            theme: Theme::Dark,
            language: Language::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CurrentDocument {
    pub path: PathBuf,
    pub page_count: usize,
    pub current_page: usize,
    pub zoom: f32,
    pub rotation: i32,
}

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub current_doc: Mutex<Option<CurrentDocument>>,
    pdf_doc: Mutex<Option<PdfDocument>>,
}

impl AppState {
    pub fn new() -> Self {
        let config = Self::load_config();

        Self {
            config: Mutex::new(config),
            current_doc: Mutex::new(None),
            pdf_doc: Mutex::new(None),
        }
    }

    pub fn open_file(&self, path: PathBuf) -> anyhow::Result<()> {
        // 使用 MuPDF 加载 PDF 文件
        let pdf_doc = PdfLoader::open(&path)?;
        let page_count = pdf_doc.page_count();

        let doc = CurrentDocument {
            path: path.clone(),
            page_count,
            current_page: 0,
            zoom: 1.0,
            rotation: 0,
        };

        *self.current_doc.lock().unwrap() = Some(doc);
        *self.pdf_doc.lock().unwrap() = Some(pdf_doc);

        // 更新最近文件列表
        let mut config = self.config.lock().unwrap();
        let path_str = path.to_string_lossy().to_string();
        if !config.recent_files.contains(&path_str) {
            config.recent_files.insert(0, path_str);
            if config.recent_files.len() > 10 {
                config.recent_files.pop();
            }
        }

        self.save_config(&config);
        Ok(())
    }

    pub fn close_file(&self) {
        *self.current_doc.lock().unwrap() = None;
        *self.pdf_doc.lock().unwrap() = None;
    }

    pub fn get_pdf_doc(&self) -> Option<std::sync::MutexGuard<'_, Option<PdfDocument>>> {
        self.pdf_doc.lock().ok()
    }

    pub fn navigate_to_page(&self, page: usize) -> anyhow::Result<()> {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            if page < doc.page_count {
                doc.current_page = page;
            }
        }
        Ok(())
    }

    pub fn next_page(&self) -> anyhow::Result<()> {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            if doc.current_page < doc.page_count - 1 {
                doc.current_page += 1;
            }
        }
        Ok(())
    }

    pub fn prev_page(&self) -> anyhow::Result<()> {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            if doc.current_page > 0 {
                doc.current_page -= 1;
            }
        }
        Ok(())
    }

    pub fn set_zoom(&self, zoom: f32) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.zoom = zoom.clamp(0.5, 3.0);
        }
    }

    pub fn zoom_in(&self) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.zoom = (doc.zoom + 0.1).min(3.0);
        }
    }

    pub fn zoom_out(&self) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.zoom = (doc.zoom - 0.1).max(0.5);
        }
    }

    pub fn reset_zoom(&self) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.zoom = 1.0;
        }
    }

    pub fn rotate_clockwise(&self) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.rotation = (doc.rotation + 90) % 360;
        }
    }

    pub fn rotate_counter_clockwise(&self) {
        let mut doc = self.current_doc.lock().unwrap();
        if let Some(ref mut doc) = *doc {
            doc.rotation = (doc.rotation - 90 + 360) % 360;
        }
    }

    pub fn set_theme(&self, theme: Theme) {
        let mut config = self.config.lock().unwrap();
        config.theme = theme;
        self.save_config(&config);
    }

    pub fn get_theme(&self) -> Theme {
        self.config.lock().unwrap().theme
    }

    pub fn set_language(&self, language: Language) {
        let mut config = self.config.lock().unwrap();
        config.language = language;
        self.save_config(&config);
    }

    pub fn get_language(&self) -> Language {
        self.config.lock().unwrap().language
    }

    fn load_config() -> AppConfig {
        let config_path = crate::utils::path::get_config_path();
        if config_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        AppConfig::default()
    }

    fn save_config(&self, config: &AppConfig) {
        let config_path = crate::utils::path::get_config_path();
        if let Ok(content) = serde_json::to_string_pretty(config) {
            let _ = std::fs::write(config_path, content);
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
