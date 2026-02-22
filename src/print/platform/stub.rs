use crate::print::{PrintError, PrintSettings, Printer, PrinterInfo, Result};
use std::path::PathBuf;

pub struct StubPrinter;

impl Printer for StubPrinter {
    fn get_printers() -> Result<Vec<PrinterInfo>> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }

    fn print_pdf(
        _pdf_path: &PathBuf,
        _settings: &PrintSettings,
        _printer_name: Option<&str>,
    ) -> Result<()> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }

    fn show_print_dialog(_pdf_path: &PathBuf) -> Result<()> {
        Err(PrintError::PlatformError(
            "Printing not supported on this platform".to_string(),
        ))
    }

    fn show_print_preview(_pdf_path: &PathBuf) -> Result<()> {
        Err(PrintError::PlatformError(
            "Print preview not supported on this platform".to_string(),
        ))
    }
}

pub use StubPrinter as PlatformPrinter;
