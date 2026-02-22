pub mod platform;

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PrintError {
    #[error("Failed to initialize printer: {0}")]
    InitError(String),

    #[error("Failed to print: {0}")]
    PrintError(String),

    #[error("No printer available")]
    NoPrinter,

    #[error("Invalid page range: {0}")]
    InvalidPageRange(String),

    #[error("Platform error: {0}")]
    PlatformError(String),
}

pub type Result<T> = std::result::Result<T, PrintError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaperSize {
    A4,
    A3,
    A5,
    Letter,
    Legal,
    Tabloid,
}

impl PaperSize {
    pub fn dimensions_mm(&self) -> (f32, f32) {
        match self {
            PaperSize::A4 => (210.0, 297.0),
            PaperSize::A3 => (297.0, 420.0),
            PaperSize::A5 => (148.0, 210.0),
            PaperSize::Letter => (215.9, 279.4),
            PaperSize::Legal => (215.9, 355.6),
            PaperSize::Tabloid => (279.4, 431.8),
        }
    }

    pub fn dimensions_points(&self) -> (f32, f32) {
        let (w_mm, h_mm) = self.dimensions_mm();
        (w_mm * 2.83465, h_mm * 2.83465)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone)]
pub struct PageRange {
    pub start: usize,
    pub end: usize,
}

impl PageRange {
    pub fn all(page_count: usize) -> Self {
        Self {
            start: 0,
            end: page_count.saturating_sub(1),
        }
    }

    pub fn single(page: usize) -> Self {
        Self {
            start: page,
            end: page,
        }
    }

    pub fn range(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, page: usize) -> bool {
        page >= self.start && page <= self.end
    }

    pub fn page_count(&self) -> usize {
        self.end.saturating_sub(self.start) + 1
    }
}

#[derive(Debug, Clone)]
pub struct PrintSettings {
    pub paper_size: PaperSize,
    pub orientation: Orientation,
    pub page_range: Option<PageRange>,
    pub copies: u32,
    pub duplex: bool,
    pub color: bool,
    pub scale_to_fit: bool,
    pub margins: Margins,
}

impl Default for PrintSettings {
    fn default() -> Self {
        Self {
            paper_size: PaperSize::A4,
            orientation: Orientation::Portrait,
            page_range: None,
            copies: 1,
            duplex: false,
            color: true,
            scale_to_fit: true,
            margins: Margins::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Margins {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

impl Default for Margins {
    fn default() -> Self {
        Self {
            top: 10.0,
            bottom: 10.0,
            left: 10.0,
            right: 10.0,
        }
    }
}

impl Margins {
    pub fn new(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
        }
    }

    pub fn all(margin: f32) -> Self {
        Self {
            top: margin,
            bottom: margin,
            left: margin,
            right: margin,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
    pub supports_color: bool,
    pub supports_duplex: bool,
}

pub trait Printer {
    fn get_printers() -> Result<Vec<PrinterInfo>>;
    fn print_pdf(
        pdf_path: &PathBuf,
        settings: &PrintSettings,
        printer_name: Option<&str>,
    ) -> Result<()>;
    fn show_print_dialog(pdf_path: &PathBuf) -> Result<()>;
    fn show_print_preview(pdf_path: &PathBuf) -> Result<()>;
}

pub use platform::PlatformPrinter;

pub fn get_available_printers() -> Result<Vec<PrinterInfo>> {
    PlatformPrinter::get_printers()
}

pub fn print_document(
    pdf_path: &PathBuf,
    settings: &PrintSettings,
    printer_name: Option<&str>,
) -> Result<()> {
    PlatformPrinter::print_pdf(pdf_path, settings, printer_name)
}

pub fn show_print_dialog(pdf_path: &PathBuf) -> Result<()> {
    PlatformPrinter::show_print_dialog(pdf_path)
}

pub fn show_print_preview(pdf_path: &PathBuf) -> Result<()> {
    PlatformPrinter::show_print_preview(pdf_path)
}

pub fn get_default_settings() -> PrintSettings {
    PrintSettings::default()
}

pub fn validate_page_range(range_str: &str, max_pages: usize) -> Result<PageRange> {
    let range_str = range_str.trim();

    if range_str.eq_ignore_ascii_case("all") || range_str.is_empty() {
        return Ok(PageRange::all(max_pages));
    }

    if let Some(dash_pos) = range_str.find('-') {
        let start_str = &range_str[..dash_pos].trim();
        let end_str = &range_str[dash_pos + 1..].trim();

        let start = start_str.parse::<usize>().map_err(|_| {
            PrintError::InvalidPageRange(format!("Invalid start page: {}", start_str))
        })?;
        let end = end_str
            .parse::<usize>()
            .map_err(|_| PrintError::InvalidPageRange(format!("Invalid end page: {}", end_str)))?;

        if start == 0 || end == 0 || start > end || end > max_pages {
            return Err(PrintError::InvalidPageRange(format!(
                "Invalid range: {}-{} (max: {})",
                start, end, max_pages
            )));
        }

        Ok(PageRange::range(start - 1, end - 1))
    } else {
        let page = range_str.parse::<usize>().map_err(|_| {
            PrintError::InvalidPageRange(format!("Invalid page number: {}", range_str))
        })?;

        if page == 0 || page > max_pages {
            return Err(PrintError::InvalidPageRange(format!(
                "Page {} out of range (max: {})",
                page, max_pages
            )));
        }

        Ok(PageRange::single(page - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_range_all() {
        let range = PageRange::all(10);
        assert_eq!(range.start, 0);
        assert_eq!(range.end, 9);
        assert_eq!(range.page_count(), 10);
    }

    #[test]
    fn test_page_range_single() {
        let range = PageRange::single(5);
        assert_eq!(range.start, 5);
        assert_eq!(range.end, 5);
        assert_eq!(range.page_count(), 1);
    }

    #[test]
    fn test_validate_page_range_all() {
        let result = validate_page_range("all", 10).unwrap();
        assert_eq!(result.start, 0);
        assert_eq!(result.end, 9);
    }

    #[test]
    fn test_validate_page_range_single() {
        let result = validate_page_range("5", 10).unwrap();
        assert_eq!(result.start, 4);
        assert_eq!(result.end, 4);
    }

    #[test]
    fn test_validate_page_range_range() {
        let result = validate_page_range("3-7", 10).unwrap();
        assert_eq!(result.start, 2);
        assert_eq!(result.end, 6);
    }
}
